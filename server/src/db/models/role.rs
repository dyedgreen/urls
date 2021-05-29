use crate::db::id::{RoleID, UserID};
use crate::db::models::{Permission, User};
use crate::schema::roles;
use crate::Context;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User)]
pub struct Role {
    id: RoleID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    user_id: UserID,
    permission: Permission,
}

impl Role {
    pub fn id(&self) -> RoleID {
        self.id
    }

    pub fn permission(&self) -> Permission {
        self.permission
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }
}

impl Role {
    /// Creates a new role for the given user and assigns the given
    /// permission.
    pub async fn create(ctx: &Context, user_id: UserID, permission: Permission) -> Result<Self> {
        let role = Self {
            id: RoleID::new(),
            created_at: ctx.now().naive_utc(),
            updated_at: ctx.now().naive_utc(),

            user_id,
            permission,
        };
        diesel::insert_into(roles::table)
            .values(&role)
            .execute(&*ctx.conn().await?)?;
        Ok(role)
    }

    /// Removes the given role from the database and revokes the
    /// users permission.
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        diesel::delete(self).execute(&*ctx.conn().await?)?;
        Ok(())
    }

    /// Deletes a role by permission for the given user.
    pub async fn delete_by_permission(
        ctx: &Context,
        user_id: UserID,
        permission: Permission,
    ) -> Result<Self> {
        let role: Self = roles::table
            .filter(roles::dsl::user_id.eq(user_id))
            .filter(roles::dsl::permission.eq(permission))
            .get_result(&*ctx.conn().await?)?;
        role.delete(ctx).await?;
        Ok(role)
    }
}
