use crate::{db::models::Login, db::Pool, email::Mailer, Context};
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use warp::Filter;

pub mod account;
pub mod admin;
pub mod comments;
pub mod error;
pub mod feed;
pub mod graphiql;
pub mod login;
pub mod logout;
pub mod register;
pub mod search;
pub mod session;
pub mod url_lists;
pub mod xsrf;

const XSRF_COOKIE_NAME: &str = "xsrf";
const AUTH_COOKIE_NAME: &str = "session";

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

    let user_agent = warp::header::optional::<String>("user-agent")
        .or(warp::any().map(|| None))
        .unify();
    let remote_address = warp::header("x-forwarded-for")
        .map(|fwd_for: String| {
            fwd_for
                .parse::<SocketAddr>()
                .map(|addr| addr.ip())
                .or_else(|_| fwd_for.parse::<IpAddr>())
                .ok()
        })
        .or(warp::addr::remote().map(|remote: Option<SocketAddr>| remote.map(|addr| addr.ip())))
        .unify();

    user_agent
        .and(remote_address)
        .and(session::token())
        .and(xsrf::token())
        .and_then(move |user_agent, remote_address, session, xsrf| {
            let ctx = Context::for_request(&pool, &mailer, xsrf, user_agent, remote_address);
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
