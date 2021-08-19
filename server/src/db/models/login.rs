use crate::db::id::{LoginID, UserID};
use crate::db::models::User;
use crate::schema::logins;
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use nanoid::nanoid;

const LOGIN_LIMIT_PER_HOUR: i64 = 3;
const LOGIN_VALID_MINUTES: i64 = 60;
const WEB_SESSION_MAX_UNUSED_DAYS: i64 = 30;
const EMAIL_TOKEN_ALPHABET: &[char] = &[
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User)]
pub struct Login {
    id: LoginID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    user_id: UserID,
    email_token: String,
    claim_until: NaiveDateTime,
    claimed: bool,
    session_token: Option<String>,
    last_used: NaiveDateTime,
}

impl Login {
    pub fn id(&self) -> LoginID {
        self.id
    }

    pub fn email_token(&self) -> &str {
        self.email_token.as_str()
    }

    pub fn claim_until(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.claim_until, Utc)
    }

    pub fn is_claimed(&self) -> bool {
        self.claimed
    }

    pub fn session_token(&self) -> Option<&str> {
        self.session_token.as_ref().map(|s| s.as_str())
    }

    pub fn last_used(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.last_used, Utc)
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }
}

impl Login {
    /// Creates a new login in the database. This will fail, if too
    /// many requests have been made within the last hour, this will fail.
    pub async fn create(ctx: &Context, user_id: UserID) -> Result<Self> {
        let conn = ctx.conn().await?;

        // Check there haven't been too many logins within the
        // last hour.
        let last_hour = ctx.now() - Duration::hours(1);
        let num_logins_last_hour: i64 = logins::table
            .filter(logins::dsl::user_id.eq(user_id))
            .filter(logins::dsl::created_at.gt(last_hour.naive_utc()))
            .count()
            .get_result(&*conn)?;
        if num_logins_last_hour >= LOGIN_LIMIT_PER_HOUR {
            return Err(anyhow!(
                "Exceeded login limit of {} per hour",
                LOGIN_LIMIT_PER_HOUR,
            ));
        }

        let login = Login {
            id: LoginID::new(),
            user_id,
            email_token: nanoid!(12, EMAIL_TOKEN_ALPHABET),
            claim_until: (ctx.now() + Duration::minutes(LOGIN_VALID_MINUTES)).naive_utc(),
            claimed: false,
            session_token: None,
            last_used: ctx.now().naive_utc(),

            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),
        };

        diesel::insert_into(logins::table)
            .values(&login)
            .execute(&*conn)?;

        Ok(login)
    }

    /// Claims the login token and returns a session token. The session can be
    /// used to authenticate to the graphql API.
    pub async fn claim(&mut self, ctx: &Context, email_token: &str) -> Result<String> {
        if self.is_claimed() {
            Err(anyhow!("The login was already claimed"))
        } else if self.claim_until() < ctx.now() {
            Err(anyhow!("The login is expired"))
        } else if self.email_token() != email_token {
            Err(anyhow!("Invalid login token"))
        } else {
            let session_token = nanoid!(64);
            self.claimed = true;
            self.session_token = Some(session_token.clone());
            self.last_used = ctx.now().naive_utc();
            self.updated_at = ctx.now().naive_utc();
            let conn = ctx.conn().await?;
            *self = self.save_changes(&*conn)?;
            Ok(session_token)
        }
    }

    /// Retrieves an active login from the database and uses it to obtain a
    /// user session. This function would typically be called to construct
    /// a request context. If the session token is invalid, this returns an
    /// error.
    pub async fn use_session(ctx: &mut Context, session_token: &str) -> Result<()> {
        let conn = ctx.conn().await?;
        let mut login: Self = logins::table
            .filter(logins::dsl::session_token.eq(session_token))
            .filter(logins::dsl::claimed.eq(true))
            .get_result(&*conn)?;
        if login.last_used() + Duration::days(WEB_SESSION_MAX_UNUSED_DAYS) < ctx.now() {
            Err(anyhow!("The login session is expired"))
        } else {
            login.last_used = ctx.now().naive_utc();
            login.updated_at = ctx.now().naive_utc();
            login.save_changes::<Login>(&*conn)?;
            drop(conn);
            ctx.set_logged_in_user(login.user_id);
            Ok(())
        }
    }
}
