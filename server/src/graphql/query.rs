use crate::db::id::{CommentID, UrlID};
use crate::db::models::{Comment, Url};
use crate::graphql::viewer::Viewer;
use crate::Context;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    /// The `viewer` field represents the
    /// current visitor or authenticated user,
    /// and groups fields which depend on the
    /// current viewer.
    fn viewer() -> Viewer {
        Viewer
    }

    #[graphql(name = "fetch__Url")]
    async fn fetch_url(ctx: &Context, id: UrlID) -> FieldResult<Url> {
        Ok(Url::find(ctx, id).await?)
    }

    #[graphql(name = "fetch__Comment")]
    async fn fetch_url(ctx: &Context, id: CommentID) -> FieldResult<Comment> {
        Ok(Comment::find(ctx, id).await?)
    }
}
