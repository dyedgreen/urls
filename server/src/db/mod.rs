use crate::config::Config;
use anyhow::Result;
use bb8_diesel::{bb8, DieselConnectionManager};
use diesel::sqlite::SqliteConnection;

pub mod id;

pub type Pool = bb8::Pool<DieselConnectionManager<SqliteConnection>>;
pub type PooledConnection<'a> =
    bb8::PooledConnection<'a, DieselConnectionManager<SqliteConnection>>;

diesel_migrations::embed_migrations!();

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
