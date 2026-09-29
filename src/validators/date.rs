//! ISO 8601 date validator (YYYY-MM-DD).
//!
//! Rules:
//! - Format must be exactly `YYYY-MM-DD`
//! - Year: 1–9999
//! - Month: 01–12
//! - Day: correct for the given month, with leap-year awareness for February

use crate::{ValidationError, Validator};

/// Configuration for [`DateValidator`].
#[derive(Debug, Clone)]
pub struct DateConfig {
    /// Earliest allowed year (inclusive). Default: 1.
    pub min_year: u32,
    /// Latest allowed year (inclusive). Default: 9999.
    pub max_year: u32,
}

impl Default for DateConfig {
    fn default() -> Self {
        Self { min_year: 1, max_year: 9999 }
    }
}

/// Validates ISO 8601 dates.
///
/// # Example
/// ```
/// use input_validation_engine::validators::date::{DateValidator, DateConfig};
/// use input_validation_engine::Validator;
///
/// assert!(DateValidator::validate("2024-02-29", &DateConfig::default()).is_ok()); // leap year
/// assert!(DateValidator::validate("2023-02-29", &DateConfig::default()).is_err()); // not leap
/// ```
pub struct DateValidator;

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11               => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => 0,
    }
}

impl Validator for DateValidator {
    type Config = DateConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Expect exactly "NNNN-NN-NN"
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            return Err(ValidationError::InvalidFormat {
                field: "date",
                reason: "expected YYYY-MM-DD".to_string(),
            });
        }

        let year: u32 = parts[0].parse().map_err(|_| ValidationError::InvalidYear)?;
        let month: u8 = parts[1].parse().map_err(|_| ValidationError::InvalidMonth)?;
        let day: u8   = parts[2].parse().map_err(|_| ValidationError::InvalidDay)?;

        if year < config.min_year || year > config.max_year {
            return Err(ValidationError::InvalidYear);
        }
        if month < 1 || month > 12 {
            return Err(ValidationError::InvalidMonth);
        }

        let max_day = days_in_month(month, year);
        if day < 1 || day > max_day {
            // Distinguish leap-year Feb 29 specifically
            if month == 2 && day == 29 {
                return Err(ValidationError::NotALeapYear);
            }
            return Err(ValidationError::InvalidDay);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> DateConfig { DateConfig::default() }

    #[test]
    fn valid_regular() {
        assert!(DateValidator::validate("2024-06-15", &cfg()).is_ok());
    }
    #[test]
    fn valid_leap_feb29() {
        assert!(DateValidator::validate("2024-02-29", &cfg()).is_ok());
    }
    #[test]
    fn invalid_not_leap() {
        assert_eq!(DateValidator::validate("2023-02-29", &cfg()), Err(ValidationError::NotALeapYear));
    }
    #[test]
    fn invalid_month_13() {
        assert_eq!(DateValidator::validate("2024-13-01", &cfg()), Err(ValidationError::InvalidMonth));
    }
    #[test]
    fn invalid_day_31_april() {
        assert_eq!(DateValidator::validate("2024-04-31", &cfg()), Err(ValidationError::InvalidDay));
    }
    #[test]
    fn wrong_format() {
        assert!(DateValidator::validate("2024/06/15", &cfg()).is_err());
    }
    #[test]
    fn empty() {
        assert_eq!(DateValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
    #[test]
    fn valid_400_year_leap() {
        // 2000 is a leap year (divisible by 400)
        assert!(DateValidator::validate("2000-02-29", &cfg()).is_ok());
    }
    #[test]
    fn invalid_100_year_not_leap() {
        // 1900 is NOT a leap year (divisible by 100 but not 400)
        assert_eq!(DateValidator::validate("1900-02-29", &cfg()), Err(ValidationError::NotALeapYear));
    }
}
