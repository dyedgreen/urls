use serde_json::{json, Value};
mod setup;

#[tokio::test(flavor = "multi_thread")]
async fn test_login() {
    let (server, ctx) = setup::mock().await;

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

    let res = setup::graphql(query, vars, "").reply(&server).await;
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

    let email = setup::last_email(&ctx).await;
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

    let res = setup::graphql(query, vars, "").reply(&server).await;
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

    let res = setup::graphql(query, json!(null), session)
        .reply(&server)
        .await;
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

    // test the session can be revoked
    let query = "
        query IsLoggedIn {
            viewer {
                logins {
                    nodes {
                        id
                    }
                }
            }
        }
    ";

    let res = setup::graphql(query, json!(null), session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    let login_id = body
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("viewer")
        .unwrap()
        .as_object()
        .unwrap()
        .get("logins")
        .unwrap()
        .as_object()
        .unwrap()
        .get("nodes")
        .unwrap()
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .as_object()
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // revoke the login
    let query = "
        mutation RevokeLogin($login: ID!) {
            revokeLogin(login: $login) {
                ok
            }
        }
    ";
    let vars = json!({
        "login": login_id,
    });

    let res = setup::graphql(query, vars, session).reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    assert_eq!(
        body,
        json!({
            "data": {
                "revokeLogin": { "ok": true },
            },
        })
    );

    // test the session was revoked
    let query = "
        query IsLoggedIn {
            viewer {
                email
            }
        }
    ";

    let res = setup::graphql(query, json!(null), session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).expect("Invalid JSON");
    assert_eq!(
        body,
        json!({
            "data": {
                "viewer": { "email": null },
            },
        })
    );
}
