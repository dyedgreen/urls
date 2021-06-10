use once_cell::sync::Lazy;
use std::collections::HashSet;

const DOMAINS_TXT: &str = include_str!("../hack/domains.txt");
const DOMAINS: Lazy<HashSet<&'static str>> = Lazy::new(|| DOMAINS_TXT.split('\n').collect());

/// Determine if this email address is
/// from a disposable email provider.
pub fn is_disposable(email: &str) -> bool {
    let host = email
        .rsplit_once('@')
        .map(|(_, host)| host)
        .unwrap_or(email);
    DOMAINS.contains(host)
}

/// Normalize email addresses, using a set
/// of known heuristics about common email
/// providers.
pub fn normalize(email: &str) -> String {
    let email = email.trim();
    match email.rsplit_once('@') {
        Some((user, "gmail.com")) => {
            // https://support.google.com/mail/answer/7436150
            let mut user_norm = user.replace('.', "");
            user_norm.make_ascii_lowercase();
            user_norm.push_str("@gmail.com");
            user_norm
        }
        Some(_) | None => email.to_ascii_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_smoke_test_disposable() {
        assert_eq!(is_disposable("fridraraho@memeil.top"), true);
        assert_eq!(is_disposable("peter.parker@gmail.com"), false);
    }

    #[test]
    fn basic_smoke_test_normalize() {
        assert_eq!(
            normalize("peter.parker@gmail.com"),
            normalize("PeterParker@gmail.com")
        );
    }
}
