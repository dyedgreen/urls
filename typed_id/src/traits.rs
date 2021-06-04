use super::{ERR, ID, SIZE};
use arrayvec::ArrayString;
use std::{
    convert::{TryFrom, TryInto},
    fmt, ops,
    str::FromStr,
};

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
