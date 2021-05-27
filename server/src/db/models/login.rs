use crate::db::id::{LoginID, UserID};
use crate::db::models::User;
use crate::schema::logins;
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use nanoid::nanoid;
use web_session::Session;

const LOGIN_LIMIT_PER_HOUR: i64 = 3;
const LOGIN_VALID_MINUTES: i64 = 60;
const WEB_SESSION_VALID_DAYS: i64 = 7;
const TOKEN_ALPHABET: &[char] = &[
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
    token: String,
    valid_until: NaiveDateTime,
    claimed: bool,
}

impl Login {
    pub fn id(&self) -> LoginID {
        self.id
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn valid_until(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.valid_until, Utc)
    }

    pub fn is_claimed(&self) -> bool {
        self.claimed
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
            token: nanoid!(12, TOKEN_ALPHABET),
            valid_until: (ctx.now() + Duration::minutes(LOGIN_VALID_MINUTES)).naive_utc(),
            claimed: false,

            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),
        };

        diesel::insert_into(logins::table)
            .values(&login)
            .execute(&*conn)?;

        Ok(login)
    }

    /// Claims the login token and returns a session. The session can be
    /// used to authenticate to the graphql API.
    pub async fn claim(&mut self, ctx: &Context, token: &str) -> Result<Session<UserID>> {
        if self.is_claimed() {
            Err(anyhow!("The login was already claimed"))
        } else if self.valid_until() < ctx.now() {
            Err(anyhow!("The login is expired"))
        } else if self.token() != token {
            Err(anyhow!("Invalid login token"))
        } else {
            self.claimed = true;
            self.updated_at = ctx.now().naive_utc();
            let conn = ctx.conn().await?;
            *self = self.save_changes(&*conn)?;
            Ok(Session::new(
                self.user_id,
                ctx.now() + Duration::days(WEB_SESSION_VALID_DAYS),
            ))
        }
    }
}
