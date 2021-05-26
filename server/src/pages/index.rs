use crate::pages::{error, ContextFilter};
use crate::Context;
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    Ok(ctx.render::<()>("pages/index.html", None)?.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
