use server::*;
use std::convert::Infallible;
use warp::{Filter, Reply};

/// Setup an isolated test environment with mock
/// data. This can be used for running end-to-end
/// tests on the server and API.
pub async fn mock() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let test_conf = config::Config::test();
    let pool = db::connect(&test_conf)
        .await
        .expect("Failed to connect to test database");
    // TODO: Might want to update working directory, such that templates and www are
    //       found correctly...
    global_routes(pool)
}
