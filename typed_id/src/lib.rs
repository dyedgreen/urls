use arrayvec::ArrayString;
use nanoid::nanoid;
use std::cmp::{Eq, PartialEq};
use std::convert::{TryFrom, TryInto};
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
