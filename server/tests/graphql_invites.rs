use serde_json::{json, Value};
mod setup;

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
        let res = setup::graphql(query, vars, &session).reply(&server).await;
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
    let res = setup::graphql(query, vars, &session).reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().get("data").unwrap().is_null());
    assert!(body.as_object().unwrap().contains_key("errors"));

    // admins have unlimited invites
    for _ in 0..10 {
        let vars = json!({});
        let res = setup::graphql(query, vars, &session_admin)
            .reply(&server)
            .await;
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
async fn test_create_account() {
    let (server, ctx) = setup::mock().await;
    let session = setup::session_token(&ctx, "test.user@urls.fyi").await;

    // issue an invite
    let query = "
        mutation IssueInvite {
            issueInvite {
                token
            }
        }
    ";

    let vars = json!({});
    let res = setup::graphql(query, vars, &session).reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    let token = body
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("issueInvite")
        .unwrap()
        .as_object()
        .unwrap()
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // create a new account using the invite
    let query = "
        mutation RegisterUser($name: String!, $email: String!, $token: String!) {
            registerUser(input: { name: $name, email: $email }, token: $token) {
                name
                permissions
                invite {
                    token
                }
            }
        }
    ";

    let vars = json!({
        "name": "Test Register User",
        "email": "test.register@urls.fyi",
        "token": token,
    });
    let res = setup::graphql(query, vars, "").reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(
        body,
        json!({
            "data": {
                "registerUser": {
                    "name": "Test Register User",
                    "permissions": [],
                    "invite": { "token": token }
                }
            }
        })
    );

    // invite can't be used twice
    let vars = json!({
        "name": "Test Register User Twice",
        "email": "test.register.twice@urls.fyi",
        "token": token,
    });
    let res = setup::graphql(query, vars, "").reply(&server).await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().get("data").unwrap().is_null());
    assert!(body.as_object().unwrap().contains_key("errors"));
}
