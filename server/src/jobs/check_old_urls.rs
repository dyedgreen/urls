use crate::db::models::Url;
use crate::schema::urls;
use crate::Context;
use anyhow::Result;
use chrono::Duration;
use diesel::prelude::*;

const DAYS_BETWEEN_CHECKS: i64 = 30;

/// Update the URL meta information and status for
/// old submissions.
pub async fn job(ctx: Context) -> Result<()> {
    let update_before = ctx.now() - Duration::days(DAYS_BETWEEN_CHECKS);
    let old_urls: Vec<Url> = urls::table
        .filter(urls::dsl::updated_at.lt(update_before.naive_utc()))
        .load(&*ctx.conn().await?)?;

    log::info!("Updating meta information for {} urls", old_urls.len());
    for mut url in old_urls {
        url.update_url_meta(&ctx)
            .await
            .map_err(|err| log::error!("Failed to update url meta: {}", err))
            .ok();
    }
    Ok(())
}
