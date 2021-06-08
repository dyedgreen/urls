use crate::db::id::CommentID;
use crate::db::models::{Comment, Url, User};
use crate::schema::comments;
use crate::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use juniper_relay::{RelayConnection, RelayConnectionNode};

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

    /// Comments which directly reply to this comment.
    async fn replies(
        &self,
        ctx: &Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<RelayConnection<Comment>> {
        let conn = ctx.conn().await?;
        RelayConnection::new(first, after, last, before, |after, before, limit| {
            let mut query = comments::table
                .filter(comments::dsl::replies_to.eq(self.id()))
                .order_by(comments::dsl::created_at.asc())
                .into_boxed();

            if let Some(after) = after {
                let after: Comment = comments::table.find(after).get_result(&*conn)?;
                query = query.filter(comments::dsl::created_at.gt(after.created_at().naive_utc()));
            }

            if let Some(before) = before {
                let before: Comment = comments::table.find(before).get_result(&*conn)?;
                query = query.filter(comments::dsl::created_at.lt(before.created_at().naive_utc()));
            }

            if let Some(limit) = limit {
                query = query.limit(limit);
            }

            Ok(query.load(&*conn)?)
        })
    }
}
