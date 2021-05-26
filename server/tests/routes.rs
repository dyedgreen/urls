mod setup;

macro_rules! check_status {
    ($name:ident, $path:expr, $status:expr) => {
        check_status!($name, "GET", $path, $status);
    };
    ($name:ident, $method:expr, $path:expr, $status:expr) => {
        #[tokio::test(flavor = "multi_thread")]
        async fn $name() {
            let server = setup::mock().await;

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
check_status!(graphiql_page, "/graphql/playground", 200);
check_status!(not_found, "/404", 404);
