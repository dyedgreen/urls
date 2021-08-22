use serde_json::{json, Value};
mod setup;

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
    let res = setup::graphql(query_grant, vars.clone(), &session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().contains_key("errors"));

    let res = setup::graphql(query_grant, vars.clone(), &session_admin)
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
    let res = setup::graphql(query_revoke, vars.clone(), &session)
        .reply(&server)
        .await;
    assert_eq!(res.status(), 200);

    let body: Value = serde_json::from_slice(res.body()).unwrap();
    assert!(body.as_object().unwrap().contains_key("errors"));

    let res = setup::graphql(query_revoke, vars.clone(), &session_admin)
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
