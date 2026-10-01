//! Password strength validator with configurable rules.

use crate::{ValidationError, Validator};

/// Configuration for [`PasswordValidator`].
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Minimum length (default: 8).
    pub min_length: usize,
    /// Maximum length (default: 128).
    pub max_length: usize,
    /// Require at least one uppercase ASCII letter (default: `true`).
    pub require_uppercase: bool,
    /// Require at least one lowercase ASCII letter (default: `true`).
    pub require_lowercase: bool,
    /// Require at least one ASCII digit (default: `true`).
    pub require_number: bool,
    /// Require at least one symbol / special character (default: `false`).
    pub require_symbol: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_number: true,
            require_symbol: false,
        }
    }
}

impl PasswordConfig {
    /// Create a strict config that also requires a symbol.
    pub fn strict() -> Self {
        Self { require_symbol: true, ..Self::default() }
    }
}

/// Validates password strength.
///
/// # Example
/// ```
/// use input_validation_engine::validators::password::{PasswordValidator, PasswordConfig};
/// use input_validation_engine::Validator;
///
/// assert!(PasswordValidator::validate("Secure1pass", &PasswordConfig::default()).is_ok());
/// assert!(PasswordValidator::validate("weak",        &PasswordConfig::default()).is_err());
/// ```
pub struct PasswordValidator;

const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~\"\\";

impl PasswordValidator {
    /// Return a heuristic strength score from 0 (weak) to 4 (strong).
    ///
    /// This score measures length and character variety; it does not check
    /// whether a password is common or has appeared in a data breach.
    pub fn score(input: &str) -> u8 {
        let length_score = match input.chars().count() {
            0..=7 => 0,
            8..=11 => 1,
            12..=15 => 2,
            _ => 3,
        };
        let has_uppercase = input.chars().any(|ch| ch.is_ascii_uppercase());
        let has_lowercase = input.chars().any(|ch| ch.is_ascii_lowercase());
        let has_number = input.chars().any(|ch| ch.is_ascii_digit());
        let has_symbol = input.chars().any(|ch| SYMBOLS.contains(ch));
        let variety_score = match [has_uppercase, has_lowercase, has_number, has_symbol]
            .into_iter()
            .filter(|present| *present)
            .count()
        {
            4 => 2,
            3 => 1,
            _ => 0,
        };

        (length_score + variety_score).min(4)
    }
}

impl Validator for PasswordValidator {
    type Config = PasswordConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        // Do NOT trim passwords — leading/trailing spaces can be intentional
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

        if config.require_uppercase && !input.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(ValidationError::MissingUppercase);
        }
        if config.require_lowercase && !input.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError::MissingLowercase);
        }
        if config.require_number && !input.chars().any(|c| c.is_ascii_digit()) {
            return Err(ValidationError::MissingNumber);
        }
        if config.require_symbol && !input.chars().any(|c| SYMBOLS.contains(c)) {
            return Err(ValidationError::MissingSymbol);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> PasswordConfig { PasswordConfig::default() }

    #[test]
    fn valid_password() {
        assert!(PasswordValidator::validate("Secure1pass", &cfg()).is_ok());
    }
    #[test]
    fn valid_with_symbol() {
        assert!(PasswordValidator::validate("Secure1pass!", &PasswordConfig::strict()).is_ok());
    }
    #[test]
    fn too_short() {
        assert_eq!(
            PasswordValidator::validate("Ab1", &cfg()),
            Err(ValidationError::TooShort { min: 8, actual: 3 })
        );
    }
    #[test]
    fn missing_uppercase() {
        assert_eq!(
            PasswordValidator::validate("secure1pass", &cfg()),
            Err(ValidationError::MissingUppercase)
        );
    }
    #[test]
    fn missing_lowercase() {
        assert_eq!(
            PasswordValidator::validate("SECURE1PASS", &cfg()),
            Err(ValidationError::MissingLowercase)
        );
    }
    #[test]
    fn missing_number() {
        assert_eq!(
            PasswordValidator::validate("Securepass", &cfg()),
            Err(ValidationError::MissingNumber)
        );
    }
    #[test]
    fn missing_symbol_when_required() {
        assert_eq!(
            PasswordValidator::validate("Secure1pass", &PasswordConfig::strict()),
            Err(ValidationError::MissingSymbol)
        );
    }
    #[test]
    fn empty() {
        assert_eq!(PasswordValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
    #[test]
    fn too_long() {
        let long = "A1a".repeat(50); // 150 chars > 128 max
        assert!(matches!(
            PasswordValidator::validate(&long, &cfg()),
            Err(ValidationError::TooLong { .. })
        ));
    }

    #[test]
    fn score_is_bounded_and_rewards_length_and_variety() {
        assert_eq!(PasswordValidator::score(""), 0);
        assert_eq!(PasswordValidator::score("password"), 1);
        assert_eq!(PasswordValidator::score("Secure1pass!"), 4);
        assert_eq!(PasswordValidator::score("A1a!".repeat(20).as_str()), 4);
    }
}
