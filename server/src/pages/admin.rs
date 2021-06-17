use crate::pages::ContextFilter;
use crate::{Config, Context};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

async fn check_backup_permissions(ctx: Context) -> Result<(), Rejection> {
    let log_err = |err| {
        log::warn!("Unauthorized attempt to download database backup: {}", err);
        warp::reject::not_found()
    };
    ctx.user()
        .await
        .map_err(log_err)?
        .check_permissions(&ctx, |perm| perm.access_admin_backups())
        .await
        .map_err(log_err)?;
    Ok(())
}

pub fn backup(ctx: impl ContextFilter + 'static) -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(ctx)
        .and_then(check_backup_permissions)
        .untuple_one()
        .and(warp::fs::file(Config::env().database_file()))
        .map(|file| {
            warp::reply::with_header(
                file,
                "Content-Disposition",
                "attachment; filename=\"backup.sqlite\"",
            )
        })
        .boxed()
}
