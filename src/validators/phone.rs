//! Phone number validator (E.164 format).
//!
//! E.164 format: `+` followed by 7–15 digits (country code + subscriber number).
//! Example: `+254712345678` (Kenya), `+12025550173` (USA)

use crate::{ValidationError, Validator};

/// Configuration for [`PhoneValidator`].
#[derive(Debug, Clone)]
pub struct PhoneConfig {
    /// Allow spaces or hyphens as separators (they are stripped before validation).
    pub allow_separators: bool,
    /// Minimum number of digits (excluding `+`), default 7.
    pub min_digits: usize,
    /// Maximum number of digits (excluding `+`), default 15.
    pub max_digits: usize,
}

impl Default for PhoneConfig {
    fn default() -> Self {
        Self {
            allow_separators: true,
            min_digits: 7,
            max_digits: 15,
        }
    }
}

/// Validates international phone numbers in E.164 format.
///
/// # Example
/// ```
/// use input_validation_engine::validators::phone::{PhoneValidator, PhoneConfig};
/// use input_validation_engine::Validator;
///
/// assert!(PhoneValidator::validate("+12025550173", &PhoneConfig::default()).is_ok());
/// assert!(PhoneValidator::validate("12025550173",  &PhoneConfig::default()).is_err());
/// ```
pub struct PhoneValidator;

impl Validator for PhoneValidator {
    type Config = PhoneConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        if !input.starts_with('+') {
            return Err(ValidationError::MissingCountryCode);
        }

        // Strip leading '+', then optionally strip separators
        let digits_part = &input[1..];
        let cleaned: String = if config.allow_separators {
            digits_part
                .chars()
                .filter(|c| !matches!(c, ' ' | '-' | '(' | ')'))
                .collect()
        } else {
            digits_part.to_string()
        };

        // All remaining characters must be digits
        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidPhoneCharacters);
        }

        let digit_count = cleaned.len();
        if digit_count < config.min_digits || digit_count > config.max_digits {
            return Err(ValidationError::InvalidPhoneLength);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> PhoneConfig { PhoneConfig::default() }

    #[test]
    fn valid_us() {
        assert!(PhoneValidator::validate("+12025550173", &cfg()).is_ok());
    }
    #[test]
    fn valid_kenya() {
        assert!(PhoneValidator::validate("+254712345678", &cfg()).is_ok());
    }
    #[test]
    fn valid_with_spaces() {
        assert!(PhoneValidator::validate("+1 202 555 0173", &cfg()).is_ok());
    }
    #[test]
    fn valid_with_hyphens() {
        assert!(PhoneValidator::validate("+1-800-555-0100", &cfg()).is_ok());
    }
    #[test]
    fn missing_plus() {
        assert_eq!(PhoneValidator::validate("12025550173", &cfg()), Err(ValidationError::MissingCountryCode));
    }
    #[test]
    fn non_digit_chars() {
        assert_eq!(PhoneValidator::validate("+1abc5550173", &cfg()), Err(ValidationError::InvalidPhoneCharacters));
    }
    #[test]
    fn too_short() {
        assert_eq!(PhoneValidator::validate("+123", &cfg()), Err(ValidationError::InvalidPhoneLength));
    }
    #[test]
    fn too_long() {
        assert_eq!(PhoneValidator::validate("+1234567890123456", &cfg()), Err(ValidationError::InvalidPhoneLength));
    }
    #[test]
    fn empty() {
        assert_eq!(PhoneValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
}
