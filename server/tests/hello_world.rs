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
