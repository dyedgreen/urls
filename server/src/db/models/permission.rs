use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use juniper::GraphQLEnum;
use std::io::Write;

#[derive(GraphQLEnum, AsExpression, FromSqlRow, Debug, Clone, Copy)]
#[sql_type = "Text"]
pub enum Permission {
    Administrator,
    Moderator,
}

impl Permission {
    /// Determine if this permission grants the ability to
    /// create unlimited invite tokens.
    pub fn unlimited_invites(&self) -> bool {
        match *self {
            Permission::Administrator => true,
            Permission::Moderator => false,
        }
    }

    /// Determine if this permission grants the ability to
    /// create or delete user roles.
    pub fn modify_user_roles(&self) -> bool {
        match *self {
            Permission::Administrator => true,
            Permission::Moderator => false,
        }
    }
}

impl<DB> ToSql<Text, DB> for Permission
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result {
        let t = match *self {
            Permission::Administrator => "administrator",
            Permission::Moderator => "moderator",
        };
        t.to_sql(out)
    }
}

impl<DB> FromSql<Text, DB> for Permission
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "administrator" => Ok(Permission::Administrator),
            "moderator" => Ok(Permission::Moderator),
            _ => Err("Unrecognized permission".into()),
        }
    }
}
