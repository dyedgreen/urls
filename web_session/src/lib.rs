use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac, NewMac};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// A session which can be serialized to and
/// de-serialized from bytes or base64. The
/// session checks verifies if the server authored
/// the original byte string.
#[derive(Debug, Serialize, Deserialize, std::cmp::PartialEq)]
pub struct Session<T> {
    value: T,
    expires: DateTime<Utc>,
}

impl<T: Serialize + DeserializeOwned> Session<T> {
    /// Constructs a new session.
    pub fn new(value: T, expires: DateTime<Utc>) -> Self {
        Self { value, expires }
    }

    /// Retrieves the value container by the
    /// session, if the session is not expired.
    pub fn value(self) -> Option<T> {
        if Utc::now() <= self.expires {
            Some(self.value)
        } else {
            None
        }
    }

    fn bytes(&self, key: &[u8]) -> Result<Vec<u8>, &'static str> {
        let sess_bytes = bincode::serialize(&self).map_err(|_| "failed to serialize session")?;
        let mut mac = HmacSha256::new_from_slice(key).expect("Sha256 accepts any size slice.");
        mac.update(&sess_bytes);
        let sign_bytes = mac.finalize().into_bytes();
        let session_pair = (sess_bytes.as_slice(), sign_bytes.as_slice());
        Ok(bincode::serialize(&session_pair).map_err(|_| "failed to serialize session pair")?)
    }

    fn from_bytes(bytes: &[u8], key: &[u8]) -> Result<Self, &'static str> {
        let (sess_bytes, sign_bytes) = bincode::deserialize::<(Vec<u8>, Vec<u8>)>(bytes)
            .map_err(|_| "failed to deserialize session pair")?;
        let mut mac = HmacSha256::new_from_slice(key).expect("Sha256 accepts any size slice.");
        mac.update(&sess_bytes);
        mac.verify(&sign_bytes)
            .map_err(|_| "invalid session signature")?;
        Ok(bincode::deserialize(&sess_bytes).map_err(|_| "failed to deserialize session")?)
    }

    /// Returns a signed base64 representation of the
    /// session.
    pub fn base64(&self, key: &[u8]) -> Result<String, &'static str> {
        Ok(base64::encode(&self.bytes(key)?))
    }

    /// Try to build a session from a base64 string. If the
    /// session can not be parsed of is not signed with the
    /// provided key, this will return an `Err`.
    pub fn from_base64(base64_bytes: &str, key: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::from_bytes(
            &base64::decode(base64_bytes).map_err(|_| "failed to decode base64 session bytes")?,
            key,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    const KEY: &[u8] = b"my super secret test key";

    #[test]
    fn basic_session() {
        let sess = Session::new("hello world".to_string(), Utc::now());
        let token: String = sess.base64(KEY).unwrap();
        let sess2: Session<String> = Session::from_base64(&token, KEY).unwrap();
        assert_eq!(sess, sess2);
    }

    #[test]
    fn basic_bad_session() {
        let sess = Session::new("hello world".to_string(), Utc::now());
        let token = sess.base64(KEY).unwrap();
        let sess2: Result<Session<String>, _> = Session::from_base64(&token[1..], KEY);
        assert!(sess2.is_err());
    }

    #[test]
    fn test_different_keys() {
        let sess = Session::new("hello world".to_string(), Utc::now());
        let token1 = sess.base64(b"hello world").unwrap();
        let token2 = sess.base64(b"hallo Welt").unwrap();
        assert_ne!(token1, token2);
    }

    #[test]
    fn session_expires() {
        let expired = Session::new("expired".to_string(), Utc::now() - Duration::minutes(1));
        let valid = Session::new("valid".to_string(), Utc::now() + Duration::minutes(1));

        assert_eq!(None, expired.value());
        assert_eq!(Some("valid".to_string()), valid.value());
    }
}
