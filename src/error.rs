//! Structured validation errors with human-readable messages.

use std::fmt;

/// Describes exactly why an input failed validation.
///
/// Each variant carries enough context for the caller to generate a
/// precise, user-facing error message.
#[derive(Debug, PartialEq, Clone)]
pub enum ValidationError {
    // ── Generic ──────────────────────────────────────────────────────────────
    /// Input was an empty string or contained only whitespace.
    EmptyInput,
    /// Input is shorter than the minimum allowed length.
    TooShort { min: usize, actual: usize },
    /// Input is longer than the maximum allowed length.
    TooLong { max: usize, actual: usize },
    /// General format error with a field name and reason.
    InvalidFormat { field: &'static str, reason: String },

    // ── Email ─────────────────────────────────────────────────────────────────
    /// The `@` sign is missing.
    MissingAtSign,
    /// The local part (before `@`) is invalid.
    InvalidLocalPart,
    /// The domain part (after `@`) is invalid.
    InvalidDomain,
    /// The top-level domain is missing or invalid.
    InvalidTLD,

    // ── Phone ─────────────────────────────────────────────────────────────────
    /// Phone number doesn't start with `+` (E.164 required).
    MissingCountryCode,
    /// Phone number contains non-digit characters after the `+`.
    InvalidPhoneCharacters,
    /// Phone number is outside the valid digit count range.
    InvalidPhoneLength,

    // ── URL ───────────────────────────────────────────────────────────────────
    /// URL is missing a valid scheme (http, https, ftp…).
    MissingScheme,
    /// URL is missing a host / domain.
    MissingHost,
    /// URL scheme is present but not recognised.
    UnsupportedScheme(String),

    // ── Date ──────────────────────────────────────────────────────────────────
    /// Year value is out of a sensible range.
    InvalidYear,
    /// Month is not 1–12.
    InvalidMonth,
    /// Day is out of range for the given month/year.
    InvalidDay,
    /// Feb 29 was given for a year that is not a leap year.
    NotALeapYear,

    // ── UUID ──────────────────────────────────────────────────────────────────
    /// UUID doesn't match the standard hyphenated or compact form.
    InvalidUuidFormat,
    /// UUID version nibble is not 1–5.
    InvalidUuidVersion,

    // ── Credit Card ───────────────────────────────────────────────────────────
    /// Card number fails the Luhn checksum.
    InvalidLuhnChecksum,
    /// Card number length is wrong for its detected network.
    InvalidCardLength,
    /// Card network (Visa, Mastercard…) could not be identified.
    UnknownCardNetwork,

    // ── IP Address ────────────────────────────────────────────────────────────
    /// IPv4 octet is out of the 0–255 range.
    InvalidIPv4Octet,
    /// IPv4 doesn't have exactly 4 octets.
    InvalidIPv4Format,
    /// IPv6 segment is not a valid hex group.
    InvalidIPv6Segment,
    /// IPv6 doesn't have the right number of groups.
    InvalidIPv6Format,

    // ── Postal Code ───────────────────────────────────────────────────────────
    /// Postal code doesn't match the pattern for the given country.
    InvalidPostalCodeFormat { country: String },
    /// Country code is not supported.
    UnsupportedCountry(String),

    // ── Username ──────────────────────────────────────────────────────────────
    /// Username contains a character that is not allowed.
    InvalidUsernameCharacter(char),
    /// Username matches a reserved/forbidden word.
    ReservedUsername(String),

    // ── Password ──────────────────────────────────────────────────────────────
    /// Password is missing at least one uppercase letter.
    MissingUppercase,
    /// Password is missing at least one lowercase letter.
    MissingLowercase,
    /// Password is missing at least one digit.
    MissingNumber,
    /// Password is missing at least one special/symbol character.
    MissingSymbol,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Input must not be empty."),
            Self::TooShort { min, actual } =>
                write!(f, "Input is too short: minimum {min} characters, got {actual}."),
            Self::TooLong { max, actual } =>
                write!(f, "Input is too long: maximum {max} characters, got {actual}."),
            Self::InvalidFormat { field, reason } =>
                write!(f, "Invalid {field}: {reason}"),

            Self::MissingAtSign =>
                write!(f, "Email is missing the '@' sign."),
            Self::InvalidLocalPart =>
                write!(f, "Email local part (before '@') is invalid."),
            Self::InvalidDomain =>
                write!(f, "Email domain (after '@') is invalid."),
            Self::InvalidTLD =>
                write!(f, "Email top-level domain is missing or invalid."),

            Self::MissingCountryCode =>
                write!(f, "Phone number must start with '+' followed by a country code."),
            Self::InvalidPhoneCharacters =>
                write!(f, "Phone number must contain only digits after '+'."),
            Self::InvalidPhoneLength =>
                write!(f, "Phone number digit count must be between 7 and 15 (E.164)."),

            Self::MissingScheme =>
                write!(f, "URL is missing a scheme (e.g. 'https://')."),
            Self::MissingHost =>
                write!(f, "URL is missing a host."),
            Self::UnsupportedScheme(s) =>
                write!(f, "URL scheme '{s}' is not supported. Use http, https, or ftp."),

            Self::InvalidYear  => write!(f, "Year is out of valid range (1 – 9999)."),
            Self::InvalidMonth => write!(f, "Month must be between 01 and 12."),
            Self::InvalidDay   => write!(f, "Day is out of range for the given month/year."),
            Self::NotALeapYear => write!(f, "Feb 29 is only valid in a leap year."),

            Self::InvalidUuidFormat  => write!(f, "UUID must be in 8-4-4-4-12 hyphenated format."),
            Self::InvalidUuidVersion => write!(f, "UUID version must be 1–5."),

            Self::InvalidLuhnChecksum => write!(f, "Credit card number fails the Luhn checksum."),
            Self::InvalidCardLength   => write!(f, "Credit card number length is incorrect."),
            Self::UnknownCardNetwork  => write!(f, "Card network (Visa, Mastercard…) could not be identified."),

            Self::InvalidIPv4Octet  => write!(f, "IPv4 octet value must be 0–255."),
            Self::InvalidIPv4Format => write!(f, "IPv4 address must have exactly 4 dot-separated octets."),
            Self::InvalidIPv6Segment => write!(f, "IPv6 segment contains invalid hex digits."),
            Self::InvalidIPv6Format  => write!(f, "IPv6 address format is invalid."),

            Self::InvalidPostalCodeFormat { country } =>
                write!(f, "Postal code format is invalid for country '{country}'."),
            Self::UnsupportedCountry(c) =>
                write!(f, "Country '{c}' is not currently supported for postal code validation."),

            Self::InvalidUsernameCharacter(ch) =>
                write!(f, "Username contains an invalid character: '{ch}'."),
            Self::ReservedUsername(name) =>
                write!(f, "Username '{name}' is reserved and cannot be used."),

            Self::MissingUppercase => write!(f, "Password must contain at least one uppercase letter."),
            Self::MissingLowercase => write!(f, "Password must contain at least one lowercase letter."),
            Self::MissingNumber    => write!(f, "Password must contain at least one digit."),
            Self::MissingSymbol    => write!(f, "Password must contain at least one special character."),
        }
    }
}

impl std::error::Error for ValidationError {}
