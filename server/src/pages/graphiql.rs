use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::{filters::BoxedFilter, reply::Response, Filter, Reply};

#[derive(Template)]
#[template(path = "pages/graphiql.html")]
struct Page<'a> {
    xsrf_token: &'a str,
}

async fn handle(ctx: Context) -> Result<Response, error::ServerError> {
    let page = Page {
        xsrf_token: ctx.xsrf_token(),
    };
    let resp = super::xsrf::cookie(&ctx, page);
    Ok(resp.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(handle(ctx).await) })
        .boxed()
}
