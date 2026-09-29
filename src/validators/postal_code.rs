//! Postal code validator with country-specific rules.
//!
//! Supported countries:
//! - `US` — 5-digit or ZIP+4 (e.g. `90210`, `90210-1234`)
//! - `UK` — UK postcode formats (e.g. `SW1A 1AA`, `M1 1AE`)
//! - `CA` — Canadian postal code (e.g. `K1A 0B1`)
//! - `DE` — German PLZ, 5 digits (e.g. `10115`)
//! - `AU` — Australian 4-digit code (e.g. `2000`)
//! - `KE` — Kenyan 5-digit code (e.g. `00100`)

use crate::{ValidationError, Validator};

/// Supported countries for postal code validation.
#[derive(Debug, Clone, PartialEq)]
pub enum Country {
    US,
    UK,
    CA,
    DE,
    AU,
    KE,
}

impl Country {
    fn code(&self) -> &'static str {
        match self {
            Country::US => "US",
            Country::UK => "UK",
            Country::CA => "CA",
            Country::DE => "DE",
            Country::AU => "AU",
            Country::KE => "KE",
        }
    }
}

/// Configuration for [`PostalCodeValidator`].
#[derive(Debug, Clone)]
pub struct PostalCodeConfig {
    /// Target country whose rules to apply.
    pub country: Country,
}

impl Default for PostalCodeConfig {
    fn default() -> Self {
        Self { country: Country::US }
    }
}

/// Validates postal codes for specific countries.
///
/// # Example
/// ```
/// use input_validation_engine::validators::postal_code::{PostalCodeValidator, PostalCodeConfig, Country};
/// use input_validation_engine::Validator;
///
/// let cfg = PostalCodeConfig { country: Country::US };
/// assert!(PostalCodeValidator::validate("90210", &cfg).is_ok());
/// assert!(PostalCodeValidator::validate("ABCDE", &cfg).is_err());
/// ```
pub struct PostalCodeValidator;

fn validate_us(input: &str) -> bool {
    // 12345 or 12345-6789
    let input = input.trim();
    if input.len() == 5 {
        input.chars().all(|c| c.is_ascii_digit())
    } else if input.len() == 10 {
        let (zip, ext) = input.split_at(5);
        zip.chars().all(|c| c.is_ascii_digit())
            && ext.starts_with('-')
            && ext[1..].chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

fn validate_uk(input: &str) -> bool {
    // UK postcodes: AN NAA, ANN NAA, AAN NAA, AANN NAA, ANA NAA, AANA NAA
    // Simplified: 2–4 chars outward + space + digit + 2 letters
    let s = input.trim().to_uppercase();
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 { return false; }
    let (out, inward) = (parts[0], parts[1]);
    // inward must be digit + two letters
    let inward_ok = inward.len() == 3
        && inward.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        && inward[1..].chars().all(|c| c.is_ascii_alphabetic());
    // outward: 2–4 chars
    let outward_ok = (2..=4).contains(&out.len())
        && out.chars().all(|c| c.is_ascii_alphanumeric());
    inward_ok && outward_ok
}

fn validate_ca(input: &str) -> bool {
    // A1A 1A1
    let s = input.trim().to_uppercase();
    let s = s.replace(' ', "");
    if s.len() != 6 { return false; }
    let chars: Vec<char> = s.chars().collect();
    chars[0].is_ascii_alphabetic()
        && chars[1].is_ascii_digit()
        && chars[2].is_ascii_alphabetic()
        && chars[3].is_ascii_digit()
        && chars[4].is_ascii_alphabetic()
        && chars[5].is_ascii_digit()
}

fn validate_numeric(input: &str, len: usize) -> bool {
    let s = input.trim();
    s.len() == len && s.chars().all(|c| c.is_ascii_digit())
}

impl Validator for PostalCodeValidator {
    type Config = PostalCodeConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        let valid = match config.country {
            Country::US => validate_us(input),
            Country::UK => validate_uk(input),
            Country::CA => validate_ca(input),
            Country::DE => validate_numeric(input, 5),
            Country::AU => validate_numeric(input, 4),
            Country::KE => validate_numeric(input, 5),
        };

        if valid {
            Ok(())
        } else {
            Err(ValidationError::InvalidPostalCodeFormat {
                country: config.country.code().to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    #[test]
    fn valid_us_5digit() {
        let cfg = PostalCodeConfig { country: Country::US };
        assert!(PostalCodeValidator::validate("90210", &cfg).is_ok());
    }
    #[test]
    fn valid_us_zip4() {
        let cfg = PostalCodeConfig { country: Country::US };
        assert!(PostalCodeValidator::validate("90210-1234", &cfg).is_ok());
    }
    #[test]
    fn invalid_us() {
        let cfg = PostalCodeConfig { country: Country::US };
        assert!(PostalCodeValidator::validate("ABCDE", &cfg).is_err());
    }
    #[test]
    fn valid_uk() {
        let cfg = PostalCodeConfig { country: Country::UK };
        assert!(PostalCodeValidator::validate("SW1A 1AA", &cfg).is_ok());
    }
    #[test]
    fn invalid_uk() {
        let cfg = PostalCodeConfig { country: Country::UK };
        assert!(PostalCodeValidator::validate("12345", &cfg).is_err());
    }
    #[test]
    fn valid_ca() {
        let cfg = PostalCodeConfig { country: Country::CA };
        assert!(PostalCodeValidator::validate("K1A 0B1", &cfg).is_ok());
    }
    #[test]
    fn valid_de() {
        let cfg = PostalCodeConfig { country: Country::DE };
        assert!(PostalCodeValidator::validate("10115", &cfg).is_ok());
    }
    #[test]
    fn valid_ke() {
        let cfg = PostalCodeConfig { country: Country::KE };
        assert!(PostalCodeValidator::validate("00100", &cfg).is_ok());
    }
    #[test]
    fn empty() {
        let cfg = PostalCodeConfig { country: Country::US };
        assert_eq!(PostalCodeValidator::validate("", &cfg), Err(ValidationError::EmptyInput));
    }
}
