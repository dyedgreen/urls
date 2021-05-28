use crate::db::models::{Invite, User};
use crate::schema::invites;
use crate::Context;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult, ID};
use juniper_relay::RelayConnection;

pub struct Viewer;

#[graphql_object(context = Context)]
impl Viewer {
    /// The ID of the logged in user, prefixed
    /// with `viewer-`, or `viewer-public` if not
    /// logged in.
    fn id(ctx: &Context) -> ID {
        ctx.maybe_user_id()
            .map(|id| format!("viewer-{}", id))
            .unwrap_or_else(|| "viewer-public".into())
            .into()
    }

    /// The currently logged in user, if any.
    async fn user(ctx: &Context) -> FieldResult<Option<User>> {
        Ok(ctx.maybe_user().await?)
    }

    /// Email address of the currently logged in user.
    async fn email(ctx: &Context) -> FieldResult<Option<String>> {
        // This field is on the viewer, since the email of other uses
        // should not be accessible without being logged in as that user.
        // By having it on the viewer, the graphql type system can enforce
        // that invariant.
        let email = ctx
            .maybe_user()
            .await?
            .and_then(|user| user.email().ok())
            .map(|address| address.to_string());
        Ok(email)
    }

    /// Invitations issued by the currently logged in user. If no
    /// user is logged in, the connection will be empty.
    async fn invites(
        ctx: &Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        claimed: Option<bool>,
    ) -> FieldResult<RelayConnection<Invite>> {
        if let Some(user_id) = ctx.maybe_user_id() {
            let conn = ctx.conn().await?;
            // TODO: We might want to move this to some other place ...
            let edges =
                RelayConnection::new(first, after, last, before, |after, before, limit| {
                    let mut query = invites::table
                        .filter(invites::dsl::created_by.eq(user_id))
                        .order_by(invites::dsl::created_at.desc())
                        .into_boxed();

                    if let Some(claimed) = claimed {
                        if claimed {
                            query = query.filter(invites::dsl::claimed_by.is_not_null());
                        } else {
                            query = query.filter(invites::dsl::claimed_by.is_null());
                        }
                    }

                    if let Some(after) = after {
                        let after: Invite = invites::table.find(after).get_result(&*conn)?;
                        query = query
                            .filter(invites::dsl::created_at.lt(after.created_at().naive_utc()));
                    }
                    if let Some(before) = before {
                        let before: Invite = invites::table.find(before).get_result(&*conn)?;
                        query = query
                            .filter(invites::dsl::created_at.lt(before.created_at().naive_utc()));
                    }
                    if let Some(limit) = limit {
                        query = query.limit(limit);
                    }

                    let edges = query.load(&*conn)?;
                    Ok(edges)
                })?;
            Ok(edges)
        } else {
            Ok(RelayConnection::empty())
        }
    }
}
