use super::{ERR, ID, SIZE};
use arrayvec::ArrayString;
use std::convert::{TryFrom, TryInto};
use std::{fmt, str::FromStr};

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

impl<const KIND: u64> TryFrom<[u8; SIZE]> for ID<KIND> {
    type Error = &'static str;

    fn try_from(value: [u8; SIZE]) -> Result<Self, Self::Error> {
        let inner = ArrayString::from_byte_string(&value).map_err(|_| ERR)?;
        if inner.len() == SIZE {
            Ok(Self(inner))
        } else {
            Err(ERR)
        }
    }
}

impl<const KIND: u64> TryFrom<&[u8]> for ID<KIND> {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map_err(|_| ERR)
            .and_then(TryFrom::<[u8; SIZE]>::try_from)
    }
}

impl<const KIND: u64> AsRef<str> for ID<KIND> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const KIND: u64> AsRef<[u8]> for ID<KIND> {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
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
