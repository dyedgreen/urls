use crate::db::id::UserID;
use crate::db::models::User;
use crate::db::{Pool, PooledConnection, SearchIndex};
use crate::email::Mailer;
use crate::schema::users;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use once_cell::sync::Lazy;
use std::net::IpAddr;

const SERVER_XSRF_TOKEN: &str = "server_xsrt_token";
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    use ::std::time::Duration;

    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Urlsbot/0.1.0; +https://urls.fyi/bot.html)")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::limited(10))
        .gzip(true)
        .brotli(true)
        .build()
        .map_err(|err| log::error!("Failed to build http client: {}", err))
        .unwrap()
});

/// Application request context. The context holds information
/// about the current request, and also can provide access to
/// application level resources such as database handles.
///
/// The context can be extracted from any request, and used to
/// generate static pages. It is also available to most GraphQL
/// resolvers.
#[derive(Clone)]
pub struct Context {
    pool: Pool,
    mailer: Mailer,
    xsrf_token: String,
    login_session: Option<(UserID, String)>,
    request_time: DateTime<Utc>,
    user_agent: Option<String>,
    remote_ip: Option<IpAddr>,
}

impl Context {
    /// Create a new request context.
    pub fn for_request(
        pool: &Pool,
        mailer: &Mailer,
        xsrf_token: String,
        user_agent: Option<String>,
        remote_ip: Option<IpAddr>,
    ) -> Self {
        Self {
            pool: pool.clone(),
            mailer: mailer.clone(),
            xsrf_token,
            login_session: None,
            request_time: Utc::now(),
            user_agent,
            remote_ip,
        }
    }

    /// Create a new context for operations
    /// initiated by the server.
    pub fn for_server(pool: &Pool, mailer: &Mailer) -> Self {
        Self {
            pool: pool.clone(),
            mailer: mailer.clone(),
            xsrf_token: SERVER_XSRF_TOKEN.to_string(),
            login_session: None,
            request_time: Utc::now(),
            user_agent: None,
            remote_ip: None,
        }
    }

    /// Updates the user associated with this context.
    /// This exists to be used when constructing the
    /// context, and is probably not what you want.
    pub fn set_logged_in_user(&mut self, user: UserID, session_token: String) {
        self.login_session = Some((user, session_token));
    }

    /// Retrieve a database connection from the
    /// connection pool.
    pub async fn conn(&self) -> Result<PooledConnection<'_>> {
        Ok(self.pool.db.get().await?)
    }

    /// Retrieve a handle to the search index.
    pub fn search(&self) -> &SearchIndex {
        &self.pool.search
    }

    /// Retrieve the mailer to send an email
    /// message. Note that sending emails costs
    /// money.
    pub fn mailer(&self) -> &Mailer {
        &self.mailer
    }

    /// Retrieve an http client which can be used
    /// to make requests from the server. Requests
    /// employ a server-wide connection pool.
    pub fn http_client(&self) -> reqwest::Client {
        HTTP_CLIENT.clone()
    }

    /// Retrieve the ID of the logged in user.
    pub fn maybe_user_id(&self) -> Option<UserID> {
        self.login_session.as_ref().map(|(id, _)| *id)
    }

    /// Determine if the context has a logged in user.
    /// This is essentially an alias for
    /// `ctx.maybe_user_id().is_some()`
    pub fn is_logged_in(&self) -> bool {
        self.maybe_user_id().is_some()
    }

    /// Retrieve the ID of the logged in user
    /// as a `Result`. This is useful if you want
    /// to enforce a logged in user. If you do not
    /// want to force a logged in user, use
    /// [`maybe_user_id`](maybe_user_id).
    ///
    /// # Examples
    ///
    /// ```
    /// use server::Context;
    /// use anyhow::Result;
    ///
    /// fn some_handler(ctx: &Context) -> Result<()> {
    ///     // ...
    ///     let id = ctx.user_id()?;
    ///     // ...
    ///     Ok(())
    /// }
    /// ```
    pub fn user_id(&self) -> Result<UserID> {
        self.maybe_user_id().ok_or_else(|| anyhow!("Not logged in"))
    }

    /// Retrieve the logged in `User`. This requires
    /// a database query. If you only need the users
    /// ID, prefer [`maybe_user_id`](maybe_user_id).
    pub async fn maybe_user(&self) -> Result<Option<User>> {
        if let Some(id) = self.maybe_user_id() {
            let conn = self.conn().await?;
            let user = users::table.find(id).get_result(&*conn)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Retrieve the logged in user from this
    /// context. This is similar to [`user_id`](user_id),
    /// and is meant to force a logged in user.
    /// Also see [`maybe_user`](maybe_user).
    pub async fn user(&self) -> Result<User> {
        self.maybe_user()
            .await?
            .ok_or_else(|| anyhow!("Not logged in"))
    }

    /// Prefer this over `Utc::now()`, since it
    /// will remain consistent over the life-time
    /// of a given request.
    pub fn now(&self) -> DateTime<Utc> {
        self.request_time
    }

    /// Return the current login session
    /// token, if any.
    pub fn session_token(&self) -> Option<&str> {
        self.login_session.as_ref().map(|(_, token)| token.as_str())
    }

    /// Return the contexts XSRF token, e.g. to
    /// render it into a template.
    pub fn xsrf_token(&self) -> &str {
        &self.xsrf_token
    }

    /// Check if a given XSRF token is valid.
    pub fn check_xsrf_token(&self, token: &str) -> bool {
        self.xsrf_token == token
    }

    /// Return the user-agent of the request
    /// which created this context.
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Return the IP address of the remote
    /// request which created this context.
    pub fn remote_ip_address(&self) -> Option<IpAddr> {
        self.remote_ip
    }
}

impl juniper::Context for Context {}
