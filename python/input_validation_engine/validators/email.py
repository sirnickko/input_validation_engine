"""
validators/email.py — Email address validator.

Rules enforced:
- Must contain exactly one ``@``
- Local part: 1–64 chars, alphanumeric + ``.-_+``
- Domain: one or more labels separated by ``.``, each label alphanumeric + ``-``
- TLD: 2–63 alphabetic characters
"""

from __future__ import annotations

from dataclasses import dataclass, field

from input_validation_engine.errors import (
    EmptyInput,
    InvalidDomain,
    InvalidLocalPart,
    InvalidTLD,
    MissingAtSign,
    TooLong,
)
from input_validation_engine.validator import Validator

_LOCAL_ALLOWED = frozenset(
    "abcdefghijklmnopqrstuvwxyz"
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    "0123456789"
    "._+-"
)
_DOMAIN_LABEL_ALLOWED = frozenset(
    "abcdefghijklmnopqrstuvwxyz"
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    "0123456789-"
)


@dataclass
class EmailConfig:
    """Configuration for :class:`EmailValidator`."""

    max_length: int = 254


class EmailValidator(Validator[EmailConfig]):
    """
    Validates email addresses.

    Example::

        EmailValidator.validate("user@example.com", EmailConfig())  # OK
        EmailValidator.validate("not-an-email", EmailConfig())       # raises MissingAtSign
    """

    @classmethod
    def validate(cls, input: str, config: EmailConfig = EmailConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()
        if len(input) > config.max_length:
            raise TooLong(max=config.max_length, actual=len(input))

        at_count = input.count("@")
        if at_count == 0:
            raise MissingAtSign()
        if at_count > 1:
            raise InvalidLocalPart()

        local, domain = input.split("@", 1)

        # ── local part ────────────────────────────────────────────────────────
        if not local or len(local) > 64:
            raise InvalidLocalPart()
        if local.startswith(".") or local.endswith(".") or ".." in local:
            raise InvalidLocalPart()
        if not all(c in _LOCAL_ALLOWED for c in local):
            raise InvalidLocalPart()

        # ── domain part ───────────────────────────────────────────────────────
        if not domain:
            raise InvalidDomain()
        labels = domain.split(".")
        if len(labels) < 2:
            raise InvalidDomain()
        for label in labels:
            if not label or len(label) > 63:
                raise InvalidDomain()
            if label.startswith("-") or label.endswith("-"):
                raise InvalidDomain()
            if not all(c in _DOMAIN_LABEL_ALLOWED for c in label):
                raise InvalidDomain()

        # ── TLD ───────────────────────────────────────────────────────────────
        tld = labels[-1]
        if not (2 <= len(tld) <= 63):
            raise InvalidTLD()
        if not tld.isalpha():
            raise InvalidTLD()
