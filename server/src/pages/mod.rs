use crate::db::id::UserID;
use crate::{config::Config, db::Pool, Context};
use std::convert::Infallible;
use warp::Filter;
use web_session::Session;

pub mod error;
pub mod index;
pub mod xsrf;

const XSRF_COOKIE_NAME: &'static str = "xsrf";
const AUTH_COOKIE_NAME: &'static str = "session";

/// Captures a context from the given request. This never fails, and
/// thus should be used at the end of a filter chain to extract the context
/// only if the request will be processed by that filter.
pub fn context(conf: &Config, pool: Pool) -> impl ContextFilter {
    let conf = conf.clone();
    warp::cookie(AUTH_COOKIE_NAME)
        .map(move |token: String| {
            Session::<UserID>::from_base64(&token, conf.session_key())
                .ok()
                .and_then(|sess| sess.value())
        })
        .or(warp::any().map(|| None))
        .unify()
        .map(move |logged_in_user| Context::new(&pool, logged_in_user))
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
