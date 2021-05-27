use crate::{config::Config, db::Pool, email::Mailer, Context};
use std::convert::Infallible;
use warp::Filter;
use web_session::Session;

pub mod error;
pub mod graphiql;
pub mod index;
pub mod login;
pub mod logout;
pub mod xsrf;

const XSRF_COOKIE_NAME: &'static str = "xsrf";
const AUTH_COOKIE_NAME: &'static str = "session";

/// Captures a context from the given request. This never fails, and
/// thus should be used at the end of a filter chain to extract the context
/// only if the request will be processed by that filter.
pub fn context(pool: Pool, mailer: Mailer) -> impl ContextFilter {
    warp::cookie(AUTH_COOKIE_NAME)
        .map(|session: String| Some(session))
        .or(warp::any().map(|| None))
        .unify()
        .and(xsrf::token())
        .map(move |session: Option<String>, xsrf: String| {
            let logged_in_user = session
                .and_then(|sess| Session::from_base64(&sess, Config::env().session_key()).ok())
                .and_then(|sess| sess.value());
            Context::new(&pool, &mailer, xsrf, logged_in_user)
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
