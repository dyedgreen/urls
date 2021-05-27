use crate::db::id::UserID;
use crate::db::models::User;
use crate::Context;
use chrono::{DateTime, Utc};
use juniper::graphql_object;

#[graphql_object(context = Context)]
impl User {
    /// A globally unique identifier for this
    /// user.
    fn id(&self) -> UserID {
        self.id()
    }

    /// The display name of this user.
    fn name(&self) -> &str {
        self.name()
    }

    /// The date when this user account
    /// was created.
    fn joined(&self) -> DateTime<Utc> {
        self.created_at()
    }
}
