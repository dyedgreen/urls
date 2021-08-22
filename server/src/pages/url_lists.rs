use crate::db::id::UserID;
use crate::db::models::{Url, UrlOrdering, User};
use crate::pages::{error, ContextFilter};
use crate::Context;
use askama::Template;
use std::convert::TryInto;
use warp::{filters::BoxedFilter, http::Uri, reply::Response, Filter, Rejection, Reply};

const PAGINATE_BOUNDS: i64 = 2;
const PAGE_SIZE: i64 = 10;

#[derive(Template)]
#[template(path = "pages/url_list.html")]
struct Page<'a> {
    title: &'a str,

    list_header: Option<ListHeader<'a>>,
    url_list: &'a [UrlPartial],

    pagination: PaginatePartial<'a>,

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

#[derive(Template)]
#[template(path = "partials/paginate.html")]
struct PaginatePartial<'a> {
    route: &'a str,
    page: u32,
    page_count: u32,
}

impl PaginatePartial<'_> {
    fn show_link(&self, idx: &u32) -> bool {
        let idx = *idx as i64;

        let start = (self.page as i64 - PAGINATE_BOUNDS).max(2);
        let end = (self.page as i64 + PAGINATE_BOUNDS).min(self.page_count as i64 - 3);

        let is_first = idx == 0;
        let is_last = idx + 1 == self.page_count as i64;

        let in_start_range = idx >= start && idx <= start + 2 * PAGINATE_BOUNDS;
        let in_end_range = idx >= end - 2 * PAGINATE_BOUNDS && idx <= end;

        let would_be_single_dots = if idx == 1 {
            self.show_link(&2)
        } else if idx + 2 == self.page_count as i64 && idx > 0 {
            self.show_link(&(idx as u32 - 1))
        } else {
            false
        };

        is_first || is_last || in_start_range || in_end_range || would_be_single_dots
    }

    fn show_dots(&self, idx: &u32) -> bool {
        if self.show_link(idx) {
            false
        } else {
            let idx = *idx;
            idx == 1 || idx + 2 == self.page_count
        }
    }
}

async fn handle(
    ctx: Context,
    order: UrlOrdering,
    page: u32,
    route: &str,
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

        pagination: PaginatePartial {
            route,
            page,
            page_count: page_count.try_into()?,
        },

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
            error::reply(handle(ctx, UrlOrdering::Ranked, page, "", "home").await)
        })
        .boxed()
}

pub fn best(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::Best, page, "/best", "best").await)
        })
        .boxed()
}

pub fn recent(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            error::reply(handle(ctx, UrlOrdering::Recent, page, "/recent", "recent").await)
        })
        .boxed()
}

pub fn user(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    warp::path::param()
        .and(paginate())
        .and(ctx)
        .and_then(|user_id: UserID, page: u32, ctx: Context| async move {
            error::reply(
                handle(
                    ctx,
                    UrlOrdering::User(user_id),
                    page,
                    &format!("/user/{}", user_id),
                    "user",
                )
                .await,
            )
        })
        .boxed()
}

pub fn mine(ctx: impl ContextFilter + 'static) -> BoxedFilter<(Response,)> {
    paginate()
        .and(ctx)
        .and_then(|page: u32, ctx: Context| async move {
            match ctx.maybe_user_id() {
                Some(user_id) => error::reply(
                    handle(ctx, UrlOrdering::User(user_id), page, "/mine", "mine").await,
                ),
                None => Ok(warp::redirect::temporary(Uri::from_static("/login")).into_response()),
            }
        })
        .boxed()
}
