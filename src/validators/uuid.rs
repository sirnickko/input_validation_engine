//! UUID validator (RFC 4122).
//!
//! Accepts both the standard hyphenated form `8-4-4-4-12`
//! and the compact 32-hex-char form.
//! Version nibble must be 1–5.

use crate::{ValidationError, Validator};

/// Configuration for [`UuidValidator`].
#[derive(Debug, Clone)]
pub struct UuidConfig {
    /// Also accept compact 32-char hex form (no hyphens). Default: `true`.
    pub allow_compact: bool,
    /// If `Some(v)`, only accept that specific version (1–5). Default: `None` (any).
    pub require_version: Option<u8>,
}

impl Default for UuidConfig {
    fn default() -> Self {
        Self { allow_compact: true, require_version: None }
    }
}

/// Validates RFC 4122 UUIDs.
///
/// # Example
/// ```
/// use input_validation_engine::validators::uuid::{UuidValidator, UuidConfig};
/// use input_validation_engine::Validator;
///
/// assert!(UuidValidator::validate("550e8400-e29b-41d4-a716-446655440000", &UuidConfig::default()).is_ok());
/// assert!(UuidValidator::validate("not-a-uuid", &UuidConfig::default()).is_err());
/// ```
pub struct UuidValidator;

fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

impl Validator for UuidValidator {
    type Config = UuidConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Normalise to 32-char hex for further checks
        let hex: String = if input.contains('-') {
            // Expect 8-4-4-4-12
            let parts: Vec<&str> = input.split('-').collect();
            if parts.len() != 5
                || parts[0].len() != 8
                || parts[1].len() != 4
                || parts[2].len() != 4
                || parts[3].len() != 4
                || parts[4].len() != 12
            {
                return Err(ValidationError::InvalidUuidFormat);
            }
            let joined = parts.concat();
            if !is_hex(&joined) {
                return Err(ValidationError::InvalidUuidFormat);
            }
            joined
        } else if input.len() == 32 && is_hex(&input) {
            if !config.allow_compact {
                return Err(ValidationError::InvalidUuidFormat);
            }
            input.clone()
        } else {
            return Err(ValidationError::InvalidUuidFormat);
        };

        // Version nibble is the 13th hex char (index 12)
        let version_char = hex.chars().nth(12).unwrap();
        let version = version_char.to_digit(16).unwrap() as u8;

        if version < 1 || version > 5 {
            return Err(ValidationError::InvalidUuidVersion);
        }

        if let Some(req) = config.require_version {
            if version != req {
                return Err(ValidationError::InvalidUuidVersion);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> UuidConfig { UuidConfig::default() }

    #[test]
    fn valid_v4_hyphenated() {
        assert!(UuidValidator::validate("550e8400-e29b-41d4-a716-446655440000", &cfg()).is_ok());
    }
    #[test]
    fn valid_v1() {
        assert!(UuidValidator::validate("6ba7b810-9dad-11d1-80b4-00c04fd430c8", &cfg()).is_ok());
    }
    #[test]
    fn valid_compact() {
        assert!(UuidValidator::validate("550e8400e29b41d4a716446655440000", &cfg()).is_ok());
    }
    #[test]
    fn compact_rejected_when_disabled() {
        let cfg = UuidConfig { allow_compact: false, ..UuidConfig::default() };
        assert_eq!(
            UuidValidator::validate("550e8400e29b41d4a716446655440000", &cfg),
            Err(ValidationError::InvalidUuidFormat)
        );
    }
    #[test]
    fn invalid_format() {
        assert_eq!(UuidValidator::validate("not-a-uuid", &cfg()), Err(ValidationError::InvalidUuidFormat));
    }
    #[test]
    fn invalid_version_0() {
        // Version nibble = 0 (position 12)
        assert_eq!(
            UuidValidator::validate("550e8400-e29b-01d4-a716-446655440000", &cfg()),
            Err(ValidationError::InvalidUuidVersion)
        );
    }
    #[test]
    fn empty() {
        assert_eq!(UuidValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
    #[test]
    fn wrong_group_lengths() {
        assert_eq!(
            UuidValidator::validate("550e840-e29b-41d4-a716-446655440000", &cfg()),
            Err(ValidationError::InvalidUuidFormat)
        );
    }
}
