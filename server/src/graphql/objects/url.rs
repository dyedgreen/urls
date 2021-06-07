use crate::db::id::{CommentID, UrlID};
use crate::db::models::{Comment, Url, User};
use crate::schema::comments;
use crate::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use juniper_relay::{RelayConnection, RelayConnectionNode};
use std::convert::TryInto;

impl RelayConnectionNode for Url {
    type Cursor = UrlID;

    fn cursor(&self) -> Self::Cursor {
        self.id()
    }

    fn connection_type_name() -> &'static str {
        "UrlConnection"
    }

    fn edge_type_name() -> &'static str {
        "UrlConnectionEdge"
    }
}

#[graphql_object(context = Context)]
impl Url {
    /// A globally unique identifier for this
    /// url.
    fn id(&self) -> UrlID {
        self.id()
    }

    /// The URL that was submitted.
    fn url(&self) -> FieldResult<String> {
        Ok(self.url()?.to_string())
    }

    /// The title of the linked page. This is parsed
    /// from the page when the url is submitted.
    fn title(&self) -> Option<&str> {
        self.title()
    }

    /// A description of the linked page. This is parsed
    /// from the page when the url is submitted.
    fn description(&self) -> Option<&str> {
        self.description()
    }

    /// The image url of the linked page. This is the
    /// image that would e.g. be displayed in a Twitter
    /// timeline. These images typically have a 2:1 aspect
    /// ratio.
    fn image(&self) -> FieldResult<Option<String>> {
        Ok(self.image()?.map(|uri| uri.to_string()))
    }

    /// The time this url was submitted.
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at()
    }

    /// The user who submitted this URL.
    async fn created_by(&self, ctx: &Context) -> FieldResult<User> {
        Ok(self.created_by(ctx).await?)
    }

    /// The total number of upvotes this submission has received.
    async fn upvote_count(&self, ctx: &Context) -> FieldResult<i32> {
        Ok(self.upvote_count(ctx).await?.try_into()?)
    }

    /// If the URL was upvoted by the current viewer.
    async fn upvoted_by_viewer(&self, ctx: &Context) -> FieldResult<bool> {
        Ok(self.upvoted_by_viewer(ctx).await?)
    }

    /// List comments and optionally filter by replies-to
    /// thread.
    async fn comments(
        &self,
        ctx: &Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        replies_to: Option<CommentID>,
    ) -> FieldResult<RelayConnection<Comment>> {
        let conn = ctx.conn().await?;
        RelayConnection::new(first, after, last, before, |after, before, limit| {
            let mut query = comments::table
                .filter(comments::dsl::url_id.eq(self.id()))
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

            if let Some(replies_to) = replies_to {
                query = query.filter(comments::dsl::replies_to.eq(replies_to));
            }

            Ok(query.load(&*conn)?)
        })
    }
}
