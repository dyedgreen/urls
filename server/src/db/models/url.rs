use crate::db::id::{UrlID, UserID};
use crate::db::models::User;
use crate::schema::{url_upvotes, urls, users};
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use futures_util::StreamExt;
use juniper::GraphQLInputObject;
use meta_parser::Meta;
use validator::Validate;
use warp::http::Uri;

const INCLUDE_DAYS_IN_RANKED: i64 = 7;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
pub struct Url {
    id: UrlID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    url: String,
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
    created_by: UserID,
}

#[derive(Debug, Clone, Validate, GraphQLInputObject)]
pub struct NewUrlInput {
    #[validate(url(message = "Please submit a valid URL"))]
    url: String,
}

impl Url {
    pub fn id(&self) -> UrlID {
        self.id
    }

    /// The URL being shared. This should usually
    /// succeed, unless the database was corrupted.
    pub fn url(&self) -> Result<Uri> {
        Ok(self.url.parse()?)
    }

    /// Return the url as a `&str`. This always succeeds
    /// but might return an invalid Uri, since it simply
    /// returns the value found in the database.
    pub fn url_str(&self) -> &str {
        &self.url
    }

    /// The title provided by the linked html document, if
    /// available.
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(AsRef::as_ref)
    }

    /// The description provided by the linked html
    /// document, if available.
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(AsRef::as_ref)
    }

    /// The image uri provided by the linked html
    /// document, if available.
    pub fn image(&self) -> Result<Option<Uri>> {
        let maybe_uri = self
            .image
            .as_ref()
            .map(|img_uri| img_uri.parse())
            .transpose()?;
        Ok(maybe_uri)
    }

    /// Return the image uri as a `&str`. This always succeeds
    /// but might return an invalid Uri, since it simply
    /// returns the value found in the database.
    pub fn image_str(&self) -> Option<&str> {
        self.image.as_ref().map(AsRef::as_ref)
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }

    pub async fn created_by(&self, ctx: &Context) -> Result<User> {
        let user = users::table
            .find(self.created_by)
            .get_result(&*ctx.conn().await?)?;
        Ok(user)
    }

    pub async fn upvote_count(&self, ctx: &Context) -> Result<i64> {
        let count = url_upvotes::table
            .filter(url_upvotes::dsl::url_id.eq(self.id))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;
        Ok(count)
    }

    pub async fn upvoted_by_viewer(&self, ctx: &Context) -> Result<bool> {
        if let Some(user_id) = ctx.maybe_user_id() {
            let count: i64 = url_upvotes::table
                .filter(url_upvotes::dsl::url_id.eq(self.id))
                .filter(url_upvotes::dsl::user_id.eq(user_id))
                .select(diesel::dsl::count_star())
                .get_result(&*ctx.conn().await?)?;
            Ok(count == 1)
        } else {
            Ok(false)
        }
    }
}

/// Determine how to order and filter the url
/// pagination.
#[derive(Debug, Clone, Copy)]
pub enum UrlOrdering {
    /// Default ranking used on the home page.
    Ranked,
    /// All time best submissions.
    Best,
    /// Submissions from the given user, ranked
    /// chronologically.
    User(UserID),
    /// All submissions, ranked chronologically.
    Recent,
}

impl Url {
    pub async fn find(ctx: &Context, url_id: UrlID) -> Result<Self> {
        let url = urls::table.find(url_id).get_result(&*ctx.conn().await?)?;
        Ok(url)
    }

    /// Returns URLs ranked according to the given ordering, as well, as the total number of
    /// available pages for the given ordering.
    pub async fn paginate(
        ctx: &Context,
        order: UrlOrdering,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Self>, i64)> {
        use UrlOrdering::*;

        let total_count_query = urls::table.select(diesel::dsl::count_star());
        let total_count: i64 = match order {
            Ranked | Best | Recent => total_count_query.get_result(&*ctx.conn().await?)?,
            User(creator_id) => total_count_query
                .filter(urls::dsl::created_by.eq(creator_id))
                .get_result(&*ctx.conn().await?)?,
        };
        let page_count = if total_count % page_size != 0 {
            total_count / page_size + 1
        } else {
            total_count / page_size
        };

        let query = urls::table.order_by(urls::dsl::created_at.desc());
        let page = match order {
            Ranked => {
                let count_vote_after = ctx.now() - Duration::days(INCLUDE_DAYS_IN_RANKED);
                let join_on_recent = url_upvotes::dsl::url_id
                    .eq(urls::dsl::id)
                    .and(url_upvotes::dsl::created_at.ge(count_vote_after.naive_utc()));
                query
                    .left_outer_join(url_upvotes::table.on(join_on_recent))
                    .group_by(urls::all_columns)
                    .order_by(diesel::dsl::count(urls::dsl::id).desc())
                    .then_order_by(url_upvotes::dsl::created_at.is_null().asc()) // order 1 higher than none
                    .then_order_by(urls::dsl::created_at.desc())
                    .select(urls::all_columns)
                    .offset(page * page_size)
                    .limit(page_size)
                    .load(&*ctx.conn().await?)?
            }
            Best => query
                .left_outer_join(url_upvotes::table)
                .group_by(urls::all_columns)
                .order_by(diesel::dsl::count(urls::dsl::id).desc())
                .then_order_by(url_upvotes::dsl::created_at.is_null().asc()) // order 1 higher than none
                .then_order_by(urls::dsl::created_at.desc())
                .select(urls::all_columns)
                .offset(page * page_size)
                .limit(page_size)
                .load(&*ctx.conn().await?)?,
            User(creator_id) => query
                .filter(urls::dsl::created_by.eq(creator_id))
                .offset(page * page_size)
                .limit(page_size)
                .load(&*ctx.conn().await?)?,
            Recent => query
                .offset(page * page_size)
                .limit(page_size)
                .load(&*ctx.conn().await?)?,
        };

        Ok((page, page_count))
    }
}

impl Url {
    /// Creates a new role for the given user and assigns the given
    /// permission.
    pub async fn create(ctx: &Context, input: NewUrlInput, created_by: UserID) -> Result<Self> {
        input.validate()?;
        let NewUrlInput { url } = input;

        // verify URL is unique, to avoid an additional query
        let exists: i64 = urls::table
            .filter(urls::dsl::url.eq(&url))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;
        if exists > 0 {
            return Err(anyhow!("The url was already submitted"));
        }

        let resp = ctx.http_client().get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("Failed to load url with status {}", resp.status()));
        }

        let mut meta = Meta::new();
        let mut stream = resp.bytes_stream();
        while let Some(part) = stream.next().await {
            meta.parse(&part?);
        }

        let url = Url {
            id: UrlID::new(),
            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),

            url,
            title: meta.title,
            description: meta.description,
            image: meta.image,
            created_by,
        };

        diesel::insert_into(urls::table)
            .values(&url)
            .execute(&*ctx.conn().await?)?;
        Ok(url)
    }

    /// Deletes the given URL from the database. URLs can only be deleted
    /// by moderators or the user who created them.
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        if self.created_by != ctx.user_id()? {
            ctx.user()
                .await?
                .check_permissions(ctx, |perm| perm.delete_any_url())
                .await?;
        }
        let conn = ctx.conn().await?;
        let upvotes = url_upvotes::table.filter(url_upvotes::dsl::url_id.eq(self.id));
        diesel::delete(upvotes).execute(&*conn)?;
        diesel::delete(self).execute(&*conn)?;
        Ok(())
    }

    /// Upvote the URL as the logged in user.
    pub async fn upvote(&self, ctx: &Context) -> Result<()> {
        diesel::insert_into(url_upvotes::table)
            .values((
                url_upvotes::dsl::user_id.eq(ctx.user_id()?),
                url_upvotes::dsl::url_id.eq(self.id()),
                url_upvotes::dsl::created_at.eq(ctx.now().naive_utc()),
            ))
            .execute(&*ctx.conn().await?)?;
        Ok(())
    }

    /// Rescind an upvote for the URL as the logged in user.
    pub async fn rescind_upvote(&self, ctx: &Context) -> Result<()> {
        let upvote = url_upvotes::table
            .filter(url_upvotes::dsl::url_id.eq(self.id()))
            .filter(url_upvotes::dsl::user_id.eq(ctx.user_id()?));
        diesel::delete(upvote).execute(&*ctx.conn().await?)?;
        Ok(())
    }
}
