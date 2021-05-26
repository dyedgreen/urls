use server::*;
use std::convert::Infallible;
use std::env;
use warp::{Filter, Reply};

fn set_work_dir() {
    let mut dir = env::current_dir().unwrap();
    if dir.ends_with("server") {
        dir.pop();
        env::set_current_dir(dir).unwrap();
    }
}

/// Setup an isolated test environment with mock
/// data. This can be used for running end-to-end
/// tests on the server and API.
pub async fn mock() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    set_work_dir();

    let test_conf = Config::test();
    let pool = db::connect(&test_conf)
        .await
        .expect("Failed to connect to test database");
    let mailer = email::connect(&test_conf)
        .await
        .expect("Failed to connect to test mailer");

    global_routes(&test_conf, pool, mailer)
}
