use nanoid::nanoid;
use std::convert::Infallible;
use warp::{Filter, Reply};

/// A filter which extracts the XSRF token
/// from the request.
pub fn token() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::cookie::optional(super::XSRF_COOKIE_NAME).map(|t: Option<String>| t.unwrap_or(nanoid!()))
}

/// This can be used to set the xsrf cookie with the correct extracted
/// cookie value.
pub fn cookie(reply: impl Reply, token: &str) -> impl Reply {
    warp::reply::with_header(
        reply,
        "Set-Cookie",
        format!("{}={}; Path=/; HttpOnly", super::XSRF_COOKIE_NAME, token),
    )
}
