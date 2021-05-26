mod setup;

#[tokio::test(flavor = "multi_thread")]
async fn index_page() {
    let server = setup::mock().await;

    let res = warp::test::request()
        .method("GET")
        .path("/")
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found() {
    let server = setup::mock().await;

    let res = warp::test::request()
        .method("GET")
        .path("/404")
        .reply(&server)
        .await;
    assert_eq!(res.status(), 404);
}

#[tokio::test(flavor = "multi_thread")]
async fn graphiql_page() {
    let server = setup::mock().await;

    let res = warp::test::request()
        .method("GET")
        .path("/graphql/playground")
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);
}
