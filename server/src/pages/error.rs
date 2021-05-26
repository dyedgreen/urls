use crate::templates;
use std::{convert::Infallible, fmt::Display};
use warp::http;
use warp::{reply::Response, Rejection, Reply};

#[derive(Debug, Clone, Copy)]
pub enum ServerError {
    Internal,
    NotFound,
    Request,
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
pub fn reply<R, E>(maybe_reply: Result<R, E>) -> Result<Response, Infallible>
where
    R: Reply,
    E: Into<ServerError>,
{
    maybe_reply
        .map(|reply| reply.into_response())
        .map_err(Into::into)
        .or_else(|error| {
            let status = match error {
                ServerError::Internal => http::StatusCode::INTERNAL_SERVER_ERROR,
                ServerError::Request => http::StatusCode::BAD_REQUEST,
                ServerError::NotFound => http::StatusCode::NOT_FOUND,
            };

            let mut ctx = tera::Context::new();
            ctx.insert("status", &status.as_u16());
            let page = templates::render("pages/error.html", &ctx);

            match page {
                Ok(body) => {
                    Ok(warp::reply::with_status(warp::reply::html(body), status).into_response())
                }
                Err(err) => {
                    log::error!("Failed to render error page: {}", err);
                    Ok(warp::reply::with_status(
                        "500 Internal server error",
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                    .into_response())
                }
            }
        })
}

/// Recover from a rejection with a rendered
/// error page.
pub async fn recover(rejection: Rejection) -> Result<Response, Infallible> {
    if rejection.is_not_found() {
        reply::<&str, _>(Err(ServerError::NotFound))
    } else {
        reply::<&str, _>(Err(ServerError::Request))
    }
}
