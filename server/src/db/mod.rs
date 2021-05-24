use crate::config::Config;
use anyhow::Result;
use bb8_diesel::{bb8, DieselConnectionManager};
use chrono::{DateTime, Utc};
use diesel::sqlite::SqliteConnection;

diesel_migrations::embed_migrations!();
pub type Pool = bb8::Pool<DieselConnectionManager<SqliteConnection>>;
pub type PooledConnection<'a> =
    bb8::PooledConnection<'a, DieselConnectionManager<SqliteConnection>>;

#[derive(Debug, Clone)]
pub struct Context {
    pool: Pool,
    request_time: DateTime<Utc>,
}

impl Context {
    /// Create a new request context.
    pub fn new(pool: &Pool) -> Self {
        Self {
            pool: pool.clone(),
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

pub async fn connect(config: &Config) -> Result<Pool> {
    let manager = DieselConnectionManager::new(config.database());
    let pool = bb8::Pool::builder().max_size(8).build(manager).await?;

    {
        // Run migrations
        let conn = pool.get().await?;
        embedded_migrations::run_with_output(&*conn, &mut std::io::stdout())?;

        // TODO: Do any first time setup if needed ...
    }

    Ok(pool)
}
