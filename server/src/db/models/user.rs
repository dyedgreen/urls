use crate::db::id::UserID;
use crate::db::models::{Invite, Login, Permission, Role};
use crate::schema::{logins, roles, users};
use crate::Context;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use lettre::address::Address;
use lettre::message::{Mailbox, Message};
use std::str::FromStr;
use validator::Validate;
use web_session::Session;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct User {
    id: UserID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    name: String,
    email: String,
}

#[derive(Debug, Clone, Validate, GraphQLInputObject)]
pub struct NewUserInput {
    #[validate(length(min = 1, max = 256, message = "A name is required"))]
    pub name: String,
    #[validate(email(message = "A valid email address is required"))]
    pub email: String,
}

#[derive(Debug, Clone, Validate, GraphQLInputObject)]
pub struct UpdateUserInput {
    #[validate(length(min = 1, max = 256, message = "Invalid name"))]
    name: Option<String>,
    #[validate(email(message = "Invalid email"))]
    email: Option<String>,
}

impl User {
    /// Unique identifier for this user. This is
    /// a random unique identifier and is safe
    /// to share with clients.
    pub fn id(&self) -> UserID {
        self.id
    }

    /// Name of this user.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the email address of this user.
    pub fn email(&self) -> Result<Address> {
        let address = Address::from_str(&self.email)?;
        Ok(address)
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }

    /// Return a list of all active permissions for this
    /// user.
    pub async fn permissions(&self, ctx: &Context) -> Result<Vec<Permission>> {
        let conn = ctx.conn().await?;
        let permissions = Role::belonging_to(self)
            .order_by(roles::dsl::created_at.asc())
            .select(roles::dsl::permission)
            .load(&*conn)?;
        Ok(permissions)
    }
}

impl User {
    /// Creates a new user in the database. Also see
    /// [`create_with_invite`](create_with_invite), which
    /// requires an unclaimed invite and is most likely
    /// what you want.
    pub async fn create(ctx: &Context, input: NewUserInput) -> Result<Self> {
        let input = NewUserInput {
            name: input.name.trim().into(),
            email: input.email.trim().into(),
        };
        input.validate()?;
        let NewUserInput { name, email } = input;

        let user = User {
            id: UserID::new(),
            name,
            email,

            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),
        };

        let conn = ctx.conn().await?;
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn)?;

        Ok(user)
    }

    /// Create a user by claiming the given invite.
    pub async fn create_with_invite(
        ctx: &Context,
        input: NewUserInput,
        mut invite: Invite,
    ) -> Result<Self> {
        let user = Self::create(ctx, input).await?;
        match invite.claim(ctx, &user).await {
            Ok(()) => Ok(user),
            Err(err) => {
                // TODO: Should this use a transaction?
                diesel::delete(&user).execute(&*ctx.conn().await?)?;
                Err(err.into())
            }
        }
    }

    /// Update this users details using data given in an update
    /// object. This is meant to be exposed from the graphql API.
    pub async fn update(&mut self, ctx: &Context, input: UpdateUserInput) -> Result<()> {
        let input = UpdateUserInput {
            name: input.name.map(|name| name.trim().into()),
            email: input.email.map(|email| email.trim().into()),
        };
        input.validate()?;
        let UpdateUserInput { name, email } = input;

        if let Some(name) = name {
            self.name = name;
            self.updated_at = ctx.now().naive_utc();
        }

        if let Some(email) = email {
            self.email = email;
            self.updated_at = ctx.now().naive_utc();
        }

        *self = self.save_changes(&*ctx.conn().await?)?;
        Ok(())
    }

    /// Retrieve a user by it's email address.
    pub async fn find_by_email(ctx: &Context, email: &str) -> Result<Self> {
        let conn = ctx.conn().await?;
        let user = users::table
            .filter(users::dsl::email.eq(email))
            .get_result(&*conn)?;
        Ok(user)
    }

    /// Creates a login and sends an email to the user, containing the
    /// login token.
    pub async fn request_login(&self, ctx: &Context) -> Result<()> {
        let login = Login::create(ctx, self.id()).await?;
        let email = Message::builder()
            .from("noreply@urls.fyi".parse().unwrap())
            .to(Mailbox::new(Some(self.name.clone()), self.email()?))
            .subject("Login request")
            .body(format!(
                "A login code was requested for your account ({email}).\n\n\
                Code: {token}\n\n\
                If you did not request the code, you may safely ignore this email.",
                email = self.email,
                token = login.token(),
            ))?;
        ctx.mailer().send(email).await?;
        Ok(())
    }

    /// Login this user by consuming a login token and returning a web
    /// session.
    pub async fn login(&self, ctx: &Context, token: &str) -> Result<Session<UserID>> {
        let mut login: Login = Login::belonging_to(self)
            .filter(logins::dsl::token.eq(token))
            .filter(logins::dsl::valid_until.gt(ctx.now().naive_utc()))
            .get_result(&*ctx.conn().await?)?;
        let session = login.claim(ctx, token).await?;
        Ok(session)
    }
}
