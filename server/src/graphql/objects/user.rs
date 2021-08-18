use crate::db::id::UserID;
use crate::db::models::{Invite, Permission, Url, User};
use crate::schema::urls;
use crate::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use juniper_relay::RelayConnection;

#[graphql_object(context = Context)]
impl User {
    /// A globally unique identifier for this
    /// user.
    fn id(&self) -> UserID {
        self.id()
    }

    /// The display name of this user.
    fn name(&self) -> &str {
        self.name()
    }

    /// The date when this user account
    /// was created.
    fn joined(&self) -> DateTime<Utc> {
        self.created_at()
    }

    /// Invitation used by this user to register
    /// their account, if any.
    async fn invite(&self, ctx: &Context) -> FieldResult<Option<Invite>> {
        Ok(self.invite(ctx).await?)
    }

    /// Urls submitted by this user in reverse
    /// chronological order.
    async fn submissions(
        &self,
        ctx: &Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<RelayConnection<Url>> {
        let conn = ctx.conn().await?;
        RelayConnection::new(first, after, last, before, |after, before, limit| {
            let mut query = urls::table
                .filter(urls::dsl::created_by.eq(self.id()))
                .order_by(urls::dsl::created_at.desc())
                .into_boxed();

            if let Some(after) = after {
                let after: Url = urls::table.find(after).get_result(&*conn)?;
                query = query.filter(urls::dsl::created_at.lt(after.created_at().naive_utc()));
            }

            if let Some(before) = before {
                let before: Url = urls::table.find(before).get_result(&*conn)?;
                query = query.filter(urls::dsl::created_at.gt(before.created_at().naive_utc()));
            }

            if let Some(limit) = limit {
                query = query.limit(limit);
            }

            Ok(query.load(&*conn)?)
        })
    }

    /// List of active permissions for this
    /// user.
    async fn permissions(&self, ctx: &Context) -> FieldResult<Vec<Permission>> {
        Ok(self.permissions(ctx).await?)
    }
}
