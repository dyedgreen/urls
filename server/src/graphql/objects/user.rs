use crate::db::id::UserID;
use crate::db::models::{Invite, Permission, User};
use crate::Context;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, FieldResult};

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

    /// Invitation used by this user to register
    /// their account, if any.
    async fn invite(&self, ctx: &Context) -> FieldResult<Option<Invite>> {
        Ok(self.invite(ctx).await?)
    }

    /// List of active permissions for this
    /// user.
    async fn permissions(&self, ctx: &Context) -> FieldResult<Vec<Permission>> {
        Ok(self.permissions(ctx).await?)
    }
}
