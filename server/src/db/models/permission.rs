use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use std::io::Write;

#[derive(AsExpression, FromSqlRow, Debug, Clone)]
#[sql_type = "Text"]
pub enum Permission {
    Administrator,
    Moderator,
}

impl<DB> ToSql<Text, DB> for Permission
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result {
        let t = match *self {
            Permission::Administrator => "administrator",
            Permission::Moderator => "moderator",
        };
        <str as ToSql<Text, DB>>::to_sql(t, out)
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
