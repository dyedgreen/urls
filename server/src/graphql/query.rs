use crate::db::id::{CommentID, UrlID, UserID};
use crate::db::models::{Comment, Url, User};
use crate::graphql::{search::Search, viewer::Viewer};
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

    /// Search through all submitted urls.
    async fn search(query: String) -> Search {
        Search::new(query)
    }

    #[graphql(name = "fetch__Url")]
    async fn fetch_url(ctx: &Context, id: UrlID) -> FieldResult<Url> {
        Ok(Url::find(ctx, id).await?)
    }

    #[graphql(name = "fetch__Comment")]
    async fn fetch_comment(ctx: &Context, id: CommentID) -> FieldResult<Comment> {
        Ok(Comment::find(ctx, id).await?)
    }

    #[graphql(name = "fetch__User")]
    async fn fetch_user(ctx: &Context, id: UserID) -> FieldResult<User> {
        Ok(User::find(ctx, id).await?)
    }
}
