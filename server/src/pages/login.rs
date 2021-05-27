use crate::pages::{error, ContextFilter};
use crate::Context;
use serde::Serialize;
use warp::{filters::BoxedFilter, http::Uri, reply::Response, Filter, Reply};

#[derive(Debug, Serialize)]
struct Data {
    auth_cookie: &'static str,
}

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    if ctx.is_logged_in() {
        Ok(warp::redirect::temporary(Uri::from_static("/")).into_response())
    } else {
        let data = Data {
            auth_cookie: super::AUTH_COOKIE_NAME,
        };
        Ok(ctx.render("pages/login.html", Some(data))?.into_response())
    }
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
