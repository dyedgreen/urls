use serde_json::{json, Value};
use warp::test::RequestBuilder;

mod setup;

fn request(query: &str, variables: Value, session: &str) -> RequestBuilder {
    let body: Value = json!({
        "query": query,
        "variables": variables,
    });
    warp::test::request()
        .path("/graphql")
        .method("POST")
        .header("Cookie", format!("xsrf=fake_xsrf; session={}", session))
        .header("X-XSRF-Token", "fake_xsrf")
        .header("Content-Type", "application/json")
        .body(body.to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_login() {
    let (server, mailer) = setup::mock().await;

    // Obtain a login email
    let query = "
        mutation RequestLogin($email: String!) {
            requestLogin(email: $email) {
                ok
            }
        }
    ";
    let vars = json!({
        "email": "test.user@urls.fyi",
    });

    let res = request(query, vars, "").reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    assert_eq!(
        body,
        json!({
            "data": {
                "requestLogin": { "ok": true },
            },
        })
    );

    let email = setup::last_email(&mailer).await;
    let token = email
        .split_whitespace()
        .find(|maybe_token| maybe_token.len() == 12)
        .expect("Email should contain a 12 character login token")
        .clone();

    // obtain a session from the emailed token
    let query = "
        mutation RequestLogin($email: String!, $token: String!) {
            login(email: $email, token: $token)
        }
    ";
    let vars = json!({
        "email": "test.user@urls.fyi",
        "token": token,
    });

    let res = request(query, vars, "").reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    assert_eq!(false, body.as_object().unwrap().contains_key("errors"));

    let session = body
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("login")
        .unwrap()
        .as_str()
        .unwrap();

    // test the session is valid and for the original user
    let query = "
        query IsLoggedIn {
            viewer {
                email
            }
        }
    ";

    let res = request(query, json!(null), session).reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    assert_eq!(
        body,
        json!({
            "data": {
                "viewer": { "email": "test.user@urls.fyi" },
            },
        })
    );
}
