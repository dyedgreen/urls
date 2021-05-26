use crate::pages::{error, ContextFilter};
use crate::Context;
use warp::{filters::BoxedFilter, http::Uri, reply::Response, Filter, Reply};

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    if ctx.is_logged_in() {
        Ok(warp::redirect(Uri::from_static("/")).into_response())
    } else {
        Ok(ctx.render::<()>("pages/login.html", None)?.into_response())
    }
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
