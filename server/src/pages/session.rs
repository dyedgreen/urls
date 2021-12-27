use crate::Context;
use std::convert::Infallible;
use warp::{http::HeaderValue, Filter, Reply};

/// A filter which extracts the login / auth session token
/// from the request.
pub fn token() -> impl Filter<Extract = (Option<String>,), Error = Infallible> + Clone {
    warp::cookie::optional(super::AUTH_COOKIE_NAME)
}

/// This can be used to set the auth session cookie with the correct extracted
/// cookie value.
pub fn cookie(ctx: &Context, reply: impl Reply) -> impl Reply {
    let mut resp = reply.into_response();
    if let Some(session_token) = ctx.session_token() {
        let value = format!(
            "{}={}; Path=/; HttpOnly; SameSite=Strict; max-age=7776000",
            super::AUTH_COOKIE_NAME,
            session_token,
        );
        if let Ok(header) = HeaderValue::from_str(&value) {
            resp.headers_mut().append("Set-Cookie", header);
        }
    }
    resp
}
