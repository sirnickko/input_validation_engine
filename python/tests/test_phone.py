"""
tests/test_phone.py — Unit tests for PhoneValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidPhoneCharacters,
    InvalidPhoneLength,
    MissingCountryCode,
)
from input_validation_engine.validators import PhoneConfig, PhoneValidator

cfg = PhoneConfig()


def test_valid_us():
    PhoneValidator.validate("+12025550173", cfg)

def test_valid_kenya():
    PhoneValidator.validate("+254712345678", cfg)

def test_valid_with_spaces():
    PhoneValidator.validate("+1 202 555 0173", cfg)

def test_valid_with_hyphens():
    PhoneValidator.validate("+1-800-555-0100", cfg)

def test_missing_plus():
    with pytest.raises(MissingCountryCode):
        PhoneValidator.validate("12025550173", cfg)

def test_non_digit_chars():
    with pytest.raises(InvalidPhoneCharacters):
        PhoneValidator.validate("+1abc5550173", cfg)

def test_too_short():
    with pytest.raises(InvalidPhoneLength):
        PhoneValidator.validate("+123", cfg)

def test_too_long():
    with pytest.raises(InvalidPhoneLength):
        PhoneValidator.validate("+1234567890123456", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        PhoneValidator.validate("", cfg)
