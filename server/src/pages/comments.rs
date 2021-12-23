use crate::db::id::UrlID;
use crate::db::models::{Url, User};
use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

#[derive(Template)]
#[template(path = "pages/comments.html")]
struct Page<'a> {
    url_partial: UrlPartial,
    xsrf_token: &'a str,
    is_logged_in: bool,
}

#[derive(Template)]
#[template(path = "partials/url.html")]
struct UrlPartial {
    url: Url,
    created_by: User,
    upvote_count: i64,
    is_upvoted_by_viewer: bool,
    comment_count: i64,
    is_logged_in: bool,
}

async fn handle(ctx: Context, url_id: UrlID) -> Result<Response, error::ServerError> {
    let url = Url::find(&ctx, url_id).await.map_err(error::not_found)?;
    let page = Page {
        url_partial: UrlPartial {
            created_by: url.created_by(&ctx).await?,
            upvote_count: url.upvote_count(&ctx).await?,
            is_upvoted_by_viewer: url.upvoted_by_viewer(&ctx).await?,
            comment_count: url.comment_count(&ctx).await?,
            is_logged_in: ctx.is_logged_in(),
            url,
        },
        xsrf_token: ctx.xsrf_token(),
        is_logged_in: ctx.is_logged_in(),
    };
    let resp = super::xsrf::cookie(&ctx, page);
    Ok(resp.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::param()
        .and(warp::path::end())
        .and(ctx)
        .and_then(
            |url_id: UrlID, ctx: Context| async move { error::reply(handle(ctx, url_id).await) },
        )
        .boxed()
}
