use crate::db::id::{CommentID, UrlID, UserID};
use crate::db::models::{Url, User};
use crate::schema::comments;
use crate::Context;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use pulldown_cmark::{html, Options, Parser};
use validator::Validate;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(Url)]
#[belongs_to(User, foreign_key = "created_by")]
#[belongs_to(Comment, foreign_key = "replies_to")]
pub struct Comment {
    id: CommentID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    comment: String,
    url_id: UrlID,
    created_by: UserID,
    replies_to: Option<CommentID>,
}

#[derive(Debug, Clone, Validate, GraphQLInputObject)]
pub struct NewCommentInput {
    #[validate(length(min = 1, message = "The comment can not be empty"))]
    comment: String,
    url: UrlID,
    replies_to: Option<CommentID>,
}

impl Comment {
    pub fn id(&self) -> CommentID {
        self.id
    }

    pub fn text(&self) -> &str {
        &self.comment
    }

    /// Render the given markdown `text` as
    /// html. This safely escapes and html present
    /// on the input.
    pub fn html(&self) -> String {
        let mut out = String::new();
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TASKLISTS);
        opts.insert(Options::ENABLE_SMART_PUNCTUATION);
        let parser = Parser::new_ext(self.text(), opts);
        html::push_html(&mut out, parser);
        out
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }

    pub async fn url(&self, ctx: &Context) -> Result<Url> {
        Ok(Url::find(ctx, self.url_id).await?)
    }

    pub async fn created_by(&self, ctx: &Context) -> Result<User> {
        Ok(User::find(ctx, self.created_by).await?)
    }

    pub async fn replies_to(&self, ctx: &Context) -> Result<Option<Self>> {
        if let Some(comment_id) = self.replies_to {
            let comment = comments::table
                .find(comment_id)
                .get_result(&*ctx.conn().await?)?;
            Ok(Some(comment))
        } else {
            Ok(None)
        }
    }
}

impl Comment {
    pub async fn find(ctx: &Context, id: CommentID) -> Result<Self> {
        let comment = comments::table.find(id).get_result(&*ctx.conn().await?)?;
        Ok(comment)
    }
}

impl Comment {
    /// Creates a new comment in the database.
    pub async fn create(ctx: &Context, mut input: NewCommentInput) -> Result<Self> {
        input.comment = input.comment.trim().into();
        input.validate()?;

        let comment = Comment {
            id: CommentID::new(),
            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),

            comment: input.comment,
            url_id: input.url,
            created_by: ctx.user_id()?,
            replies_to: input.replies_to,
        };
        diesel::insert_into(comments::table)
            .values(&comment)
            .execute(&*ctx.conn().await?)?;

        Ok(comment)
    }

    /// Deletes a given comment from the database. If the comment
    /// has replies, the comment is censored instead. (This is done
    /// to prevent loosing deletion of replies.)
    pub async fn delete(&mut self, ctx: &Context) -> Result<()> {
        if self.created_by != ctx.user_id()? {
            ctx.user()
                .await?
                .check_permissions(ctx, |perm| perm.delete_any_comment())
                .await?;
        }

        let replies_count: i64 = comments::table
            .filter(comments::dsl::replies_to.eq(self.id()))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;

        if replies_count > 0 {
            self.updated_at = ctx.now().naive_utc();
            self.comment = "[DELETED]".to_string();
            *self = self.save_changes(&*ctx.conn().await?)?;
        } else {
            diesel::delete(&*self).execute(&*ctx.conn().await?)?;
        }

        Ok(())
    }
}
