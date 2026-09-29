//! Username validator with configurable rules.

use crate::{ValidationError, Validator};

/// Configuration for [`UsernameValidator`].
#[derive(Debug, Clone)]
pub struct UsernameConfig {
    /// Minimum length (default: 3).
    pub min_length: usize,
    /// Maximum length (default: 32).
    pub max_length: usize,
    /// Allow underscore `_` (default: `true`).
    pub allow_underscores: bool,
    /// Allow hyphen `-` (default: `true`).
    pub allow_hyphens: bool,
    /// Allow dot `.` (default: `false`).
    pub allow_dots: bool,
    /// Words that are forbidden (case-insensitive). Default: common reserved words.
    pub reserved: Vec<String>,
}

impl Default for UsernameConfig {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_length: 32,
            allow_underscores: true,
            allow_hyphens: true,
            allow_dots: false,
            reserved: vec![
                "admin".to_string(),
                "root".to_string(),
                "system".to_string(),
                "superuser".to_string(),
                "null".to_string(),
                "undefined".to_string(),
            ],
        }
    }
}

/// Validates usernames.
///
/// # Example
/// ```
/// use input_validation_engine::validators::username::{UsernameValidator, UsernameConfig};
/// use input_validation_engine::Validator;
///
/// assert!(UsernameValidator::validate("nick_99", &UsernameConfig::default()).is_ok());
/// assert!(UsernameValidator::validate("admin",   &UsernameConfig::default()).is_err());
/// ```
pub struct UsernameValidator;

impl Validator for UsernameValidator {
    type Config = UsernameConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }
        if input.len() < config.min_length {
            return Err(ValidationError::TooShort {
                min: config.min_length,
                actual: input.len(),
            });
        }
        if input.len() > config.max_length {
            return Err(ValidationError::TooLong {
                max: config.max_length,
                actual: input.len(),
            });
        }

        // Must start and end with alphanumeric
        let first = input.chars().next().unwrap();
        let last  = input.chars().last().unwrap();
        if !first.is_ascii_alphanumeric() {
            return Err(ValidationError::InvalidUsernameCharacter(first));
        }
        if !last.is_ascii_alphanumeric() {
            return Err(ValidationError::InvalidUsernameCharacter(last));
        }

        // Check each character
        for ch in input.chars() {
            let ok = ch.is_ascii_alphanumeric()
                || (config.allow_underscores && ch == '_')
                || (config.allow_hyphens    && ch == '-')
                || (config.allow_dots       && ch == '.');
            if !ok {
                return Err(ValidationError::InvalidUsernameCharacter(ch));
            }
        }

        // No consecutive special chars
        let special: &[char] = &['_', '-', '.'];
        let chars: Vec<char> = input.chars().collect();
        for window in chars.windows(2) {
            if special.contains(&window[0]) && special.contains(&window[1]) {
                return Err(ValidationError::InvalidFormat {
                    field: "username",
                    reason: "consecutive special characters are not allowed".to_string(),
                });
            }
        }

        // Reserved word check (case-insensitive)
        let lower = input.to_lowercase();
        for reserved in &config.reserved {
            if lower == reserved.to_lowercase() {
                return Err(ValidationError::ReservedUsername(input.to_string()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> UsernameConfig { UsernameConfig::default() }

    #[test]
    fn valid_simple() {
        assert!(UsernameValidator::validate("alice", &cfg()).is_ok());
    }
    #[test]
    fn valid_with_underscore() {
        assert!(UsernameValidator::validate("nick_99", &cfg()).is_ok());
    }
    #[test]
    fn valid_with_hyphen() {
        assert!(UsernameValidator::validate("john-doe", &cfg()).is_ok());
    }
    #[test]
    fn reserved_word() {
        assert_eq!(
            UsernameValidator::validate("admin", &cfg()),
            Err(ValidationError::ReservedUsername("admin".to_string()))
        );
    }
    #[test]
    fn reserved_case_insensitive() {
        assert!(UsernameValidator::validate("ADMIN", &cfg()).is_err());
    }
    #[test]
    fn too_short() {
        assert_eq!(
            UsernameValidator::validate("ab", &cfg()),
            Err(ValidationError::TooShort { min: 3, actual: 2 })
        );
    }
    #[test]
    fn invalid_special_char() {
        assert!(UsernameValidator::validate("nick@99", &cfg()).is_err());
    }
    #[test]
    fn consecutive_specials() {
        assert!(UsernameValidator::validate("nick__99", &cfg()).is_err());
    }
    #[test]
    fn starts_with_special() {
        assert!(UsernameValidator::validate("_nick", &cfg()).is_err());
    }
    #[test]
    fn ends_with_special() {
        assert!(UsernameValidator::validate("nick_", &cfg()).is_err());
    }
    #[test]
    fn empty() {
        assert_eq!(UsernameValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
}
