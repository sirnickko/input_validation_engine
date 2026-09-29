//! Credit card number validator.
//!
//! Uses the **Luhn algorithm** to verify the check digit,
//! then detects the card network from the IIN/BIN prefix.
//!
//! Supported networks: Visa, Mastercard, American Express, Discover, JCB,
//! Diners Club.

use crate::{ValidationError, Validator};

/// Detected card network.
#[derive(Debug, PartialEq, Clone)]
pub enum CardNetwork {
    Visa,
    Mastercard,
    Amex,
    Discover,
    Jcb,
    DinersClub,
}

/// Configuration for [`CreditCardValidator`].
#[derive(Debug, Clone)]
pub struct CreditCardConfig {
    /// Allow spaces and hyphens as grouping separators (stripped before checking).
    pub allow_separators: bool,
}

impl Default for CreditCardConfig {
    fn default() -> Self {
        Self { allow_separators: true }
    }
}

/// Validates credit card numbers via Luhn algorithm + network detection.
///
/// # Example
/// ```
/// use input_validation_engine::validators::credit_card::{CreditCardValidator, CreditCardConfig};
/// use input_validation_engine::Validator;
///
/// // Visa test number
/// assert!(CreditCardValidator::validate("4111111111111111", &CreditCardConfig::default()).is_ok());
/// assert!(CreditCardValidator::validate("1234567890123456", &CreditCardConfig::default()).is_err());
/// ```
pub struct CreditCardValidator;

/// Run the Luhn algorithm. Returns `true` if the number is valid.
fn luhn_check(digits: &[u8]) -> bool {
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            let mut n = d as u32;
            if i % 2 == 1 {
                n *= 2;
                if n > 9 { n -= 9; }
            }
            n
        })
        .sum();
    sum % 10 == 0
}

/// Detect the card network from the digit string.
fn detect_network(digits: &str) -> Option<CardNetwork> {
    let n = digits.len();
    let first  = &digits[..1];
    let first2 = if n >= 2 { &digits[..2] } else { "" };
    let first4 = if n >= 4 { &digits[..4] } else { "" };
    let first6 = if n >= 6 { &digits[..6] } else { "" };

    // American Express: starts with 34 or 37, length 15
    if (first2 == "34" || first2 == "37") && n == 15 {
        return Some(CardNetwork::Amex);
    }
    // Visa: starts with 4, length 13 or 16
    if first == "4" && (n == 13 || n == 16) {
        return Some(CardNetwork::Visa);
    }
    // Mastercard: starts with 51–55 or 2221–2720, length 16
    if n == 16 {
        if let Ok(f2) = first2.parse::<u32>() {
            if (51..=55).contains(&f2) {
                return Some(CardNetwork::Mastercard);
            }
        }
        if let Ok(f4) = first4.parse::<u32>() {
            if (2221..=2720).contains(&f4) {
                return Some(CardNetwork::Mastercard);
            }
        }
    }
    // Discover: starts with 6011, 622126–622925, 644–649, 65; length 16
    if n == 16 {
        if first4 == "6011" || first2 == "65" {
            return Some(CardNetwork::Discover);
        }
        if let Ok(f6) = first6.parse::<u32>() {
            if (622126..=622925).contains(&f6) {
                return Some(CardNetwork::Discover);
            }
        }
        if let Ok(f3) = digits[..3].parse::<u32>() {
            if (644..=649).contains(&f3) {
                return Some(CardNetwork::Discover);
            }
        }
    }
    // JCB: starts with 3528–3589, length 16
    if n == 16 {
        if let Ok(f4) = first4.parse::<u32>() {
            if (3528..=3589).contains(&f4) {
                return Some(CardNetwork::Jcb);
            }
        }
    }
    // Diners Club: starts with 300–305 or 36 or 38, length 14
    if n == 14 {
        if first2 == "36" || first2 == "38" {
            return Some(CardNetwork::DinersClub);
        }
        if let Ok(f3) = digits[..3].parse::<u32>() {
            if (300..=305).contains(&f3) {
                return Some(CardNetwork::DinersClub);
            }
        }
    }
    None
}

impl Validator for CreditCardValidator {
    type Config = CreditCardConfig;

    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Strip separators if allowed
        let cleaned: String = if config.allow_separators {
            input.chars().filter(|c| !matches!(c, ' ' | '-')).collect()
        } else {
            input.to_string()
        };

        // Must be all digits
        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidFormat {
                field: "credit card",
                reason: "must contain only digits".to_string(),
            });
        }

        // Parse to digit array
        let digits: Vec<u8> = cleaned.chars().map(|c| c as u8 - b'0').collect();

        // Detect network (also validates length)
        detect_network(&cleaned).ok_or(ValidationError::UnknownCardNetwork)?;

        // Luhn check
        if !luhn_check(&digits) {
            return Err(ValidationError::InvalidLuhnChecksum);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    fn cfg() -> CreditCardConfig { CreditCardConfig::default() }

    // Standard test numbers (all pass Luhn)
    #[test]
    fn valid_visa() {
        assert!(CreditCardValidator::validate("4111111111111111", &cfg()).is_ok());
    }
    #[test]
    fn valid_visa_spaces() {
        assert!(CreditCardValidator::validate("4111 1111 1111 1111", &cfg()).is_ok());
    }
    #[test]
    fn valid_mastercard() {
        assert!(CreditCardValidator::validate("5500005555555559", &cfg()).is_ok());
    }
    #[test]
    fn valid_amex() {
        assert!(CreditCardValidator::validate("378282246310005", &cfg()).is_ok());
    }
    #[test]
    fn valid_discover() {
        assert!(CreditCardValidator::validate("6011111111111117", &cfg()).is_ok());
    }
    #[test]
    fn invalid_luhn() {
        assert_eq!(
            CreditCardValidator::validate("4111111111111112", &cfg()),
            Err(ValidationError::InvalidLuhnChecksum)
        );
    }
    #[test]
    fn unknown_network() {
        assert_eq!(
            CreditCardValidator::validate("1234567890123456", &cfg()),
            Err(ValidationError::UnknownCardNetwork)
        );
    }
    #[test]
    fn non_digits() {
        assert!(CreditCardValidator::validate("4111-ABCD-1111-1111", &cfg()).is_err());
    }
    #[test]
    fn empty() {
        assert_eq!(CreditCardValidator::validate("", &cfg()), Err(ValidationError::EmptyInput));
    }
}
