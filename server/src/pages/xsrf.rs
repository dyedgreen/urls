use crate::Context;
use nanoid::nanoid;
use std::convert::Infallible;
use warp::{http::HeaderValue, Filter, Reply};

/// A filter which extracts the XSRF token
/// from the request.
pub fn token() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::cookie::optional(super::XSRF_COOKIE_NAME)
        .map(|t: Option<String>| t.unwrap_or_else(|| nanoid!()))
}

/// This can be used to set the xsrf cookie with the correct extracted
/// cookie value.
pub fn cookie(ctx: &Context, reply: impl Reply) -> impl Reply {
    let mut resp = reply.into_response();
    let value = format!(
        "{}={}; Path=/; HttpOnly; SameSite=Strict",
        super::XSRF_COOKIE_NAME,
        ctx.xsrf_token(),
    );
    if let Ok(header) = HeaderValue::from_str(&value) {
        resp.headers_mut().append("Set-Cookie", header);
    }
    resp
}
