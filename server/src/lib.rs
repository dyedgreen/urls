extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::convert::Infallible;
use warp::{Filter, Reply};

pub mod config;
pub mod context;
pub mod db;
pub mod email;
pub mod graphql;
pub mod pages;
pub mod schema;
pub mod templates;

pub use config::Config;
pub use context::Context;

/// Global routes for the app. These are separated out to enable
/// simple integration testing on the whole server without running
/// an actual web server.
pub fn global_routes(
    conf: &config::Config,
    pool: db::Pool,
    mailer: email::Mailer,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let ctx = pages::context(&conf, pool, mailer);

    let index = ctx.clone().with(warp::wrap_fn(pages::index::page));
    let index = warp::path::end().and(index).boxed();

    let api = ctx.clone().with(warp::wrap_fn(graphql::api));
    let api = warp::path("graphql").and(api).boxed();

    let graphiql = ctx.clone().with(warp::wrap_fn(pages::graphiql::page));
    let graphiql = warp::path!("graphql" / "playground").and(graphiql).boxed();

    let www = warp::fs::dir(conf.www().to_path_buf()).boxed();

    let routes = index.or(api).or(graphiql).or(www);
    let server = routes
        .recover(pages::error::recover)
        .map(|reply| warp::reply::with_header(reply, "X-Frame-Options", "DENY"))
        .map(|reply| warp::reply::with_header(reply, "X-Content-Type-Options", "nosniff"))
        .map(|reply| warp::reply::with_header(reply, "Referrer-Policy", "no-referrer"))
        .with(warp::log("http"));

    server
}
