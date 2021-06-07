use crate::db::id::CommentID;
use crate::db::models::{Comment, Url, User};
use crate::Context;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, FieldResult};
use juniper_relay::RelayConnectionNode;

impl RelayConnectionNode for Comment {
    type Cursor = CommentID;

    fn cursor(&self) -> Self::Cursor {
        self.id()
    }

    fn connection_type_name() -> &'static str {
        "CommentConnection"
    }

    fn edge_type_name() -> &'static str {
        "CommentConnectionEdge"
    }
}

#[graphql_object(context = Context)]
impl Comment {
    /// A globally unique identifier for this
    /// comment.
    fn id(&self) -> CommentID {
        self.id()
    }

    /// The raw markdown text content of this comment.
    fn text(&self) -> &str {
        self.text()
    }

    /// An html rendered version of this comment. The
    /// raw markdown has been sanitized and can be considered
    /// safe.
    fn html(&self) -> String {
        self.html()
    }

    /// The URL that was commented on.
    async fn url(&self, ctx: &Context) -> FieldResult<Url> {
        Ok(self.url(ctx).await?)
    }

    /// The time this comment was made.
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at()
    }

    /// The user who made this comment.
    async fn created_by(&self, ctx: &Context) -> FieldResult<User> {
        Ok(self.created_by(ctx).await?)
    }

    /// The comment which was directly replied to,
    /// is any.
    async fn replies_to(&self, ctx: &Context) -> FieldResult<Option<Comment>> {
        Ok(self.replies_to(ctx).await?)
    }
}
