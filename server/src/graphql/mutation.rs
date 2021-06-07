use super::viewer::Viewer;
use crate::db::id::{CommentID, UrlID};
use crate::db::models::{
    Comment, Invite, NewCommentInput, NewUrlInput, NewUserInput, Permission, Role, UpdateUserInput,
    Url, User,
};
use crate::{Config, Context};
use juniper::{graphql_object, FieldResult, GraphQLObject};
use validator::Validate;

pub struct Mutation;

/// A value mutations can return, if there
/// is no other interesting object that could
/// be returned.
#[derive(Debug, Copy, Clone, GraphQLObject)]
struct Void {
    /// Indicates whether the operation was
    /// successful.
    ok: bool,
}

impl Void {
    fn ok() -> FieldResult<Self> {
        Ok(Self { ok: true })
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    /// Register a new user by claiming the provided invitation code `token`.
    async fn register_user(ctx: &Context, input: NewUserInput, token: String) -> FieldResult<User> {
        input.validate()?; // surface input errors early for better UX
        let invite = Invite::find_by_token(ctx, &token).await?;
        let user = User::create_with_invite(ctx, input, invite).await?;
        Ok(user)
    }

    /// Update details for the currently logged in user.
    async fn update_user(ctx: &Context, input: UpdateUserInput) -> FieldResult<Viewer> {
        let mut user = ctx.user().await?;
        user.update(ctx, input).await?;
        Ok(Viewer)
    }

    /// Grants the given permission to the user with the
    /// provided email.
    async fn grant_permission(
        ctx: &Context,
        permission: Permission,
        email: String,
    ) -> FieldResult<User> {
        ctx.user()
            .await?
            .check_permissions(ctx, |perm| perm.modify_user_roles())
            .await?;
        let user = User::find_by_email(ctx, &email).await?;
        Role::create(ctx, user.id(), permission).await?;
        Ok(user)
    }

    /// Revokes the given permission from the user with
    /// the provided email.
    async fn revoke_permission(
        ctx: &Context,
        permission: Permission,
        email: String,
    ) -> FieldResult<User> {
        ctx.user()
            .await?
            .check_permissions(ctx, |perm| perm.modify_user_roles())
            .await?;
        let user = User::find_by_email(ctx, &email).await?;
        Role::delete_by_permission(ctx, user.id(), permission).await?;
        Ok(user)
    }

    /// Request a login code for the user associated with the given `email`. Note
    /// this this might fail because of rate limiting.
    async fn request_login(ctx: &Context, email: String) -> FieldResult<Void> {
        let user = User::find_by_email(ctx, &email).await?;
        user.request_login(ctx).await?;
        Void::ok()
    }

    /// Login using the given `email` and a login code (or token) previously obtained
    /// from `request_login`.
    async fn login(ctx: &Context, email: String, token: String) -> FieldResult<String> {
        let user = User::find_by_email(ctx, &email).await?;
        let session = user.login(ctx, &token).await?;
        Ok(session.base64(Config::env().session_key())?)
    }

    /// Create a new invite, issued by the currently logged in user.
    async fn issue_invite(ctx: &Context) -> FieldResult<Invite> {
        let user = ctx.user().await?;
        Ok(Invite::create(ctx, &user).await?)
    }

    /// Create a new URL and crawls the associated HTML page for
    /// meta data.
    async fn submit_url(ctx: &Context, input: NewUrlInput) -> FieldResult<Url> {
        Ok(Url::create(ctx, input, ctx.user_id()?).await?)
    }

    /// Deletes a submitted URL. URLs can only be deleted by moderators
    /// or the user who originally submitted them.
    async fn delete_url(ctx: &Context, url: UrlID) -> FieldResult<Url> {
        let url = Url::find(ctx, url).await?;
        url.delete(ctx).await?;
        Ok(url)
    }

    /// Upvote the given URL as the viewer.
    async fn upvote_url(ctx: &Context, url: UrlID) -> FieldResult<Url> {
        let url = Url::find(ctx, url).await?;
        url.upvote(ctx).await?;
        Ok(url)
    }

    /// Rescind a previous upvote for the given URL.
    async fn rescind_url_upvote(ctx: &Context, url: UrlID) -> FieldResult<Url> {
        let url = Url::find(ctx, url).await?;
        url.rescind_upvote(ctx).await?;
        Ok(url)
    }

    /// Comment on the given URL as the viewer.
    async fn comment(ctx: &Context, input: NewCommentInput) -> FieldResult<Comment> {
        Ok(Comment::create(ctx, input).await?)
    }

    /// Delete the given comment. Only the original author, or a moderator
    /// is allowed to delete comments.
    async fn delete_comment(ctx: &Context, comment: CommentID) -> FieldResult<Comment> {
        let mut comment = Comment::find(ctx, comment).await?;
        comment.delete(ctx).await?;
        Ok(comment)
    }
}
