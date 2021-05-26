use crate::db::models::{NewUserInput, User};
use crate::Context;
use juniper::{graphql_object, FieldResult, GraphQLObject};

pub struct Mutation;

/// A value mutations can return, if there
/// is no other interesting information
/// that occured.
#[derive(Debug, Copy, Clone, GraphQLObject)]
struct Void {
    /// Indicates weather the operation was
    /// successful.
    ok: bool,
}

impl Void {
    fn ok() -> FieldResult<Self> {
        Ok(Self { ok: true })
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    /// TODO: Delete this function or make it admin only or smth ...
    async fn create_user(ctx: &Context, input: NewUserInput) -> FieldResult<User> {
        let user = User::create(ctx, input).await?;
        Ok(user)
    }

    /// Request a login code for the user associated with the given `email`. Note
    /// this this might fail because of rate limiting.
    async fn request_login(ctx: &Context, email: String) -> FieldResult<Void> {
        let user = User::find_by_email(ctx, &email).await?;
        user.request_login(ctx).await?;
        Void::ok()
    }
}
