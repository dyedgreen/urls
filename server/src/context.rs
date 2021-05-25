use crate::db::id::UserID;
use crate::db::{Pool, PooledConnection};
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Context {
    pool: Pool,
    logged_in_user: Option<UserID>,
    request_time: DateTime<Utc>,
}

impl Context {
    /// Create a new request context.
    pub fn new(pool: &Pool, logged_in_user: Option<UserID>) -> Self {
        Self {
            pool: pool.clone(),
            logged_in_user,
            request_time: Utc::now(),
        }
    }

    /// Retrieve a database connection from the
    /// connection pool.
    pub async fn conn(&self) -> Result<PooledConnection<'_>> {
        Ok(self.pool.get().await?)
    }

    /// Prefer this over `Utc::now()`, since it
    /// will remain consistent over the life-time
    /// of a given request.
    pub fn now(&self) -> DateTime<Utc> {
        self.request_time
    }
}
