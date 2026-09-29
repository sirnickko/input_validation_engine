//! IP address validator — supports both IPv4 and IPv6.

use crate::{ValidationError, Validator};

/// Which IP versions are accepted.
#[derive(Debug, Clone, PartialEq)]
pub enum IpVersion {
    /// Only IPv4.
    V4,
    /// Only IPv6.
    V6,
    /// Either IPv4 or IPv6.
    Both,
}

/// Configuration for [`IpValidator`].
#[derive(Debug, Clone)]
pub struct IpConfig {
    /// Which version(s) to accept (default: `Both`).
    pub version: IpVersion,
}

impl Default for IpConfig {
    fn default() -> Self {
        Self { version: IpVersion::Both }
    }
}

/// Validates IPv4 and IPv6 addresses.
///
/// # Example
/// ```
/// use input_validation_engine::validators::ip_address::{IpValidator, IpConfig};
/// use input_validation_engine::Validator;
///
/// assert!(IpValidator::validate("192.168.1.1", &IpConfig::default()).is_ok());
/// assert!(IpValidator::validate("::1",         &IpConfig::default()).is_ok());
/// assert!(IpValidator::validate("999.0.0.1",   &IpConfig::default()).is_err());
/// ```
pub struct IpValidator;

fn validate_ipv4(input: &str) -> Result<(), ValidationError> {
    let octets: Vec<&str> = input.split('.').collect();
    if octets.len() != 4 {
        return Err(ValidationError::InvalidIPv4Format);
    }
    for octet in &octets {
        let val: u32 = octet.parse().map_err(|_| ValidationError::InvalidIPv4Octet)?;
        if val > 255 {
            return Err(ValidationError::InvalidIPv4Octet);
        }
        // Disallow leading zeros (e.g. "01") — ambiguous (octal in some contexts)
        if octet.len() > 1 && octet.starts_with('0') {
            return Err(ValidationError::InvalidIPv4Octet);
        }
    }
    Ok(())
}

fn validate_ipv6(input: &str) -> Result<(), ValidationError> {
    // Handle `::` (double colon = compressed zeros)
    let double_colon_count = input.matches("::").count();
    if double_colon_count > 1 {
        return Err(ValidationError::InvalidIPv6Format);
    }

    let (left, right) = if double_colon_count == 1 {
        let idx = input.find("::").unwrap();
        (&input[..idx], &input[idx + 2..])
    } else {
        (input, "")
    };

    let mut groups: Vec<&str> = Vec::new();
    if !left.is_empty()  { groups.extend(left.split(':'));  }
    if !right.is_empty() { groups.extend(right.split(':')); }

    // With `::` we may have fewer than 8 groups; without it exactly 8
    let max_groups = if double_colon_count == 1 { 7 } else { 8 };
    let min_groups = if double_colon_count == 1 { 0 } else { 8 };

    if groups.len() > max_groups || groups.len() < min_groups {
        return Err(ValidationError::InvalidIPv6Format);
    }

    for seg in &groups {
        if seg.len() > 4 || seg.is_empty() {
            return Err(ValidationError::InvalidIPv6Segment);
        }
        if !seg.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ValidationError::InvalidIPv6Segment);
        }
    }

    Ok(())
}

impl Validator for IpValidator {
    type Config = IpConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        let is_v4 = input.contains('.') && !input.contains(':');
        let is_v6 = input.contains(':');

        match (&config.version, is_v4, is_v6) {
            (IpVersion::V4, true, _)    => validate_ipv4(input),
            (IpVersion::V6, _, true)    => validate_ipv6(input),
            (IpVersion::Both, true, _)  => validate_ipv4(input),
            (IpVersion::Both, _, true)  => validate_ipv6(input),
            (IpVersion::V4, false, _)   => Err(ValidationError::InvalidIPv4Format),
            (IpVersion::V6, true, _)    => Err(ValidationError::InvalidIPv6Format),
            _                           => Err(ValidationError::InvalidFormat {
                field: "IP address",
                reason: "could not determine IP version".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> IpConfig { IpConfig::default() }

    // IPv4
    #[test]
    fn valid_ipv4() {
        assert!(IpValidator::validate("192.168.1.1", &cfg()).is_ok());
    }
    #[test]
    fn valid_ipv4_boundary() {
        assert!(IpValidator::validate("0.0.0.0", &cfg()).is_ok());
        assert!(IpValidator::validate("255.255.255.255", &cfg()).is_ok());
    }
    #[test]
    fn invalid_ipv4_octet() {
        assert_eq!(IpValidator::validate("256.0.0.1", &cfg()), Err(ValidationError::InvalidIPv4Octet));
    }
    #[test]
    fn invalid_ipv4_too_few() {
        assert_eq!(IpValidator::validate("192.168.1", &cfg()), Err(ValidationError::InvalidIPv4Format));
    }
    #[test]
    fn invalid_ipv4_leading_zero() {
        assert_eq!(IpValidator::validate("192.168.01.1", &cfg()), Err(ValidationError::InvalidIPv4Octet));
    }

    // IPv6
    #[test]
    fn valid_ipv6_full() {
        assert!(IpValidator::validate("2001:0db8:85a3:0000:0000:8a2e:0370:7334", &cfg()).is_ok());
    }
    #[test]
    fn valid_ipv6_compressed() {
        assert!(IpValidator::validate("::1", &cfg()).is_ok());
        assert!(IpValidator::validate("2001:db8::1", &cfg()).is_ok());
    }
    #[test]
    fn invalid_ipv6_double_double_colon() {
        assert_eq!(IpValidator::validate("::1::1", &cfg()), Err(ValidationError::InvalidIPv6Format));
    }
    #[test]
    fn empty() {
        assert_eq!(IpValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
}
