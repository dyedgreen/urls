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

async fn generate_mock_users(ctx: &Context) {
    use db::models::{NewUserInput, Permission, Role, User};

    let admin = User::create(
        ctx,
        NewUserInput {
            name: "Test Administrator".into(),
            email: "test.admin@urls.fyi".into(),
        },
    )
    .await
    .unwrap();
    Role::create(ctx, admin.id(), Permission::Administrator)
        .await
        .unwrap();

    User::create(
        ctx,
        NewUserInput {
            name: "Test User".into(),
            email: "test.user@urls.fyi".into(),
        },
    )
    .await
    .unwrap();
}

/// Setup an isolated test environment with mock
/// data. This can be used for running end-to-end
/// tests on the server and API.
pub async fn mock() -> (
    impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone,
    email::Mailer,
) {
    set_work_dir();

    let test_conf = Config::test();
    let pool = db::connect(&test_conf)
        .await
        .expect("Failed to connect to test database");
    let mailer = email::connect(&test_conf)
        .await
        .expect("Failed to connect to test mailer");

    let ctx = Context::new(&pool, &mailer, "".into(), None);
    generate_mock_users(&ctx).await;

    (global_routes(&test_conf, pool, mailer.clone()), mailer)
}

/// Return the last sent email message.
#[allow(dead_code)]
pub async fn last_email(mailer: &email::Mailer) -> String {
    let path = match mailer.clone() {
        email::Mailer::File { last_message, .. } => last_message.lock().await.clone().unwrap(),
        _ => panic!("No email was sent"),
    };
    tokio::fs::read_to_string(path).await.unwrap()
}
