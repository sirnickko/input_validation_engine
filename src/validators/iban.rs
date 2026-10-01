//! IBAN validator using country-specific lengths and the MOD-97 checksum.

use crate::{ValidationError, Validator};

/// Configuration for [`IbanValidator`].
#[derive(Debug, Clone, Default)]
pub struct IbanConfig;

/// Validates International Bank Account Numbers.
///
/// Spaces are ignored and lowercase letters are accepted. Validation checks
/// the country-specific IBAN length and checksum, but does not verify whether
/// the account exists.
///
/// # Example
/// ```
/// use input_validation_engine::validators::iban::{IbanConfig, IbanValidator};
/// use input_validation_engine::Validator;
///
/// assert!(IbanValidator::validate("GB82 WEST 1234 5698 7654 32", &IbanConfig).is_ok());
/// ```
pub struct IbanValidator;

impl Validator for IbanValidator {
    type Config = IbanConfig;

    fn validate(input: &str, _config: &Self::Config) -> Result<(), ValidationError> {
        let iban: String = input
            .chars()
            .filter(|ch| !ch.is_ascii_whitespace())
            .collect();
        if iban.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        let normalized = iban.to_ascii_uppercase();
        let bytes = normalized.as_bytes();
        if bytes.len() < 4
            || !bytes[0].is_ascii_alphabetic()
            || !bytes[1].is_ascii_alphabetic()
            || !bytes[2].is_ascii_digit()
            || !bytes[3].is_ascii_digit()
            || !bytes[4..].iter().all(u8::is_ascii_alphanumeric)
        {
            return Err(invalid_iban(
                "expected two letters, two checksum digits, and alphanumeric BBAN",
            ));
        }

        let expected_length = country_length(&normalized[..2])
            .ok_or_else(|| invalid_iban("unsupported country code"))?;
        if normalized.len() != expected_length {
            return Err(invalid_iban("length does not match the country format"));
        }

        if mod97(&normalized) != 1 {
            return Err(invalid_iban("checksum is invalid"));
        }

        Ok(())
    }
}

fn invalid_iban(reason: &str) -> ValidationError {
    ValidationError::InvalidFormat {
        field: "IBAN",
        reason: reason.to_owned(),
    }
}

fn country_length(country: &str) -> Option<usize> {
    Some(match country {
        "AD" => 24,
        "AL" => 28,
        "AT" => 20,
        "AZ" => 28,
        "BA" => 20,
        "BE" => 16,
        "BG" => 22,
        "BH" => 22,
        "BR" => 29,
        "BY" => 28,
        "CH" => 21,
        "CR" => 22,
        "CY" => 28,
        "CZ" => 24,
        "DE" => 22,
        "DK" => 18,
        "DO" => 28,
        "EE" => 20,
        "EG" => 29,
        "ES" => 24,
        "FI" => 18,
        "FO" => 18,
        "FR" => 27,
        "GB" => 22,
        "GE" => 22,
        "GI" => 23,
        "GL" => 18,
        "GR" => 27,
        "GT" => 28,
        "HR" => 21,
        "HU" => 28,
        "IE" => 22,
        "IL" => 23,
        "IQ" => 23,
        "IS" => 26,
        "IT" => 27,
        "JO" => 30,
        "KW" => 30,
        "KZ" => 20,
        "LB" => 28,
        "LC" => 32,
        "LI" => 21,
        "LT" => 20,
        "LU" => 20,
        "LV" => 21,
        "MC" => 27,
        "MD" => 24,
        "ME" => 22,
        "MK" => 19,
        "MR" => 27,
        "MT" => 31,
        "MU" => 30,
        "NL" => 18,
        "NO" => 15,
        "PK" => 24,
        "PL" => 28,
        "PS" => 29,
        "PT" => 25,
        "QA" => 29,
        "RO" => 24,
        "RS" => 22,
        "SA" => 24,
        "SC" => 31,
        "SE" => 24,
        "SI" => 19,
        "SK" => 24,
        "SM" => 27,
        "ST" => 25,
        "SV" => 28,
        "TL" => 23,
        "TN" => 24,
        "TR" => 26,
        "UA" => 29,
        "VA" => 22,
        "VG" => 24,
        "XK" => 20,
        _ => return None,
    })
}

fn mod97(iban: &str) -> u32 {
    let mut remainder = 0_u32;
    for ch in iban.chars().skip(4).chain(iban.chars().take(4)) {
        if ch.is_ascii_digit() {
            remainder = (remainder * 10 + ch.to_digit(10).unwrap()) % 97;
        } else {
            let value = (ch as u32) - ('A' as u32) + 10;
            remainder = (remainder * 100 + value) % 97;
        }
    }
    remainder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_known_iban_and_ignores_spaces_and_case() {
        assert!(IbanValidator::validate("GB82 WEST 1234 5698 7654 32", &IbanConfig).is_ok());
        assert!(IbanValidator::validate("gb82 west 1234 5698 7654 32", &IbanConfig).is_ok());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(matches!(
            IbanValidator::validate("GB81 WEST 1234 5698 7654 32", &IbanConfig),
            Err(ValidationError::InvalidFormat { field: "IBAN", .. })
        ));
    }

    #[test]
    fn rejects_invalid_country_length_and_characters() {
        assert!(IbanValidator::validate("GB82 WEST 123", &IbanConfig).is_err());
        assert!(IbanValidator::validate("GB82-WEST-1234-5698-7654-32", &IbanConfig).is_err());
        assert!(IbanValidator::validate("ZZ82 WEST 1234 5698 7654 32", &IbanConfig).is_err());
    }
}
