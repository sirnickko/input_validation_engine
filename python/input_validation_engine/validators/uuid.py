"""
validators/uuid.py — UUID validator (RFC 4122).

Accepts both the standard hyphenated form ``8-4-4-4-12``
and the compact 32-hex-char form.
Version nibble must be 1–5.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from input_validation_engine.errors import (
    EmptyInput,
    InvalidUuidFormat,
    InvalidUuidVersion,
)
from input_validation_engine.validator import Validator


@dataclass
class UuidConfig:
    """Configuration for :class:`UuidValidator`."""

    allow_compact: bool = True
    """Also accept compact 32-char hex form (no hyphens). Default: ``True``."""
    require_version: Optional[int] = None
    """If set, only accept that specific UUID version (1–5). Default: ``None`` (any)."""


def _is_hex(s: str) -> bool:
    try:
        int(s, 16)
        return True
    except ValueError:
        return False


class UuidValidator(Validator[UuidConfig]):
    """
    Validates RFC 4122 UUIDs.

    Example::

        UuidValidator.validate("550e8400-e29b-41d4-a716-446655440000", UuidConfig())  # OK
        UuidValidator.validate("not-a-uuid", UuidConfig())                             # raises InvalidUuidFormat
    """

    @classmethod
    def validate(cls, input: str, config: UuidConfig = UuidConfig()) -> None:
        input = input.strip().lower()

        if not input:
            raise EmptyInput()

        # Normalise to 32-char hex for further checks
        if "-" in input:
            parts = input.split("-")
            expected_lengths = [8, 4, 4, 4, 12]
            if len(parts) != 5 or [len(p) for p in parts] != expected_lengths:
                raise InvalidUuidFormat()
            joined = "".join(parts)
            if not _is_hex(joined):
                raise InvalidUuidFormat()
            hex_str = joined
        elif len(input) == 32 and _is_hex(input):
            if not config.allow_compact:
                raise InvalidUuidFormat()
            hex_str = input
        else:
            raise InvalidUuidFormat()

        # Version nibble is the 13th hex char (index 12)
        version = int(hex_str[12], 16)

        if not (1 <= version <= 5):
            raise InvalidUuidVersion()

        if config.require_version is not None and version != config.require_version:
            raise InvalidUuidVersion()
