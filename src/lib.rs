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
