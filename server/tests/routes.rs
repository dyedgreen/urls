mod setup;

macro_rules! check_status {
    ($name:ident, $path:expr, $status:expr) => {
        check_status!($name, "GET", $path, $status);
    };
    ($name:ident, $method:expr, $path:expr, $status:expr) => {
        #[tokio::test(flavor = "multi_thread")]
        async fn $name() {
            let (server, _) = setup::mock().await;

            let res = warp::test::request()
                .method($method)
                .path($path)
                .reply(&server)
                .await;
            assert_eq!(res.status(), $status);
        }
    };
}

check_status!(index_page, "/", 200);
check_status!(best_page, "/best", 200);
check_status!(recent_page, "/recent", 200);
check_status!(rss_feed, "/feed.xml", 200);
check_status!(search_page, "/search", 200);
check_status!(login_page, "/login", 200);
check_status!(logout_page, "/logout", 307);
check_status!(account_page, "/account", 307);
check_status!(graphiql_page, "/graphql/playground", 200);
check_status!(not_found, "/404", 404);

#[tokio::test(flavor = "multi_thread")]
async fn admin_backup_permissions() {
    let (server, ctx) = setup::mock().await;

    let res_no_user = warp::test::request()
        .path("/admin/backup")
        .reply(&server)
        .await;
    assert_eq!(res_no_user.status(), 404);

    let sess_user = setup::session_token(&ctx, "test.user@urls.fyi").await;
    let res_user = warp::test::request()
        .path("/admin/backup")
        .header("Cookie", format!("session={}", sess_user))
        .reply(&server)
        .await;
    assert_eq!(res_user.status(), 404);

    let sess_admin = setup::session_token(&ctx, "test.admin@urls.fyi").await;
    let res_admin_user = warp::test::request()
        .path("/admin/backup")
        .header("Cookie", format!("session={}", sess_admin))
        .reply(&server)
        .await;
    assert_eq!(res_admin_user.status(), 200);
}

#[tokio::test(flavor = "multi_thread")]
async fn server_side_cookies_are_set_correctly() {
    let (server, ctx) = setup::mock().await;

    let res_no_cookies = warp::test::request().path("/").reply(&server).await;
    assert_eq!(res_no_cookies.status(), 200);
    let cookies: Vec<&str> = res_no_cookies
        .headers()
        .get_all("Set-Cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect();
    assert_eq!(cookies.len(), 1);
    assert!(cookies[0].contains("xsrf="));

    let res_xsrf_only = warp::test::request()
        .path("/")
        .header("Cookie", "xsrf=test-xsrf")
        .reply(&server)
        .await;
    assert_eq!(res_xsrf_only.status(), 200);
    let cookies: Vec<&str> = res_xsrf_only
        .headers()
        .get_all("Set-Cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect();
    assert_eq!(cookies.len(), 1);
    assert!(cookies[0].contains("xsrf=test-xsrf"));

    let sess_user = setup::session_token(&ctx, "test.user@urls.fyi").await;
    let res_sess = warp::test::request()
        .path("/")
        .header("Cookie", format!("xsrf=test-xsrf; session={}", sess_user))
        .reply(&server)
        .await;
    assert_eq!(res_sess.status(), 200);
    let cookies: Vec<&str> = res_sess
        .headers()
        .get_all("Set-Cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect();
    assert_eq!(cookies.len(), 2);
    assert!(cookies.iter().any(|v| v.contains("xsrf=test-xsrf")));
    assert!(cookies
        .iter()
        .any(|v| v.contains(&format!("session={}", sess_user))));
}
