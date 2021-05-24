use anyhow::{anyhow, Result};
use nanoid::nanoid;
use std::convert::Infallible;
use warp::{Filter, Reply};

const XSRF_COOKIE_NAME: &'static str = "xsrf";

/// A filter which extracts the XSRF token
/// from the request.
pub fn token() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::cookie::optional(XSRF_COOKIE_NAME).map(|t: Option<String>| t.unwrap_or(nanoid!()))
}

/// Check if two xsrf tokens match. If not, this returns
/// an error which contains 403 `FORBIDDEN` rejection.
pub fn check(token_a: &str, token_b: &str) -> Result<()> {
    if token_a != token_b {
        Err(anyhow!("Invalid xsrf token"))
    } else {
        Ok(())
    }
}

/// This can be used to set the xsrf cookie with the correct extracted
/// cookie value.
pub fn cookie(reply: impl Reply, token: &str) -> impl Reply {
    warp::reply::with_header(
        reply,
        "Set-Cookie",
        format!("{}={}; Path=/; HttpOnly", XSRF_COOKIE_NAME, token),
    )
}
