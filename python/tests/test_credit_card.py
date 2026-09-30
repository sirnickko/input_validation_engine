"""
tests/test_credit_card.py — Unit tests for CreditCardValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidFormat,
    InvalidLuhnChecksum,
    UnknownCardNetwork,
)
from input_validation_engine.validators import CreditCardConfig, CreditCardValidator

cfg = CreditCardConfig()


def test_valid_visa():
    CreditCardValidator.validate("4111111111111111", cfg)

def test_valid_visa_spaces():
    CreditCardValidator.validate("4111 1111 1111 1111", cfg)

def test_valid_mastercard():
    CreditCardValidator.validate("5500005555555559", cfg)

def test_valid_amex():
    CreditCardValidator.validate("378282246310005", cfg)

def test_valid_discover():
    CreditCardValidator.validate("6011111111111117", cfg)

def test_invalid_luhn():
    with pytest.raises(InvalidLuhnChecksum):
        CreditCardValidator.validate("4111111111111112", cfg)

def test_unknown_network():
    with pytest.raises(UnknownCardNetwork):
        CreditCardValidator.validate("1234567890123456", cfg)

def test_non_digits():
    with pytest.raises(InvalidFormat):
        CreditCardValidator.validate("4111-ABCD-1111-1111", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        CreditCardValidator.validate("", cfg)
