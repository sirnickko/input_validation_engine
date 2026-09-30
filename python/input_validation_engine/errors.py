"""
errors.py — Validation error types for the Input Validation Engine.

Each variant mirrors the Rust ``ValidationError`` enum so the Python port
has an identical error vocabulary.
"""

from __future__ import annotations


class ValidationError(Exception):
    """Base class for all validation errors."""

    def __str__(self) -> str:  # pragma: no cover
        return self.message

    @property
    def message(self) -> str:
        return super().__str__()


# ── Generic ───────────────────────────────────────────────────────────────────

class EmptyInput(ValidationError):
    def __init__(self) -> None:
        super().__init__("Input must not be empty.")


class TooShort(ValidationError):
    def __init__(self, *, min: int, actual: int) -> None:
        super().__init__(
            f"Input is too short: minimum {min} characters, got {actual}."
        )


class TooLong(ValidationError):
    def __init__(self, *, max: int, actual: int) -> None:
        super().__init__(
            f"Input is too long: maximum {max} characters, got {actual}."
        )


class InvalidFormat(ValidationError):
    def __init__(self, *, field: str, reason: str) -> None:
        super().__init__(f"Invalid {field}: {reason}")


# ── Email ─────────────────────────────────────────────────────────────────────

class MissingAtSign(ValidationError):
    def __init__(self) -> None:
        super().__init__("Email is missing the '@' sign.")


class InvalidLocalPart(ValidationError):
    def __init__(self) -> None:
        super().__init__("Email local part (before '@') is invalid.")


class InvalidDomain(ValidationError):
    def __init__(self) -> None:
        super().__init__("Email domain (after '@') is invalid.")


class InvalidTLD(ValidationError):
    def __init__(self) -> None:
        super().__init__("Email top-level domain is missing or invalid.")


# ── Phone ─────────────────────────────────────────────────────────────────────

class MissingCountryCode(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Phone number must start with '+' followed by a country code."
        )


class InvalidPhoneCharacters(ValidationError):
    def __init__(self) -> None:
        super().__init__("Phone number must contain only digits after '+'.")


class InvalidPhoneLength(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Phone number digit count must be between 7 and 15 (E.164)."
        )


# ── URL ───────────────────────────────────────────────────────────────────────

class MissingScheme(ValidationError):
    def __init__(self) -> None:
        super().__init__("URL is missing a scheme (e.g. 'https://').")


class MissingHost(ValidationError):
    def __init__(self) -> None:
        super().__init__("URL is missing a host.")


class UnsupportedScheme(ValidationError):
    def __init__(self, scheme: str) -> None:
        super().__init__(
            f"URL scheme '{scheme}' is not supported. Use http, https, or ftp."
        )


# ── Date ──────────────────────────────────────────────────────────────────────

class InvalidYear(ValidationError):
    def __init__(self) -> None:
        super().__init__("Year is out of valid range (1 – 9999).")


class InvalidMonth(ValidationError):
    def __init__(self) -> None:
        super().__init__("Month must be between 01 and 12.")


class InvalidDay(ValidationError):
    def __init__(self) -> None:
        super().__init__("Day is out of range for the given month/year.")


class NotALeapYear(ValidationError):
    def __init__(self) -> None:
        super().__init__("Feb 29 is only valid in a leap year.")


# ── UUID ──────────────────────────────────────────────────────────────────────

class InvalidUuidFormat(ValidationError):
    def __init__(self) -> None:
        super().__init__("UUID must be in 8-4-4-4-12 hyphenated format.")


class InvalidUuidVersion(ValidationError):
    def __init__(self) -> None:
        super().__init__("UUID version must be 1–5.")


# ── Credit Card ───────────────────────────────────────────────────────────────

class InvalidLuhnChecksum(ValidationError):
    def __init__(self) -> None:
        super().__init__("Credit card number fails the Luhn checksum.")


class InvalidCardLength(ValidationError):
    def __init__(self) -> None:
        super().__init__("Credit card number length is incorrect.")


class UnknownCardNetwork(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Card network (Visa, Mastercard…) could not be identified."
        )


# ── IP Address ────────────────────────────────────────────────────────────────

class InvalidIPv4Octet(ValidationError):
    def __init__(self) -> None:
        super().__init__("IPv4 octet value must be 0–255.")


class InvalidIPv4Format(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "IPv4 address must have exactly 4 dot-separated octets."
        )


class InvalidIPv6Segment(ValidationError):
    def __init__(self) -> None:
        super().__init__("IPv6 segment contains invalid hex digits.")


class InvalidIPv6Format(ValidationError):
    def __init__(self) -> None:
        super().__init__("IPv6 address format is invalid.")


# ── Postal Code ───────────────────────────────────────────────────────────────

class InvalidPostalCodeFormat(ValidationError):
    def __init__(self, *, country: str) -> None:
        super().__init__(
            f"Postal code format is invalid for country '{country}'."
        )


class UnsupportedCountry(ValidationError):
    def __init__(self, country: str) -> None:
        super().__init__(
            f"Country '{country}' is not currently supported for postal code validation."
        )


# ── Username ──────────────────────────────────────────────────────────────────

class InvalidUsernameCharacter(ValidationError):
    def __init__(self, ch: str) -> None:
        super().__init__(f"Username contains an invalid character: '{ch}'.")


class ReservedUsername(ValidationError):
    def __init__(self, name: str) -> None:
        super().__init__(
            f"Username '{name}' is reserved and cannot be used."
        )


# ── Password ──────────────────────────────────────────────────────────────────

class MissingUppercase(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Password must contain at least one uppercase letter."
        )


class MissingLowercase(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Password must contain at least one lowercase letter."
        )


class MissingNumber(ValidationError):
    def __init__(self) -> None:
        super().__init__("Password must contain at least one digit.")


class MissingSymbol(ValidationError):
    def __init__(self) -> None:
        super().__init__(
            "Password must contain at least one special character."
        )
