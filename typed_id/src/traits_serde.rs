use super::{ERR, ID, SIZE};
use arrayvec::ArrayString;

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
