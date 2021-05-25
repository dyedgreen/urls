use crate::pages::ContextFilter;
use crate::Context;
use juniper::{EmptySubscription, RootNode};
use warp::{filters::BoxedFilter, Filter};

mod mutation;
mod objects;
mod query;
mod viewer;

type Schema = RootNode<'static, query::Query, mutation::Mutation, EmptySubscription<Context>>;

/// GraphQL API endpoint filter.
pub fn api(ctx: impl ContextFilter + 'static) -> BoxedFilter<(impl warp::Reply,)> {
    // TODO: XSRF token protection
    let filter = warp::path::end().and(ctx).boxed();
    let schema = Schema::new(
        query::Query,
        mutation::Mutation,
        EmptySubscription::<Context>::new(),
    );
    juniper_warp::make_graphql_filter(schema, filter)
}
