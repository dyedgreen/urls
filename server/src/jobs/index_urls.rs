use crate::db::models::Url;
use crate::schema::urls;
use crate::Context;
use anyhow::Result;
use chrono::Duration;
use diesel::prelude::*;

const SECONDS_BETWEEN_CHECKS: i64 = 60;

/// Updates the search index with new and modified
/// urls.
pub async fn job(ctx: Context) -> Result<()> {
    let updated_after = ctx.now() - Duration::seconds(SECONDS_BETWEEN_CHECKS + 10);
    let urls: Vec<Url> = urls::table
        .filter(urls::dsl::updated_at.gt(updated_after.naive_utc()))
        .load(&*ctx.conn().await?)?;

    if !urls.is_empty() {
        log::info!("Re-indexing {} urls", urls.len());
        ctx.search().delete_urls(urls.iter())?;
        ctx.search().index_urls(urls.iter())?;
    }
    Ok(())
}
