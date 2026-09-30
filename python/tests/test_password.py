"""
tests/test_password.py — Unit tests for PasswordValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    MissingLowercase,
    MissingNumber,
    MissingSymbol,
    MissingUppercase,
    TooLong,
    TooShort,
)
from input_validation_engine.validators import PasswordConfig, PasswordValidator

cfg = PasswordConfig()


def test_valid_password():
    PasswordValidator.validate("Secure1pass", cfg)

def test_valid_with_symbol():
    PasswordValidator.validate("Secure1pass!", PasswordConfig.strict())

def test_too_short():
    with pytest.raises(TooShort):
        PasswordValidator.validate("Ab1", cfg)

def test_missing_uppercase():
    with pytest.raises(MissingUppercase):
        PasswordValidator.validate("secure1pass", cfg)

def test_missing_lowercase():
    with pytest.raises(MissingLowercase):
        PasswordValidator.validate("SECURE1PASS", cfg)

def test_missing_number():
    with pytest.raises(MissingNumber):
        PasswordValidator.validate("Securepass", cfg)

def test_missing_symbol_when_required():
    with pytest.raises(MissingSymbol):
        PasswordValidator.validate("Secure1pass", PasswordConfig.strict())

def test_empty():
    with pytest.raises(EmptyInput):
        PasswordValidator.validate("", cfg)

def test_too_long():
    long_pw = "A1a" * 50  # 150 chars > 128 max
    with pytest.raises(TooLong):
        PasswordValidator.validate(long_pw, cfg)
