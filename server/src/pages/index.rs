use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

#[derive(Template)]
#[template(path = "pages/index.html")]
struct Page {
    is_logged_in: bool,
}

async fn handle(ctx: Context) -> Result<impl Reply, error::ServerError> {
    let page = Page {
        is_logged_in: ctx.is_logged_in(),
    };
    Ok(page)
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
