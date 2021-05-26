use crate::db::id::UserID;
use crate::db::models::Login;
use crate::schema::users;
use crate::Context;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use lettre::address::Address;
use lettre::message::{Mailbox, Message};
use std::str::FromStr;
use validator::Validate;

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
    #[validate(length(min = 1, max = 256))]
    name: String,
    #[validate(email)]
    email: String,
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
}

impl User {
    /// Creates a new user in the database.
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
}
