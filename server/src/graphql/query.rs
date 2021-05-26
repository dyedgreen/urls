use crate::graphql::viewer::Viewer;
use crate::Context;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    /// The `viewer` field represents the
    /// current visitor or authenticated user,
    /// and groups fields which depend on the
    /// current viewer.
    fn viewer() -> Viewer {
        Viewer
    }
}
