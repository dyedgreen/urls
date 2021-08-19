use crate::{db::models::Login, db::Pool, email::Mailer, Context};
use std::convert::Infallible;
use warp::Filter;

pub mod account;
pub mod admin;
pub mod comments;
pub mod error;
pub mod graphiql;
pub mod login;
pub mod logout;
pub mod register;
pub mod search;
pub mod url_lists;
pub mod xsrf;

const XSRF_COOKIE_NAME: &'static str = "xsrf";
const AUTH_COOKIE_NAME: &'static str = "session";

/// Captures a context from the given request. This never fails, and
/// thus should be used at the end of a filter chain to extract the context
/// only if the request will be processed by that filter.
pub fn context(pool: Pool, mailer: Mailer) -> impl ContextFilter {
    async fn attempt_login(
        mut ctx: Context,
        session: Option<String>,
    ) -> Result<Context, Infallible> {
        if let Some(session_token) = session {
            Login::use_session(&mut ctx, &session_token).await.ok();
        }
        Ok(ctx)
    }
    warp::cookie(AUTH_COOKIE_NAME)
        .map(|session: String| Some(session))
        .or(warp::any().map(|| None))
        .unify()
        .and(xsrf::token())
        .and_then(move |session: Option<String>, xsrf: String| {
            let ctx = Context::new(&pool, &mailer, xsrf, None);
            attempt_login(ctx, session)
        })
}

/// A shorthand for filters which can be used to extract a request context
pub trait ContextFilter:
    Filter<Extract = (Context,), Error = Infallible> + Clone + Send + Sync
{
}
impl<F> ContextFilter for F where
    F: Filter<Extract = (Context,), Error = Infallible> + Clone + Send + Sync
{
}
