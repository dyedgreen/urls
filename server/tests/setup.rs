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
    Context,
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

    (global_routes(&test_conf, pool, mailer.clone()), ctx)
}

/// Return the last sent email message.
#[allow(dead_code)]
pub async fn last_email(ctx: &Context) -> String {
    let path = match ctx.mailer().clone() {
        email::Mailer::File { last_message, .. } => last_message.lock().await.clone().unwrap(),
        _ => panic!("No email was sent"),
    };
    tokio::fs::read_to_string(path).await.unwrap()
}

/// Return a valid session token for the given user email.
#[allow(dead_code)]
pub async fn session_token(ctx: &Context, email: &str) -> String {
    let user = db::models::User::find_by_email(ctx, email)
        .await
        .expect("Missing user");
    let mut login = db::models::Login::create(ctx, user.id()).await.unwrap();
    let email_token = login.email_token().to_string();
    login.claim(ctx, &email_token).await.unwrap()
}
