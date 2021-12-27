use crate::Context;
use askama::Template;
use std::{convert::Infallible, fmt::Display};
use warp::http;
use warp::{reply::Response, Rejection, Reply};

#[derive(Debug, Clone, Copy)]
pub enum ServerError {
    Internal,
    NotFound,
    Request,
}

#[derive(Template)]
#[template(path = "pages/error.html")]
struct ErrorPage {
    status: http::StatusCode,
}

impl<E> From<E> for ServerError
where
    E: Display,
{
    fn from(error: E) -> Self {
        log::warn!("Internal server error: {}", error);
        Self::Internal
    }
}

/// Map a general error to a 404 not found error. This bails
/// out and renders a generic 404 not found error page.
pub fn not_found(error: impl Display) -> ServerError {
    log::info!("Coercing to not found error: {}", error);
    ServerError::NotFound
}

/// Map a general error to a request error. This bails out
/// and renders a generic 400 bad request error page.
pub fn request(error: impl Display) -> ServerError {
    log::info!("Coercing to request error: {}", error);
    ServerError::Request
}

/// Turns a result into a reply. This is supposed to be used when
/// returning from a filter handler.
///
/// This also handles common headers that depend on the session, e.g.
/// XSRF token and refreshing the login session.
pub fn reply<R, E>(ctx: &Context, maybe_reply: Result<R, E>) -> Result<Response, Infallible>
where
    R: 'static + Reply,
    E: Into<ServerError>,
{
    maybe_reply
        .map(|reply| super::xsrf::cookie(ctx, reply))
        .map(|reply| super::session::cookie(ctx, reply))
        .map(|reply| reply.into_response())
        .map_err(Into::into)
        .or_else(|error| {
            let status = match error {
                ServerError::Internal => http::StatusCode::INTERNAL_SERVER_ERROR,
                ServerError::Request => http::StatusCode::BAD_REQUEST,
                ServerError::NotFound => http::StatusCode::NOT_FOUND,
            };
            let page = ErrorPage { status };
            Ok(warp::reply::with_status(page, status).into_response())
        })
}

/// Recover from a rejection with a rendered
/// error page.
pub async fn recover(rejection: Rejection) -> Result<Response, Infallible> {
    let status = if rejection.is_not_found() {
        http::StatusCode::NOT_FOUND
    } else {
        http::StatusCode::BAD_REQUEST
    };
    let page = ErrorPage { status };
    Ok(warp::reply::with_status(page, status).into_response())
}
