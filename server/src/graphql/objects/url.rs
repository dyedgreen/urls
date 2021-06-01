use crate::db::id::UrlID;
use crate::db::models::{Url, User};
use crate::Context;
use juniper::{graphql_object, FieldResult};
use juniper_relay::RelayConnectionNode;
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

    /// The unique invitation code for with
    /// this invitation.
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

    /// The user who issued this invitation.
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
}
