use warp::{filters::BoxedFilter, http::Uri, Filter, Reply};

pub fn filter() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .map(|| {
            let redirect = warp::redirect::temporary(Uri::from_static("/"));
            warp::reply::with_header(
                redirect,
                "Set-Cookie",
                format!("{}=; Path=/; Max-Age=0", super::AUTH_COOKIE_NAME),
            )
        })
        .boxed()
}
