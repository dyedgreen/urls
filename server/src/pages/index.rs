use crate::db::id::UrlID;
use crate::db::models::{Url, User};
use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

#[derive(Template)]
#[template(path = "pages/index.html")]
struct Page<'a> {
    xsrf_token: &'a str,
    is_logged_in: bool,
}

#[derive(Template)]
#[template(path = "partials/url.html")]
struct UrlPartial<'a> {
    url: &'a Url,
    created_by: &'a User,
}

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    let page = Page {
        xsrf_token: ctx.xsrf_token(),
        is_logged_in: ctx.is_logged_in(),
    };
    Ok(page.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
