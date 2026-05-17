use serde::{Deserialize, Serialize};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(raw: impl Into<String>) -> Result<Self, DomainError> {
        let raw = raw.into().trim().to_lowercase();
        if raw.is_empty() {
            return Err(DomainError::Validation("email cannot be empty".into()));
        }
        if !raw.contains('@') || !raw.contains('.') {
            return Err(DomainError::Validation("invalid email format".into()));
        }
        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_email() {
        assert!(Email::parse("user@example.com").is_ok());
    }

    #[test]
    fn rejects_missing_at_sign() {
        assert!(Email::parse("invalid.com").is_err());
    }

    #[test]
    fn normalizes_case_and_whitespace() {
        let e = Email::parse("  User@Example.COM  ").unwrap();
        assert_eq!(e.as_str(), "user@example.com");
    }
}
