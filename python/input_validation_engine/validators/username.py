"""
validators/username.py — Username validator with configurable rules.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import List

from input_validation_engine.errors import (
    EmptyInput,
    InvalidFormat,
    InvalidUsernameCharacter,
    ReservedUsername,
    TooLong,
    TooShort,
)
from input_validation_engine.validator import Validator

_DEFAULT_RESERVED = [
    "admin", "root", "system", "superuser", "null", "undefined",
]


@dataclass
class UsernameConfig:
    """Configuration for :class:`UsernameValidator`."""

    min_length: int = 3
    """Minimum length (default: 3)."""
    max_length: int = 32
    """Maximum length (default: 32)."""
    allow_underscores: bool = True
    """Allow underscore ``_`` (default: ``True``)."""
    allow_hyphens: bool = True
    """Allow hyphen ``-`` (default: ``True``)."""
    allow_dots: bool = False
    """Allow dot ``.`` (default: ``False``)."""
    reserved: List[str] = field(default_factory=lambda: list(_DEFAULT_RESERVED))
    """Words that are forbidden (case-insensitive)."""


class UsernameValidator(Validator[UsernameConfig]):
    """
    Validates usernames.

    Example::

        UsernameValidator.validate("nick_99", UsernameConfig())  # OK
        UsernameValidator.validate("admin",   UsernameConfig())  # raises ReservedUsername
    """

    @classmethod
    def validate(cls, input: str, config: UsernameConfig = UsernameConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()
        if len(input) < config.min_length:
            raise TooShort(min=config.min_length, actual=len(input))
        if len(input) > config.max_length:
            raise TooLong(max=config.max_length, actual=len(input))

        # Must start and end with alphanumeric
        if not input[0].isascii() or not input[0].isalnum():
            raise InvalidUsernameCharacter(input[0])
        if not input[-1].isascii() or not input[-1].isalnum():
            raise InvalidUsernameCharacter(input[-1])

        # Check each character
        for ch in input:
            ok = (
                (ch.isascii() and ch.isalnum())
                or (config.allow_underscores and ch == "_")
                or (config.allow_hyphens and ch == "-")
                or (config.allow_dots and ch == ".")
            )
            if not ok:
                raise InvalidUsernameCharacter(ch)

        # No consecutive special characters
        specials = frozenset("_-.")
        for a, b in zip(input, input[1:]):
            if a in specials and b in specials:
                raise InvalidFormat(
                    field="username",
                    reason="consecutive special characters are not allowed",
                )

        # Reserved word check (case-insensitive)
        lower = input.lower()
        for reserved in config.reserved:
            if lower == reserved.lower():
                raise ReservedUsername(input)
