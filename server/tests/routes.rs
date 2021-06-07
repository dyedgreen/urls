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
