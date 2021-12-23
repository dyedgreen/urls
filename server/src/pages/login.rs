use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::{filters::BoxedFilter, http::Uri, reply::Response, Filter, Reply};

#[derive(Template)]
#[template(path = "pages/login.html")]
struct Page<'a> {
    xsrf_token: &'a str,
    is_logged_in: bool,
}

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    if ctx.is_logged_in() {
        Ok(warp::redirect::temporary(Uri::from_static("/")).into_response())
    } else {
        let page = Page {
            xsrf_token: ctx.xsrf_token(),
            is_logged_in: false,
        };
        let resp = super::xsrf::cookie(&ctx, page);
        Ok(resp.into_response())
    }
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
