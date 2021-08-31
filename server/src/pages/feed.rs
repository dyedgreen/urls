use crate::db::models::{Url, UrlOrdering};
use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use chrono::{DateTime, Utc};
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

const FEED_SIZE: i64 = 32;

#[derive(Template)]
#[template(path = "pages/feed.xml")]
struct Page {
    pub_date: DateTime<Utc>,
    urls: Vec<Url>,
}

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    let (urls, _) = Url::paginate(&ctx, UrlOrdering::Recent, 0, FEED_SIZE).await?;
    let pub_date = urls.get(0).map(|url| url.created_at()).unwrap_or(ctx.now());
    Ok(Page { pub_date, urls }.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path("feed.xml")
        .and(warp::path::end())
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
