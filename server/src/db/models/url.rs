use crate::db::id::{UrlID, UserID};
use crate::db::models::{Comment, User};
use crate::schema::{comments, url_upvotes, urls, users};
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use form_urlencoded::Serializer;
use futures_util::StreamExt;
use juniper::GraphQLInputObject;
use meta_parser::Meta;
use std::convert::TryInto;
use std::str::FromStr;
use validator::Validate;
use warp::http::{uri::Scheme, StatusCode, Uri};

const INCLUDE_DAYS_IN_RANKED: i64 = 7;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
pub struct Url {
    id: UrlID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    url: String,
    status_code: i32,
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

    /// The status code which was returned when last
    /// trying to fetch the given URL.
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code.try_into().unwrap_or(500))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
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

    pub async fn comments(&self, ctx: &Context, limit: i64) -> Result<Vec<Comment>> {
        let comments = comments::table
            .filter(comments::dsl::url_id.eq(self.id))
            .order_by(comments::created_at.asc())
            .limit(limit)
            .select(comments::all_columns)
            .get_results(&*ctx.conn().await?)?;
        Ok(comments)
    }

    pub async fn comment_count(&self, ctx: &Context) -> Result<i64> {
        let count = comments::table
            .filter(comments::dsl::url_id.eq(self.id))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;
        Ok(count)
    }

    pub fn slug(&self) -> Option<String> {
        let slugify = |text: &str| {
            let words = text
                .split(|c: char| !c.is_alphanumeric())
                .filter(|word| !word.is_empty());
            let mut slug = String::new();
            for word in words {
                if slug.is_empty() {
                    slug.push_str(word);
                } else {
                    slug.push('-');
                    slug.push_str(word);
                }
            }
            slug.make_ascii_lowercase();
            slug
        };
        self.title.as_ref().map(|title| slugify(title)).or_else(|| {
            let url = self.url().ok()?;
            let authority = url.authority().map(|authority| slugify(authority.as_str()));
            let path = slugify(url.path());
            match authority {
                Some(authority) if !path.is_empty() => Some(format!("{authority}-{path}")),
                Some(authority) => Some(authority),
                None => {
                    if path.is_empty() {
                        None
                    } else {
                        Some(path)
                    }
                }
            }
        })
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

    /// Returns a list of URLs in reverse chronological order, in
    /// a way that's suitable for use with a Relay connection.
    pub async fn all_submissions(
        ctx: &Context,
        after: Option<UrlID>,
        before: Option<UrlID>,
        limit: Option<i64>,
    ) -> Result<Vec<Self>> {
        let conn = ctx.conn().await?;

        let mut query = urls::table
            .order_by(urls::dsl::created_at.desc())
            .into_boxed();

        if let Some(after) = after {
            let after: Url = urls::table.find(after).get_result(&*conn)?;
            query = query.filter(urls::dsl::created_at.lt(after.created_at().naive_utc()));
        }

        if let Some(before) = before {
            let before: Url = urls::table.find(before).get_result(&*conn)?;
            query = query.filter(urls::dsl::created_at.gt(before.created_at().naive_utc()));
        }

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        Ok(query.load(&*conn)?)
    }
}

impl Url {
    /// Normalizes a given Uri by removing known query
    /// parameters (e.g. those used for tracking).
    fn canonicalize(uri_str: &str) -> Result<Uri> {
        let uri = Uri::from_str(uri_str)?;
        let builder = Uri::builder()
            .scheme(uri.scheme().cloned().unwrap_or(Scheme::HTTPS))
            .authority(
                uri.authority()
                    .ok_or_else(|| anyhow!("Malformed URL"))?
                    .clone(),
            );

        let path_and_query = if let Some(raw) = uri.query() {
            let query = form_urlencoded::parse(raw.as_bytes())
                .filter(
                    |(name, _value)| match (uri.host().unwrap_or(""), name.as_ref()) {
                        // discard tracking parameters
                        (_, "utm_source" | "utm_medium" | "utm_campaign") => false,
                        (_, "utm_term" | "utm_content") => false,
                        // discard youtube time stamps
                        ("youtu.be" | "www.youtube.com", "t") => false,
                        // discard twitter share method tracking
                        ("twitter.com", "s") => false,
                        // keep everything else
                        (_, _) => true,
                    },
                )
                .fold(
                    Serializer::new(String::new()),
                    |mut builder, (name, value)| {
                        if value.is_empty() {
                            builder.append_key_only(name.as_ref());
                        } else {
                            builder.append_pair(name.as_ref(), value.as_ref());
                        }
                        builder
                    },
                )
                .finish();
            if query.is_empty() {
                uri.path().to_string()
            } else {
                format!("{}?{}", uri.path(), query)
            }
        } else {
            uri.path().to_string()
        };

        Ok(builder.path_and_query(path_and_query).build()?)
    }

    /// Creates a new role for the given user and assigns the given
    /// permission.
    pub async fn create(ctx: &Context, input: NewUrlInput, created_by: UserID) -> Result<Self> {
        input.validate()?;
        let NewUrlInput { url } = input;
        let url = Self::canonicalize(&url)?.to_string();

        // verify URL is unique, to avoid an additional query
        let exists: i64 = urls::table
            .filter(urls::dsl::url.eq(&url))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;
        if exists > 0 {
            return Err(anyhow!("The url was already submitted"));
        }

        let resp = ctx.http_client().get(&url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!("Failed to load url with status {}", status));
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
            status_code: status.as_u16().into(),
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

    /// Fetch the current contents of the URL and
    /// update the meta information and status code.
    pub async fn update_url_meta(&mut self, ctx: &Context) -> Result<()> {
        let resp = ctx.http_client().get(self.url.as_str()).send().await?;
        let status = resp.status();
        self.status_code = status.as_u16().into();
        self.updated_at = ctx.now().naive_utc();

        if status.is_success() {
            let mut meta = Meta::new();
            let mut stream = resp.bytes_stream();
            while let Some(part) = stream.next().await {
                meta.parse(&part?);
            }
            self.title = meta.title.or_else(|| self.title.clone());
            self.description = meta.description.or_else(|| self.description.clone());
            self.image = meta.image.or_else(|| self.image.clone());
        }

        *self = self.save_changes(&*ctx.conn().await?)?;
        Ok(())
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
        let comments = comments::table.filter(comments::dsl::url_id.eq(self.id));
        diesel::delete(upvotes).execute(&*conn)?;
        diesel::delete(comments).execute(&*conn)?;
        diesel::delete(self).execute(&*conn)?;
        ctx.search().delete_url(self)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_canonicalize() {
        let pairs = [
            ("https://urls.fyi/?utm_source=google&utm_campaign=test&allowed&other=test", "https://urls.fyi/?allowed&other=test"),
            ("https://urls.fyi/no-other-params?utm_medium=cpc&utm_content=textlink&utm_term=running+shoes", "https://urls.fyi/no-other-params"),
            ("https://urls.fyi/no-proto?other_test=&utm_medium=cpc&utm_content=text", "https://urls.fyi/no-proto?other_test"),
            ("https://www.youtube.com/watch?v=XXX&t=200s", "https://www.youtube.com/watch?v=XXX"),
            ("https://www.youtube.com/watch?v=XXX&t=200s", "https://www.youtube.com/watch?v=XXX"),
            ("https://youtu.be/YYY?t=200", "https://youtu.be/YYY"),
        ];
        for (raw, clean) in pairs {
            assert_eq!(Uri::from_static(clean), Url::canonicalize(raw).unwrap());
        }
    }

    #[test]
    fn test_slug() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd(2022, 10, 15),
            NaiveTime::from_hms(16, 5, 00),
        );
        let url = Url {
            id: UrlID::new(),
            created_at: date,
            updated_at: date,
            url: "https://urls.fyi/error/404".into(),
            status_code: 404,
            title: Some("404 :: Page Not Found".into()),
            description: None,
            image: None,
            created_by: UserID::new(),
        };
        assert_eq!(url.slug().unwrap(), "404-page-not-found");
        let url = Url { title: None, ..url };
        assert_eq!(url.slug().unwrap(), "urls-fyi-error-404");
        let url = Url {
            url: "https://tilman.dev/".into(),
            ..url
        };
        assert_eq!(url.slug().unwrap(), "tilman-dev");
    }
}
