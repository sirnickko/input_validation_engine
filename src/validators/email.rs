//! Email address validator (RFC 5322 – simplified).
//!
//! Checks structural correctness without making DNS calls.
//! Rules enforced:
//! - Must contain exactly one `@`
//! - Local part: 1–64 chars, alphanumeric + `.-_+`
//! - Domain: one or more labels separated by `.`, each label alphanumeric + `-`
//! - TLD: 2–63 alphabetic characters

use crate::{ValidationError, Validator};

/// Configuration for [`EmailValidator`].
#[derive(Debug, Clone)]
pub struct EmailConfig {
    /// Maximum total length of the email address (default 254 per RFC 5321).
    pub max_length: usize,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self { max_length: 254 }
    }
}

/// Validates email addresses.
///
/// # Example
/// ```
/// use input_validation_engine::validators::email::{EmailValidator, EmailConfig};
/// use input_validation_engine::Validator;
///
/// assert!(EmailValidator::validate("user@example.com", &EmailConfig::default()).is_ok());
/// assert!(EmailValidator::validate("not-an-email", &EmailConfig::default()).is_err());
/// ```
pub struct EmailValidator;

impl Validator for EmailValidator {
    type Config = EmailConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }
        if input.len() > config.max_length {
            return Err(ValidationError::TooLong {
                max: config.max_length,
                actual: input.len(),
            });
        }

        // Split on '@'
        let at_count = input.chars().filter(|&c| c == '@').count();
        if at_count == 0 {
            return Err(ValidationError::MissingAtSign);
        }
        if at_count > 1 {
            return Err(ValidationError::InvalidLocalPart);
        }

        let (local, domain) = input.split_once('@').unwrap();

        // --- local part ---
        if local.is_empty() || local.len() > 64 {
            return Err(ValidationError::InvalidLocalPart);
        }
        if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
            return Err(ValidationError::InvalidLocalPart);
        }
        for ch in local.chars() {
            if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '+' | '-') {
                return Err(ValidationError::InvalidLocalPart);
            }
        }

        // --- domain part ---
        if domain.is_empty() {
            return Err(ValidationError::InvalidDomain);
        }
        let labels: Vec<&str> = domain.split('.').collect();
        if labels.len() < 2 {
            return Err(ValidationError::InvalidDomain);
        }
        for label in &labels {
            if label.is_empty() || label.len() > 63 {
                return Err(ValidationError::InvalidDomain);
            }
            if label.starts_with('-') || label.ends_with('-') {
                return Err(ValidationError::InvalidDomain);
            }
            for ch in label.chars() {
                if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-') {
                    return Err(ValidationError::InvalidDomain);
                }
            }
        }

        // --- TLD ---
        let tld = labels.last().unwrap();
        if tld.len() < 2 || tld.len() > 63 {
            return Err(ValidationError::InvalidTLD);
        }
        if !tld.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::InvalidTLD);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> EmailConfig { EmailConfig::default() }

    // --- Valid ---
    #[test]
    fn valid_simple() {
        assert!(EmailValidator::validate("user@example.com", &cfg()).is_ok());
    }
    #[test]
    fn valid_plus_tag() {
        assert!(EmailValidator::validate("user+tag@example.com", &cfg()).is_ok());
    }
    #[test]
    fn valid_subdomain() {
        assert!(EmailValidator::validate("a@b.c.org", &cfg()).is_ok());
    }
    #[test]
    fn valid_hyphen_domain() {
        assert!(EmailValidator::validate("info@my-site.io", &cfg()).is_ok());
    }

    // --- Invalid ---
    #[test]
    fn missing_at() {
        assert_eq!(EmailValidator::validate("userexample.com", &cfg()), Err(ValidationError::MissingAtSign));
    }
    #[test]
    fn empty_local() {
        assert_eq!(EmailValidator::validate("@example.com", &cfg()), Err(ValidationError::InvalidLocalPart));
    }
    #[test]
    fn missing_tld() {
        assert_eq!(EmailValidator::validate("user@example", &cfg()), Err(ValidationError::InvalidDomain));
    }
    #[test]
    fn empty_input() {
        assert_eq!(EmailValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
    #[test]
    fn double_dot_local() {
        assert_eq!(EmailValidator::validate("us..er@example.com", &cfg()), Err(ValidationError::InvalidLocalPart));
    }
    #[test]
    fn numeric_tld() {
        assert!(EmailValidator::validate("a@b.123", &cfg()).is_err());
    }
}
