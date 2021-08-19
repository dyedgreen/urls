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

    let res = request(query, json!(null), session).reply(&server).await;
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

    let res = request(query, vars, session).reply(&server).await;
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

    let res = request(query, json!(null), session).reply(&server).await;
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

#[tokio::test(flavor = "multi_thread")]
async fn test_invite_limit() {
    let (server, ctx) = setup::mock().await;
    let session = setup::session_token(&ctx, "test.user@urls.fyi").await;
    let session_admin = setup::session_token(&ctx, "test.admin@urls.fyi").await;

    let query = "
        mutation IssueInvite {
            issueInvite {
                createdBy {
                    name
                }
            }
        }
    ";

    // users have 3 invites
    for _ in 0..3 {
        let vars = json!({});
        let res = request(query, vars, &session).reply(&server).await;
        assert_eq!(res.status(), 200);

        let body: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(
            body,
            json!({
                "data": {
                    "issueInvite": {
                        "createdBy": { "name": "Test User" },
                    },
                },
            })
        );
    }

    let vars = json!({});
    let res = request(query, vars, &session).reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().contains_key("errors"));

    // admins have unlimited invites
    for _ in 0..10 {
        let vars = json!({});
        let res = request(query, vars, &session_admin).reply(&server).await;
        assert_eq!(res.status(), 200);

        let body: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(
            body,
            json!({
                "data": {
                    "issueInvite": {
                        "createdBy": { "name": "Test Administrator" },
                    },
                },
            })
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_grant_revoke_permissions() {
    let (server, ctx) = setup::mock().await;
    let session = setup::session_token(&ctx, "test.user@urls.fyi").await;
    let session_admin = setup::session_token(&ctx, "test.admin@urls.fyi").await;

    let query_grant = "
        mutation GrantPermission($permission: Permission!, $email: String!) {
            grantPermission(permission: $permission, email: $email) {
                permissions
            }
        }
    ";
    let query_revoke = "
        mutation GrantPermission($permission: Permission!, $email: String!) {
            revokePermission(permission: $permission, email: $email) {
                permissions
            }
        }
    ";
    let vars = json!({
        "permission": "MODERATOR",
        "email": "test.user@urls.fyi",
    });

    // only admins can grant permissions
    let res = request(query_grant, vars.clone(), &session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().contains_key("errors"));

    let res = request(query_grant, vars.clone(), &session_admin)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(
        body,
        json!({
            "data":{
                "grantPermission": {
                    "permissions": ["MODERATOR"],
                },
            },
        })
    );

    // only admins can revoke permissions
    let res = request(query_revoke, vars.clone(), &session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().contains_key("errors"));

    let res = request(query_revoke, vars.clone(), &session_admin)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(
        body,
        json!({
            "data":{
                "revokePermission": {
                    "permissions": [],
                },
            },
        })
    );
}
