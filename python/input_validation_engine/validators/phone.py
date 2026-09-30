"""
validators/phone.py — E.164 phone number validator.

E.164 format: ``+`` followed by 7–15 digits (country code + subscriber number).
Examples: ``+254712345678`` (Kenya), ``+12025550173`` (USA).
"""

from __future__ import annotations

from dataclasses import dataclass

from input_validation_engine.errors import (
    EmptyInput,
    InvalidPhoneCharacters,
    InvalidPhoneLength,
    MissingCountryCode,
)
from input_validation_engine.validator import Validator

_SEPARATORS = frozenset(" -+()")


@dataclass
class PhoneConfig:
    """Configuration for :class:`PhoneValidator`."""

    allow_separators: bool = True
    """Strip spaces, hyphens, and parentheses before validation."""
    min_digits: int = 7
    """Minimum number of digits after the ``+`` (default 7)."""
    max_digits: int = 15
    """Maximum number of digits after the ``+`` (default 15)."""


class PhoneValidator(Validator[PhoneConfig]):
    """
    Validates international phone numbers in E.164 format.

    Example::

        PhoneValidator.validate("+12025550173", PhoneConfig())   # OK
        PhoneValidator.validate("12025550173",  PhoneConfig())   # raises MissingCountryCode
    """

    @classmethod
    def validate(cls, input: str, config: PhoneConfig = PhoneConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()
        if not input.startswith("+"):
            raise MissingCountryCode()

        digits_part = input[1:]
        if config.allow_separators:
            cleaned = "".join(c for c in digits_part if c not in _SEPARATORS)
        else:
            cleaned = digits_part

        if not cleaned.isdigit():
            raise InvalidPhoneCharacters()

        digit_count = len(cleaned)
        if not (config.min_digits <= digit_count <= config.max_digits):
            raise InvalidPhoneLength()
