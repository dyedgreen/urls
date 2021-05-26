use crate::db::id::{LoginID, UserID};
use crate::db::models::User;
use crate::schema::logins;
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use nanoid::nanoid;

const LOGIN_LIMIT_PER_HOUR: i64 = 3;
const LOGIN_VALID_MINUTES: i64 = 30;

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
                "Exceeded login limit of {} per hour for {}",
                LOGIN_LIMIT_PER_HOUR,
                user_id,
            ));
        }

        let login = Login {
            id: LoginID::new(),
            user_id,
            token: nanoid!(12),
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
}
