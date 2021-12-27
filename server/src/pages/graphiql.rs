use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use warp::Reply;
use warp::{filters::BoxedFilter, reply::Response, Filter};

#[derive(Template)]
#[template(path = "pages/graphiql.html")]
struct Page<'a> {
    xsrf_token: &'a str,
}

async fn handle(ctx: &Context) -> Result<Response, error::ServerError> {
    let page = Page {
        xsrf_token: ctx.xsrf_token(),
    };
    Ok(page.into_response())
}

pub fn page(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::end()
        .and(ctx)
        .and_then(|ctx: Context| async move { error::reply(&ctx, handle(&ctx).await) })
        .boxed()
}
