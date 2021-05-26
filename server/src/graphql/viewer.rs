use crate::db::models::User;
use crate::Context;
use juniper::{graphql_object, FieldResult, ID};

pub struct Viewer;

#[graphql_object(context = Context)]
impl Viewer {
    /// The ID of the logged in user, prefixed
    /// with `viewer-`, or `viewer-public` if not
    /// logged in.
    fn id(ctx: &Context) -> ID {
        ctx.maybe_user_id()
            .map(|id| format!("viewer-{}", id))
            .unwrap_or_else(|| "viewer-public".into())
            .into()
    }

    /// The currently logged in user, if any.
    async fn user(ctx: &Context) -> FieldResult<Option<User>> {
        Ok(ctx.maybe_user().await?)
    }
}
