use crate::db::id::UserID;
use crate::db::models::{Url, UrlOrdering, User};
use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use std::convert::TryInto;
use warp::{filters::BoxedFilter, http::Uri, reply::Response, Filter, Rejection, Reply};

const PAGE_SIZE: i64 = 10;

#[derive(Template)]
#[template(path = "pages/url_list.html")]
struct Page<'a> {
    title: &'a str,

    list_header: Option<ListHeader<'a>>,
    url_list: &'a [UrlPartial],

    page: u32,
    page_count: u32,

    is_logged_in: bool,
    xsrf_token: &'a str,
}

struct ListHeader<'a> {
    heading: &'a str,
    sub_heading: &'a str,
}

#[derive(Template)]
#[template(path = "partials/url.html")]
struct UrlPartial {
    url: Url,
    created_by: User,
    upvote_count: i64,
    is_upvoted_by_viewer: bool,
    comment_count: i64,
    is_logged_in: bool,
}

async fn handle(
    ctx: Context,
    order: UrlOrdering,
    page: u32,
    title: &str,
) -> Result<Response, error::ServerError> {
    let (urls, page_count) = Url::paginate(&ctx, order, page.into(), PAGE_SIZE).await?;

    let mut url_list = vec![];
    for url in urls {
        url_list.push(UrlPartial {
            created_by: url.created_by(&ctx).await?,
            upvote_count: url.upvote_count(&ctx).await?,
            is_upvoted_by_viewer: url.upvoted_by_viewer(&ctx).await?,
            comment_count: url.comment_count(&ctx).await?,
            url,
            is_logged_in: ctx.is_logged_in(),
        });
    }

    let user_heading;
    let list_header = match order {
        UrlOrdering::Ranked => None,
        UrlOrdering::Best => Some(ListHeader {
            heading: "Best",
            sub_heading: "All time best submissions",
        }),
        UrlOrdering::Recent => Some(ListHeader {
            heading: "Recent",
            sub_heading: "The most recent submissions",
        }),
        UrlOrdering::User(user_id) => {
            let user = User::find(&ctx, user_id).await?;
            user_heading = format!("By {}", user.name());
            Some(ListHeader {
                heading: &user_heading,
                sub_heading: "Recent submissions",
            })
        }
    };

    let page = Page {
        title,

        list_header,
        url_list: &url_list,

        page,
        page_count: page_count.try_into()?,

        is_logged_in: ctx.is_logged_in(),
        xsrf_token: ctx.xsrf_token(),
    };

    let resp = super::xsrf::cookie(page, ctx.xsrf_token());
    Ok(resp.into_response())
}

fn paginate() -> impl Filter<Extract = (u32,), Error = Rejection> + Clone + Copy {
    warp::path::end()
        .and(warp::any().map(|| 0))
        .or(warp::path!("page" / u32))
        .unify()
}

pub fn ranked(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::Ranked, page, "home").await)
        })
        .boxed()
}

pub fn best(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::Best, page, "best").await)
        })
        .boxed()
}

pub fn recent(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::Recent, page, "recent").await)
        })
        .boxed()
}

pub fn user(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::param()
        .and(paginate())
        .and(ctx)
        .and_then(|user_id: UserID, page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::User(user_id), page, "user").await)
        })
        .boxed()
}

pub fn mine(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            match ctx.maybe_user_id() {
                Some(user_id) => {
                    error::reply(handle(ctx, UrlOrdering::User(user_id), page, "mine").await)
                }
                None => Ok(warp::redirect::temporary(Uri::from_static("/login")).into_response()),
            }
        })
        .boxed()
}
