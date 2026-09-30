"""
validators/postal_code.py — Postal code validator with country-specific rules.

Supported countries:
- ``US`` — 5-digit or ZIP+4 (e.g. ``90210``, ``90210-1234``)
- ``UK`` — UK postcode formats (e.g. ``SW1A 1AA``, ``M1 1AE``)
- ``CA`` — Canadian postal code (e.g. ``K1A 0B1``)
- ``DE`` — German PLZ, 5 digits (e.g. ``10115``)
- ``AU`` — Australian 4-digit code (e.g. ``2000``)
- ``KE`` — Kenyan 5-digit code (e.g. ``00100``)
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto

from input_validation_engine.errors import (
    EmptyInput,
    InvalidPostalCodeFormat,
)
from input_validation_engine.validator import Validator


class Country(Enum):
    """Supported countries for postal code validation."""

    US = "US"
    UK = "UK"
    CA = "CA"
    DE = "DE"
    AU = "AU"
    KE = "KE"


@dataclass
class PostalCodeConfig:
    """Configuration for :class:`PostalCodeValidator`."""

    country: Country = Country.US
    """Target country whose rules to apply."""


def _validate_us(s: str) -> bool:
    """12345 or 12345-6789."""
    s = s.strip()
    if len(s) == 5:
        return s.isdigit()
    if len(s) == 10:
        zip_, ext = s[:5], s[5:]
        return zip_.isdigit() and ext.startswith("-") and ext[1:].isdigit()
    return False


def _validate_uk(s: str) -> bool:
    """UK postcodes: outward code (2–4 chars) + space + inward code (digit + 2 letters)."""
    s = s.strip().upper()
    parts = s.split()
    if len(parts) != 2:
        return False
    out, inward = parts
    inward_ok = (
        len(inward) == 3
        and inward[0].isdigit()
        and inward[1:].isalpha()
    )
    outward_ok = 2 <= len(out) <= 4 and out.isalnum()
    return inward_ok and outward_ok


def _validate_ca(s: str) -> bool:
    """A1A 1A1 (letter-digit alternating pattern)."""
    s = s.strip().upper().replace(" ", "")
    if len(s) != 6:
        return False
    pattern = [str.isalpha, str.isdigit, str.isalpha, str.isdigit, str.isalpha, str.isdigit]
    return all(fn(c) for fn, c in zip(pattern, s))


def _validate_numeric(s: str, length: int) -> bool:
    s = s.strip()
    return len(s) == length and s.isdigit()


class PostalCodeValidator(Validator[PostalCodeConfig]):
    """
    Validates postal codes for specific countries.

    Example::

        cfg = PostalCodeConfig(country=Country.US)
        PostalCodeValidator.validate("90210", cfg)   # OK
        PostalCodeValidator.validate("ABCDE", cfg)   # raises InvalidPostalCodeFormat
    """

    @classmethod
    def validate(cls, input: str, config: PostalCodeConfig = PostalCodeConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()

        dispatch = {
            Country.US: _validate_us,
            Country.UK: _validate_uk,
            Country.CA: _validate_ca,
            Country.DE: lambda s: _validate_numeric(s, 5),
            Country.AU: lambda s: _validate_numeric(s, 4),
            Country.KE: lambda s: _validate_numeric(s, 5),
        }

        valid = dispatch[config.country](input)
        if not valid:
            raise InvalidPostalCodeFormat(country=config.country.value)
