use crate::Context;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    /// TODO
    fn hello() -> &str {
        "Hello world!"
    }
}
