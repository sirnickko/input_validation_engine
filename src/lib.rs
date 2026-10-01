//! # Input Validation Engine
//!
//! A reusable, zero-dependency library for validating common input types.
//!
//! ## Validators available
//! - [`EmailValidator`] — RFC 5322-style email addresses
//! - [`PhoneValidator`] — E.164 international phone numbers
//! - [`UrlValidator`]   — HTTP/HTTPS/FTP URLs
//! - [`DateValidator`]  — ISO 8601 dates (YYYY-MM-DD)
//! - [`UuidValidator`]  — RFC 4122 UUIDs (v1–v5)
//! - [`CreditCardValidator`] — Luhn-checked card numbers
//! - [`IpValidator`]    — IPv4 and IPv6 addresses
//! - [`PostalCodeValidator`] — Country-specific postal codes
//! - [`UsernameValidator`]  — Configurable username rules
//! - [`PasswordValidator`]  — Configurable password strength
//!
//! ## Quick Example
//! ```rust
//! use input_validation_engine::validators::email::{EmailValidator, EmailConfig};
//! use input_validation_engine::Validator;
//!
//! let result = EmailValidator::validate("alice@example.com", &EmailConfig::default());
//! assert!(result.is_ok());
//! ```

pub mod error;
pub mod validators;

pub use error::ValidationError;

/// Core trait implemented by every validator.
///
/// Each validator defines its own `Config` type (with a `Default` impl)
/// so callers can use `Config::default()` for sensible defaults or customise
/// via the builder pattern.
pub trait Validator {
    /// Validator-specific configuration.
    type Config: Default;

    /// Validate `input` against the given `config`.
    ///
    /// Returns `Ok(())` if valid, or a descriptive [`ValidationError`] otherwise.
    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError>;
}

/// A validation error associated with a field name.
#[derive(Debug, PartialEq, Clone)]
pub struct FieldError {
    pub field: String,
    pub error: ValidationError,
}

/// Results from validating multiple fields without stopping at the first error.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ValidationReport {
    errors: Vec<FieldError>,
}

impl ValidationReport {
    /// Build a report from named validation results.
    pub fn from_results<I, N>(results: I) -> Self
    where
        I: IntoIterator<Item = (N, Result<(), ValidationError>)>,
        N: Into<String>,
    {
        let errors = results
            .into_iter()
            .filter_map(|(field, result)| {
                result.err().map(|error| FieldError {
                    field: field.into(),
                    error,
                })
            })
            .collect();

        Self { errors }
    }

    /// Whether every field passed validation.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// All validation errors, in input order.
    pub fn errors(&self) -> &[FieldError] {
        &self.errors
    }
}

/// Collect named validation results, retaining every failure.
pub fn validate_all<I, N>(results: I) -> ValidationReport
where
    I: IntoIterator<Item = (N, Result<(), ValidationError>)>,
    N: Into<String>,
{
    ValidationReport::from_results(results)
}

#[cfg(test)]
mod tests {
    use super::{validate_all, ValidationError};

    #[test]
    fn validate_all_keeps_every_named_error_in_order() {
        let report = validate_all([
            ("email", Err(ValidationError::MissingAtSign)),
            ("name", Ok(())),
            ("password", Err(ValidationError::MissingNumber)),
        ]);

        assert!(!report.is_valid());
        assert_eq!(report.errors().len(), 2);
        assert_eq!(report.errors()[0].field, "email");
        assert_eq!(report.errors()[1].field, "password");
    }

    #[test]
    fn validate_all_reports_valid_when_every_check_passes() {
        let report = validate_all([("email", Ok(())), ("password", Ok(()))]);

        assert!(report.is_valid());
        assert!(report.errors().is_empty());
    }
}
