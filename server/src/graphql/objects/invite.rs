use crate::db::id::InviteID;
use crate::db::models::{Invite, User};
use crate::Context;
use juniper::{graphql_object, FieldResult};
use juniper_relay_connection::RelayConnectionNode;

impl RelayConnectionNode for Invite {
    type Cursor = InviteID;

    fn cursor(&self) -> Self::Cursor {
        self.id()
    }

    fn connection_type_name() -> &'static str {
        "InviteConnection"
    }

    fn edge_type_name() -> &'static str {
        "InviteConnectionEdge"
    }
}

#[graphql_object(context = Context)]
impl Invite {
    /// A globally unique identifier for this
    /// invitation.
    fn id(&self) -> InviteID {
        self.id()
    }

    /// The unique invitation code for with
    /// this invitation.
    fn token(&self) -> &str {
        self.token()
    }

    /// The user who issued this invitation.
    async fn created_by(&self, ctx: &Context) -> FieldResult<User> {
        Ok(self.created_by(ctx).await?)
    }

    /// The user who claimed this invitation when registering, or null
    /// if it has not been claimed yet.
    async fn claimed_by(&self, ctx: &Context) -> FieldResult<Option<User>> {
        Ok(self.claimed_by(ctx).await?)
    }
}
