use crate::config::Config;
use anyhow::Result;
use async_trait::async_trait;
use bb8_diesel::{bb8, DieselConnection, DieselConnectionManager};
use diesel::{sqlite::SqliteConnection, RunQueryDsl};

pub mod id;
pub mod models;

pub type Pool = bb8::Pool<DieselConnectionManager<SqliteConnection>>;
pub type PooledConnection<'a> =
    bb8::PooledConnection<'a, DieselConnectionManager<SqliteConnection>>;

diesel_migrations::embed_migrations!();

#[derive(Debug)]
struct Customizer;

#[async_trait]
impl bb8::CustomizeConnection<DieselConnection<SqliteConnection>, diesel::r2d2::Error>
    for Customizer
{
    async fn on_acquire(
        &self,
        conn: &mut DieselConnection<SqliteConnection>,
    ) -> Result<(), diesel::r2d2::Error> {
        let query = diesel::sql_query("PRAGMA foreign_keys = ON");
        query.execute(&*conn).map_err(|err| {
            log::error!("Failed to customize connection: {}", err);
            diesel::r2d2::Error::QueryError(err)
        })?;
        Ok(())
    }
}

pub async fn connect(config: &Config) -> Result<Pool> {
    let manager = DieselConnectionManager::new(config.database());
    let pool = bb8::Pool::builder()
        .max_size(8)
        .connection_customizer(Box::new(Customizer))
        .build(manager)
        .await?;

    {
        // Run migrations
        let conn = pool.get().await?;
        embedded_migrations::run(&*conn)?;

        // TODO: Do any first time setup if needed ...
    }

    Ok(pool)
}
