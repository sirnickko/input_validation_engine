
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationError {
    EmptyInput,
    TooShort { min: usize, actual: usize },
    TooLong { max: usize, actual: usize },
    InvalidFormat { field: &'static str, reason: String },
    MissingAtSign,
    InvalidLocalPart,
    InvalidDomain,
    InvalidTLD,
    MissingCountryCode,
    InvalidPhoneCharacters,
    InvalidPhoneLength,
    MissingScheme,
    MissingHost,
    UnsupportedScheme(String),
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    NotALeapYear,
    InvalidUuidFormat,
    InvalidUuidVersion,
    InvalidLuhnChecksum,
    InvalidCardLength,
    UnknownCardNetwork,
    InvalidIPv4Octet,
    InvalidIPv4Format,
    InvalidIPv6Segment,
    InvalidIPv6Format,
    InvalidPostalCodeFormat { country: String },
    UnsupportedCountry(String),
    InvalidUsernameCharacter(char),
    ReservedUsername(String),
    MissingUppercase,
    MissingLowercase,
    MissingNumber,
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
