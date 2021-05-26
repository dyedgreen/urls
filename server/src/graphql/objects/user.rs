use crate::db::id::UserID;
use crate::db::models::User;
use crate::Context;
use juniper::graphql_object;

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> UserID {
        self.id()
    }

    fn name(&self) -> &str {
        self.name()
    }
}
