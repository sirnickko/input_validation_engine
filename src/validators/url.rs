//! URL validator.
//!
//! Checks that a URL has:
//! - A valid scheme (`http`, `https`, `ftp`)
//! - `://` separator
//! - A non-empty host
//! - Optionally: a path, query string, and/or fragment

use crate::{ValidationError, Validator};

/// Which URL schemes are accepted.
#[derive(Debug, Clone)]
pub enum AllowedSchemes {
    /// Only `http` and `https`.
    HttpOnly,
    /// `http`, `https`, and `ftp`.
    All,
    /// A custom list of allowed schemes (lowercase).
    Custom(Vec<String>),
}

/// Configuration for [`UrlValidator`].
#[derive(Debug, Clone)]
pub struct UrlConfig {
    /// Scheme allowlist (default: `http`, `https`, `ftp`).
    pub allowed_schemes: AllowedSchemes,
    /// Require the host to contain at least one `.` (default: `true`).
    pub require_dot_in_host: bool,
}

impl Default for UrlConfig {
    fn default() -> Self {
        Self {
            allowed_schemes: AllowedSchemes::All,
            require_dot_in_host: true,
        }
    }
}

/// Validates URLs.
///
/// # Example
/// ```
/// use input_validation_engine::validators::url::{UrlValidator, UrlConfig};
/// use input_validation_engine::Validator;
///
/// assert!(UrlValidator::validate("https://www.rust-lang.org/tools", &UrlConfig::default()).is_ok());
/// assert!(UrlValidator::validate("not-a-url", &UrlConfig::default()).is_err());
/// ```
pub struct UrlValidator;

impl Validator for UrlValidator {
    type Config = UrlConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Find "://"
        let scheme_end = input.find("://").ok_or(ValidationError::MissingScheme)?;
        let scheme = &input[..scheme_end].to_lowercase();

        // Validate scheme
        let allowed = match &config.allowed_schemes {
            AllowedSchemes::HttpOnly => vec!["http", "https"],
            AllowedSchemes::All      => vec!["http", "https", "ftp"],
            AllowedSchemes::Custom(v) => v.iter().map(|s| s.as_str()).collect(),
        };
        if !allowed.contains(&scheme.as_str()) {
            return Err(ValidationError::UnsupportedScheme(scheme.clone()));
        }

        // Everything after "://"
        let after_scheme = &input[scheme_end + 3..];

        // Host is everything up to '/', '?', '#', or end-of-string
        let host_end = after_scheme
            .find(|c| matches!(c, '/' | '?' | '#'))
            .unwrap_or(after_scheme.len());
        let host = &after_scheme[..host_end];

        if host.is_empty() {
            return Err(ValidationError::MissingHost);
        }
        if config.require_dot_in_host && !host.contains('.') && host != "localhost" {
            return Err(ValidationError::MissingHost);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> UrlConfig { UrlConfig::default() }

    #[test]
    fn valid_https() {
        assert!(UrlValidator::validate("https://www.rust-lang.org", &cfg()).is_ok());
    }
    #[test]
    fn valid_http_with_path() {
        assert!(UrlValidator::validate("http://example.com/some/path?q=1#anchor", &cfg()).is_ok());
    }
    #[test]
    fn valid_ftp() {
        assert!(UrlValidator::validate("ftp://files.example.com/file.txt", &cfg()).is_ok());
    }
    #[test]
    fn valid_localhost() {
        assert!(UrlValidator::validate("http://localhost/", &cfg()).is_ok());
    }
    #[test]
    fn missing_scheme() {
        assert_eq!(UrlValidator::validate("www.example.com", &cfg()), Err(ValidationError::MissingScheme));
    }
    #[test]
    fn unsupported_scheme() {
        assert!(matches!(
            UrlValidator::validate("ws://example.com", &cfg()),
            Err(ValidationError::UnsupportedScheme(_))
        ));
    }
    #[test]
    fn missing_host() {
        assert_eq!(UrlValidator::validate("https://", &cfg()), Err(ValidationError::MissingHost));
    }
    #[test]
    fn empty() {
        assert_eq!(UrlValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
}
