use super::ID;
use diesel::deserialize::{FromSql, FromSqlRow, Queryable};
use diesel::expression::{bound::Bound, AsExpression};
use diesel::sql_types::{Nullable, Text};
use diesel::{backend::Backend, row::Row, serialize::ToSql};
use std::convert::TryInto;

impl<const KIND: u64> AsExpression<Text> for ID<KIND> {
    type Expression = Bound<Text, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<const KIND: u64> AsExpression<Nullable<Text>> for ID<KIND> {
    type Expression = Bound<Nullable<Text>, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<const KIND: u64> AsExpression<Text> for &ID<KIND> {
    type Expression = Bound<Text, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<const KIND: u64> AsExpression<Nullable<Text>> for &ID<KIND> {
    type Expression = Bound<Nullable<Text>, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<DB, const KIND: u64> ToSql<Text, DB> for ID<KIND>
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, const KIND: u64> ToSql<Nullable<Text>, DB> for ID<KIND>
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, const KIND: u64> FromSql<Text, DB> for ID<KIND>
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(String::from_sql(bytes)?.as_str().try_into()?)
    }
}

impl<A, DB, const KIND: u64> FromSqlRow<A, DB> for ID<KIND>
where
    DB: Backend,
    ID<KIND>: FromSql<A, DB>,
{
    fn build_from_row<R: Row<DB>>(row: &mut R) -> diesel::deserialize::Result<Self> {
        FromSql::<A, DB>::from_sql(row.take())
    }
}

impl<T, DB, const KIND: u64> Queryable<T, DB> for ID<KIND>
where
    DB: Backend,
    Self: FromSql<T, DB>,
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}
