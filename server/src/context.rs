use crate::db::id::UserID;
use crate::db::{Pool, PooledConnection};
use crate::email::Mailer;
use crate::pages::xsrf;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use warp::Reply;

/// Application request context. The context holds information
/// about the current request, and also can provide access to
/// application level resources such as database handles.
///
/// The context can be extracted from any request, and used to
/// generate static pages. It is also available to most GraphQL
/// resolvers.
#[derive(Debug, Clone)]
pub struct Context {
    pool: Pool,
    mailer: Mailer,
    xsrf_token: String,
    logged_in_user: Option<UserID>,
    request_time: DateTime<Utc>,
}

impl Context {
    /// Create a new request context.
    pub fn new(
        pool: &Pool,
        mailer: &Mailer,
        xsrf_token: String,
        logged_in_user: Option<UserID>,
    ) -> Self {
        Self {
            pool: pool.clone(),
            mailer: mailer.clone(),
            xsrf_token,
            logged_in_user,
            request_time: Utc::now(),
        }
    }

    /// Retrieve a database connection from the
    /// connection pool.
    pub async fn conn(&self) -> Result<PooledConnection<'_>> {
        Ok(self.pool.get().await?)
    }

    /// Retrieve the mailer to send an email
    /// message. Note that sending emails costs
    /// money.
    pub fn mailer(&self) -> &Mailer {
        &self.mailer
    }

    /// Prefer this over `Utc::now()`, since it
    /// will remain consistent over the life-time
    /// of a given request.
    pub fn now(&self) -> DateTime<Utc> {
        self.request_time
    }

    /// Check if a given XSRF token is valid.
    pub fn check_xsrf_token(&self, token: &str) -> bool {
        self.xsrf_token == token
    }

    /// Render the given template and turn the
    /// result into a warp response. The reply
    /// will also set some common cookies, such
    /// as the XSRF cookie.
    ///
    /// When rendering, the context will automatically
    /// populate the template context with common
    /// values. The full list of set values is:
    ///
    /// - `xsrf_token: String` cross site request forgery token
    /// - `logged_in: bool` determine if the viewer is logged in
    pub fn render<T>(&self, template: &str, data: Option<T>) -> Result<impl Reply>
    where
        T: Serialize,
    {
        let mut render_data = if let Some(data) = data {
            tera::Context::from_serialize(data)?
        } else {
            tera::Context::new()
        };
        render_data.insert("xsrf_token", &self.xsrf_token);
        render_data.insert("logged_in", &self.logged_in_user.is_some());

        let body = crate::templates::render(template, &render_data)?;
        let html = warp::reply::html(body);
        Ok(xsrf::cookie(html, &self.xsrf_token))
    }
}

impl juniper::Context for Context {}
