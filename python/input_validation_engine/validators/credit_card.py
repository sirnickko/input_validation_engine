"""
validators/credit_card.py — Credit card number validator.

Uses the **Luhn algorithm** to verify the check digit,
then detects the card network from the IIN/BIN prefix.

Supported networks: Visa, Mastercard, American Express, Discover, JCB, Diners Club.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto
from typing import Optional

from input_validation_engine.errors import (
    EmptyInput,
    InvalidFormat,
    InvalidLuhnChecksum,
    UnknownCardNetwork,
)
from input_validation_engine.validator import Validator


class CardNetwork(Enum):
    """Detected card network."""

    VISA = auto()
    MASTERCARD = auto()
    AMEX = auto()
    DISCOVER = auto()
    JCB = auto()
    DINERS_CLUB = auto()


@dataclass
class CreditCardConfig:
    """Configuration for :class:`CreditCardValidator`."""

    allow_separators: bool = True
    """Allow spaces and hyphens as grouping separators (stripped before checking)."""


def _luhn_check(digits: list[int]) -> bool:
    """Return ``True`` if *digits* pass the Luhn algorithm."""
    total = 0
    for i, d in enumerate(reversed(digits)):
        n = d
        if i % 2 == 1:
            n *= 2
            if n > 9:
                n -= 9
        total += n
    return total % 10 == 0


def _detect_network(digits: str) -> Optional[CardNetwork]:
    """Detect the card network from the digit string."""
    n = len(digits)
    f1 = digits[:1]
    f2 = digits[:2] if n >= 2 else ""
    f3 = digits[:3] if n >= 3 else ""
    f4 = digits[:4] if n >= 4 else ""
    f6 = digits[:6] if n >= 6 else ""

    # American Express: starts with 34 or 37, length 15
    if f2 in ("34", "37") and n == 15:
        return CardNetwork.AMEX

    # Visa: starts with 4, length 13 or 16
    if f1 == "4" and n in (13, 16):
        return CardNetwork.VISA

    # Mastercard: starts with 51–55 or 2221–2720, length 16
    if n == 16:
        if f2.isdigit() and 51 <= int(f2) <= 55:
            return CardNetwork.MASTERCARD
        if f4.isdigit() and 2221 <= int(f4) <= 2720:
            return CardNetwork.MASTERCARD

    # Discover: starts with 6011, 622126–622925, 644–649, 65; length 16
    if n == 16:
        if f4 == "6011" or f2 == "65":
            return CardNetwork.DISCOVER
        if f6.isdigit() and 622126 <= int(f6) <= 622925:
            return CardNetwork.DISCOVER
        if f3.isdigit() and 644 <= int(f3) <= 649:
            return CardNetwork.DISCOVER

    # JCB: starts with 3528–3589, length 16
    if n == 16 and f4.isdigit() and 3528 <= int(f4) <= 3589:
        return CardNetwork.JCB

    # Diners Club: starts with 300–305 or 36 or 38, length 14
    if n == 14:
        if f2 in ("36", "38"):
            return CardNetwork.DINERS_CLUB
        if f3.isdigit() and 300 <= int(f3) <= 305:
            return CardNetwork.DINERS_CLUB

    return None


_SEPARATORS = frozenset(" -")


class CreditCardValidator(Validator[CreditCardConfig]):
    """
    Validates credit card numbers via Luhn algorithm + network detection.

    Example::

        CreditCardValidator.validate("4111111111111111", CreditCardConfig())  # OK (Visa)
        CreditCardValidator.validate("1234567890123456", CreditCardConfig())  # raises UnknownCardNetwork
    """

    @classmethod
    def validate(cls, input: str, config: CreditCardConfig = CreditCardConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()

        cleaned = (
            "".join(c for c in input if c not in _SEPARATORS)
            if config.allow_separators
            else input
        )

        if not cleaned.isdigit():
            raise InvalidFormat(field="credit card", reason="must contain only digits")

        digits = [int(c) for c in cleaned]

        if _detect_network(cleaned) is None:
            raise UnknownCardNetwork()

        if not _luhn_check(digits):
            raise InvalidLuhnChecksum()
