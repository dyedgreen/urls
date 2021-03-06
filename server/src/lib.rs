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
pub mod jobs;
pub mod pages;
pub mod schema;
pub mod setup;

pub use config::Config;
pub use context::Context;

/// Global routes for the app. These are separated out to enable
/// simple integration testing on the whole server without running
/// an actual web server.
///
/// Note that some places use the global `Config.env()`, to access
/// configuration information, so some of the config values set in
/// e.g. a test config might not always be honored by the resulting
/// filter. (There is the aspiration to change this in the future.)
pub fn global_routes(
    conf: &config::Config,
    pool: db::Pool,
    mailer: email::Mailer,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let ctx = pages::context(pool, mailer);

    let index = ctx.clone().with(warp::wrap_fn(pages::url_lists::ranked));
    let index = warp::any().and(index);

    let recent = ctx.clone().with(warp::wrap_fn(pages::url_lists::recent));
    let recent = warp::path("recent").and(recent);

    let best = ctx.clone().with(warp::wrap_fn(pages::url_lists::best));
    let best = warp::path("best").and(best);

    let mine = ctx.clone().with(warp::wrap_fn(pages::url_lists::mine));
    let mine = warp::path("mine").and(mine);

    let user = ctx.clone().with(warp::wrap_fn(pages::url_lists::user));
    let user = warp::path("user").and(user);

    let feed = ctx.clone().with(warp::wrap_fn(pages::feed::page));

    let comments = ctx.clone().with(warp::wrap_fn(pages::comments::page));
    let comments = warp::path("comments").and(comments);

    let login = ctx.clone().with(warp::wrap_fn(pages::login::page));
    let login = warp::path("login").and(login);

    let register = ctx.clone().with(warp::wrap_fn(pages::register::page));
    let register = warp::path("register").and(register);

    let logout = warp::path("logout").and(pages::logout::filter());

    let account = ctx.clone().with(warp::wrap_fn(pages::account::page));
    let account = warp::path("account").and(account);

    let search = ctx.clone().with(warp::wrap_fn(pages::search::page));
    let search = warp::path("search").and(search);

    let admin = ctx.clone().with(warp::wrap_fn(pages::admin::backup));
    let admin = warp::path!("admin" / "backup").and(admin);

    let api = ctx.clone().with(warp::wrap_fn(graphql::api));
    let api = warp::path("graphql").and(api);

    let graphiql = ctx.with(warp::wrap_fn(pages::graphiql::page));
    let graphiql = warp::path!("graphql" / "playground").and(graphiql);

    let www = warp::fs::dir(conf.www().to_path_buf()).boxed();

    let routes = index
        .or(recent)
        .or(best)
        .or(mine)
        .or(user)
        .or(feed)
        .or(comments)
        .or(login)
        .or(register)
        .or(logout)
        .or(account)
        .or(search)
        .or(admin)
        .or(api)
        .or(graphiql)
        .or(www);

    routes
        .recover(pages::error::recover)
        .map(|reply| warp::reply::with_header(reply, "X-Frame-Options", "DENY"))
        .map(|reply| warp::reply::with_header(reply, "X-Content-Type-Options", "nosniff"))
        .map(|reply| warp::reply::with_header(reply, "Referrer-Policy", "no-referrer"))
        .with(warp::log("http"))
}
