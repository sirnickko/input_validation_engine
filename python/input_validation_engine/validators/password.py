"""
validators/password.py — Password strength validator with configurable rules.
"""

from __future__ import annotations

from dataclasses import dataclass

from input_validation_engine.errors import (
    EmptyInput,
    MissingLowercase,
    MissingNumber,
    MissingSymbol,
    MissingUppercase,
    TooLong,
    TooShort,
)
from input_validation_engine.validator import Validator

_SYMBOLS = frozenset(r"""!@#$%^&*()-_=+[]{}|;:'",.<>?/`~"\\""")


@dataclass
class PasswordConfig:
    """Configuration for :class:`PasswordValidator`."""

    min_length: int = 8
    """Minimum length (default: 8)."""
    max_length: int = 128
    """Maximum length (default: 128)."""
    require_uppercase: bool = True
    """Require at least one uppercase ASCII letter (default: ``True``)."""
    require_lowercase: bool = True
    """Require at least one lowercase ASCII letter (default: ``True``)."""
    require_number: bool = True
    """Require at least one ASCII digit (default: ``True``)."""
    require_symbol: bool = False
    """Require at least one special character (default: ``False``)."""

    @classmethod
    def strict(cls) -> "PasswordConfig":
        """Create a strict config that also requires a symbol."""
        return cls(require_symbol=True)


class PasswordValidator(Validator[PasswordConfig]):
    """
    Validates password strength.

    Example::

        PasswordValidator.validate("Secure1pass",  PasswordConfig())          # OK
        PasswordValidator.validate("Secure1pass!", PasswordConfig.strict())   # OK
        PasswordValidator.validate("weak",         PasswordConfig())          # raises TooShort
    """

    @classmethod
    def validate(cls, input: str, config: PasswordConfig = PasswordConfig()) -> None:
        # Do NOT strip passwords — leading/trailing spaces can be intentional
        if not input:
            raise EmptyInput()
        if len(input) < config.min_length:
            raise TooShort(min=config.min_length, actual=len(input))
        if len(input) > config.max_length:
            raise TooLong(max=config.max_length, actual=len(input))

        if config.require_uppercase and not any(c.isupper() and c.isascii() for c in input):
            raise MissingUppercase()
        if config.require_lowercase and not any(c.islower() and c.isascii() for c in input):
            raise MissingLowercase()
        if config.require_number and not any(c.isdigit() and c.isascii() for c in input):
            raise MissingNumber()
        if config.require_symbol and not any(c in _SYMBOLS for c in input):
            raise MissingSymbol()
