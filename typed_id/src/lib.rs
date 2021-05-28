use arrayvec::ArrayString;
use nanoid::nanoid;
use std::convert::{TryFrom, TryInto};
use std::{
    cmp::{Eq, PartialEq},
    str::FromStr,
};
use std::{fmt, ops};

const SIZE: usize = 21;
const ERR: &str = "An ID must be exactly of size 21";

/// Typed random IDs.
///
/// IDs consist of 16 ascii only characters. Since
/// IDs are generic over a kind identifier,
/// it's easy to define unique ID types
/// for your domain objects, without needing
/// macros or defining new types yourself.
///
/// # Examples
///
/// Declare IDs for your application in a central
/// location like this:
/// ```
/// use typed_id::ID;
///
/// type FooID = ID<0>;
/// type BarID = ID<1>;
/// // ...
/// ```
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ID<const KIND: u64>(ArrayString<SIZE>);

impl<const KIND: u64> ID<KIND> {
    /// Generate a new random ID.
    pub fn new() -> Self {
        Self::try_from(nanoid!(SIZE).as_str()).unwrap()
    }

    /// Return a reference to the underlying
    /// string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<const KIND: u64> TryFrom<&str> for ID<KIND> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let inner = ArrayString::from(value).map_err(|_| ERR)?;
        if inner.len() == SIZE {
            Ok(Self(inner))
        } else {
            Err(ERR)
        }
    }
}

impl<const KIND: u64> ops::Deref for ID<KIND> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl<const KIND: u64> AsRef<str> for ID<KIND> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const KIND: u64> FromStr for ID<KIND> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl<const KIND: u64> fmt::Display for ID<KIND> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const KIND: u64> serde::Serialize for ID<KIND> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de, const KIND: u64> serde::Deserialize<'de> for ID<KIND> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = ArrayString::deserialize(deserializer)?;
        if inner.len() == SIZE {
            Ok(Self(inner))
        } else {
            use serde::de::Error;
            Err(D::Error::invalid_length(SIZE, &ERR))
        }
    }
}

impl<const KIND: u64> diesel::expression::AsExpression<diesel::sql_types::Text> for ID<KIND> {
    type Expression = diesel::expression::bound::Bound<diesel::sql_types::Text, Self>;

    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(self)
    }
}

impl<const KIND: u64>
    diesel::expression::AsExpression<diesel::sql_types::Nullable<diesel::sql_types::Text>>
    for ID<KIND>
{
    type Expression = diesel::expression::bound::Bound<
        diesel::sql_types::Nullable<diesel::sql_types::Text>,
        Self,
    >;

    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(self)
    }
}

impl<const KIND: u64> diesel::expression::AsExpression<diesel::sql_types::Text> for &ID<KIND> {
    type Expression = diesel::expression::bound::Bound<diesel::sql_types::Text, Self>;

    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(self)
    }
}

impl<const KIND: u64>
    diesel::expression::AsExpression<diesel::sql_types::Nullable<diesel::sql_types::Text>>
    for &ID<KIND>
{
    type Expression = diesel::expression::bound::Bound<
        diesel::sql_types::Nullable<diesel::sql_types::Text>,
        Self,
    >;

    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(self)
    }
}

impl<DB, const KIND: u64> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ID<KIND>
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, const KIND: u64>
    diesel::serialize::ToSql<diesel::sql_types::Nullable<diesel::sql_types::Text>, DB> for ID<KIND>
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, const KIND: u64> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ID<KIND>
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(String::from_sql(bytes)?.as_str().try_into()?)
    }
}

impl<A, DB, const KIND: u64> diesel::deserialize::FromSqlRow<A, DB> for ID<KIND>
where
    DB: diesel::backend::Backend,
    ID<KIND>: diesel::deserialize::FromSql<A, DB>,
{
    fn build_from_row<R: diesel::row::Row<DB>>(row: &mut R) -> diesel::deserialize::Result<Self> {
        diesel::deserialize::FromSql::<A, DB>::from_sql(row.take())
    }
}

impl<T, DB, const KIND: u64> diesel::deserialize::Queryable<T, DB> for ID<KIND>
where
    DB: diesel::backend::Backend,
    Self: diesel::deserialize::FromSql<T, DB>,
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

impl<S: juniper::ScalarValue, const KIND: u64> juniper::GraphQLValue<S> for ID<KIND> {
    type Context = ();
    type TypeInfo = ();

    fn type_name<'i>(&self, _: &'i ()) -> Option<&'i str> {
        Some("ID")
    }

    fn resolve(
        &self,
        _: &(),
        _: Option<&[juniper::Selection<S>]>,
        _: &juniper::Executor<Self::Context, S>,
    ) -> juniper::ExecutionResult<S> {
        Ok(juniper::Value::scalar(self.to_string()))
    }
}

impl<S, const KIND: u64> juniper::GraphQLValueAsync<S> for ID<KIND>
where
    Self::TypeInfo: Sync,
    Self::Context: Sync,
    S: juniper::ScalarValue + Send + Sync,
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        selection_set: Option<&'a [juniper::Selection<S>]>,
        executor: &'a juniper::Executor<Self::Context, S>,
    ) -> juniper::BoxFuture<'a, juniper::ExecutionResult<S>> {
        use juniper::futures::future;
        let v = juniper::GraphQLValue::resolve(self, info, selection_set, executor);
        Box::pin(future::ready(v))
    }
}

impl<S: juniper::ScalarValue, const KIND: u64> juniper::GraphQLType<S> for ID<KIND> {
    fn name(_: &()) -> Option<&'static str> {
        Some("ID")
    }

    fn meta<'r>(_: &(), registry: &mut juniper::Registry<'r, S>) -> juniper::meta::MetaType<'r, S>
    where
        juniper::DefaultScalarValue: 'r,
    {
        registry.build_scalar_type::<ID<KIND>>(&()).into_meta()
    }
}

impl<S: juniper::ScalarValue, const KIND: u64> juniper::FromInputValue<S> for ID<KIND> {
    fn from_input_value(v: &juniper::InputValue<S>) -> Option<Self> {
        v.as_string_value().and_then(|s: &str| s.try_into().ok())
    }
}

impl<S: juniper::ScalarValue, const KIND: u64> juniper::ParseScalarValue<S> for ID<KIND> {
    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

impl<S: juniper::ScalarValue, const KIND: u64> juniper::marker::IsInputType<S> for ID<KIND> {}
impl<S: juniper::ScalarValue, const KIND: u64> juniper::marker::IsOutputType<S> for ID<KIND> {}

#[cfg(test)]
mod tests {
    use super::*;

    type TestID = ID<0>;

    #[test]
    fn generate_ids() {
        let a = TestID::new();
        let b = TestID::new();

        assert_ne!(a, b);
    }

    #[test]
    fn from_str() {
        let raw = nanoid!(SIZE);
        let big = nanoid!(100);
        let small = nanoid!(1);

        assert!(TestID::try_from(raw.as_str()).is_ok());
        assert!(TestID::try_from(big.as_str()).is_err());
        assert!(TestID::try_from(small.as_str()).is_err());
    }

    #[test]
    fn to_str() {
        let before = TestID::new();
        let raw = before.as_str().to_string();
        let after = TestID::try_from(raw.as_str()).unwrap();

        assert_eq!(before, after);
    }
}
