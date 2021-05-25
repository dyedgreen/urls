use crate::db::models::{NewUserInput, User};
use crate::Context;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    /// TODO: Delete this function or make it admin only or smth ...
    async fn create_user(ctx: &Context, input: NewUserInput) -> FieldResult<User> {
        let user = User::create(ctx, input).await?;
        Ok(user)
    }
}
