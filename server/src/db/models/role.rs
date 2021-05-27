use crate::db::id::{RoleID, UserID};
use crate::db::models::{Permission, User};
use crate::schema::roles;
use chrono::{DateTime, NaiveDateTime, Utc};

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

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }
}
