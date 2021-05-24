use std::convert::Infallible;
use warp::{Filter, Reply};

pub mod config;
pub mod db;
pub mod pages;
pub mod schema;

extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

/// Global routes for the app. These are separated out to enable
/// simple integration testing on the whole server without running
/// an actual web server.
pub fn global_routes(
    _pool: db::Pool,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    // let context = pages::context(db, mailer);

    // let index = context.clone().with(warp::wrap_fn(pages::index::page));
    // let index = warp::path::end().and(index).boxed();

    // let login = context.clone().with(warp::wrap_fn(pages::login::page));
    // let login = warp::path("login").and(login).boxed();

    // let repo = context.clone().with(warp::wrap_fn(pages::repository::page));
    // let repo = warp::path("project").and(repo).boxed();

    // let booking = context.with(warp::wrap_fn(pages::booking::page));
    // let booking = warp::path("booking").and(booking).boxed();

    let www = warp::fs::dir(config::ENV.www()).boxed();

    let routes = www;
    let server = routes
        .recover(pages::error::recover)
        .map(|reply| warp::reply::with_header(reply, "X-Frame-Options", "DENY"))
        .map(|reply| warp::reply::with_header(reply, "X-Content-Type-Options", "nosniff"))
        .map(|reply| warp::reply::with_header(reply, "Referrer-Policy", "no-referrer"))
        .with(warp::log("http"));

    server
}
