use crate::pages::{error, ContextFilter};
use crate::Context;
use warp::{filters::BoxedFilter, reply::Response, Filter};

async fn handle(_ctx: Context) -> Result<&'static str, error::ServerError> {
    Ok("todo")
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
