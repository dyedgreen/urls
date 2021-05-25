use crate::pages::ContextFilter;
use crate::Context;
use juniper::{EmptySubscription, RootNode};
use warp::{filters::BoxedFilter, Filter};

mod mutation;
mod objects;
mod query;
mod viewer;

type Schema = RootNode<'static, query::Query, mutation::Mutation, EmptySubscription<Context>>;

const XSRF_HEADER_NAME: &str = "X-XSRF-Token";

/// GraphQL API endpoint filter. The filter checks
/// for a valid XSRF token in a custom header.
pub fn api(ctx: impl ContextFilter + 'static) -> BoxedFilter<(impl warp::Reply,)> {
    let filter = warp::path::end()
        .and(ctx)
        .and(warp::header(XSRF_HEADER_NAME))
        .and_then(|ctx: Context, xsrf_token: String| async move {
            if ctx.check_xsrf_token(&xsrf_token) {
                Ok(ctx)
            } else {
                Err(warp::reject())
            }
        })
        .boxed();
    let schema = Schema::new(
        query::Query,
        mutation::Mutation,
        EmptySubscription::<Context>::new(),
    );
    juniper_warp::make_graphql_filter(schema, filter)
}
