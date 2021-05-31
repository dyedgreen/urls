use crate::db::id::{UrlID, UserID};
use crate::db::models::User;
use crate::schema::{urls, users};
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use futures_util::StreamExt;
use juniper::GraphQLInputObject;
use meta_parser::Meta;
use validator::Validate;
use warp::http::Uri;

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
    #[validate(url)]
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
        diesel::delete(self).execute(&*ctx.conn().await?)?;
        Ok(())
    }
}
