use arrayvec::ArrayString;
use nanoid::nanoid;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;

mod traits;
mod traits_diesel;
mod traits_juniper;
mod traits_serde;

const SIZE: usize = 21;
const ERR: &str = "An ID must be exactly of size 21";

/// Typed random IDs.
///
/// IDs consist of ascii only, URL safe characters.
/// Since IDs are generic over a kind identifier,
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

    /// Return this IDs kind identifier.
    pub fn kind() -> u64 {
        KIND
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
