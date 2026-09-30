"""
tests/test_username.py — Unit tests for UsernameValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidFormat,
    InvalidUsernameCharacter,
    ReservedUsername,
    TooShort,
)
from input_validation_engine.validators import UsernameConfig, UsernameValidator

cfg = UsernameConfig()


def test_valid_simple():
    UsernameValidator.validate("alice", cfg)

def test_valid_with_underscore():
    UsernameValidator.validate("nick_99", cfg)

def test_valid_with_hyphen():
    UsernameValidator.validate("john-doe", cfg)

def test_reserved_word():
    with pytest.raises(ReservedUsername):
        UsernameValidator.validate("admin", cfg)

def test_reserved_case_insensitive():
    with pytest.raises(ReservedUsername):
        UsernameValidator.validate("ADMIN", cfg)

def test_too_short():
    with pytest.raises(TooShort):
        UsernameValidator.validate("ab", cfg)

def test_invalid_special_char():
    with pytest.raises(InvalidUsernameCharacter):
        UsernameValidator.validate("nick@99", cfg)

def test_consecutive_specials():
    with pytest.raises(InvalidFormat):
        UsernameValidator.validate("nick__99", cfg)

def test_starts_with_special():
    with pytest.raises(InvalidUsernameCharacter):
        UsernameValidator.validate("_nick", cfg)

def test_ends_with_special():
    with pytest.raises(InvalidUsernameCharacter):
        UsernameValidator.validate("nick_", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        UsernameValidator.validate("", cfg)
