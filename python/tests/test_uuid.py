"""
tests/test_uuid.py — Unit tests for UuidValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidUuidFormat,
    InvalidUuidVersion,
)
from input_validation_engine.validators import UuidConfig, UuidValidator

cfg = UuidConfig()


def test_valid_v4_hyphenated():
    UuidValidator.validate("550e8400-e29b-41d4-a716-446655440000", cfg)

def test_valid_v1():
    UuidValidator.validate("6ba7b810-9dad-11d1-80b4-00c04fd430c8", cfg)

def test_valid_compact():
    UuidValidator.validate("550e8400e29b41d4a716446655440000", cfg)

def test_compact_rejected_when_disabled():
    strict = UuidConfig(allow_compact=False)
    with pytest.raises(InvalidUuidFormat):
        UuidValidator.validate("550e8400e29b41d4a716446655440000", strict)

def test_invalid_format():
    with pytest.raises(InvalidUuidFormat):
        UuidValidator.validate("not-a-uuid", cfg)

def test_invalid_version_0():
    with pytest.raises(InvalidUuidVersion):
        UuidValidator.validate("550e8400-e29b-01d4-a716-446655440000", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        UuidValidator.validate("", cfg)

def test_wrong_group_lengths():
    with pytest.raises(InvalidUuidFormat):
        UuidValidator.validate("550e840-e29b-41d4-a716-446655440000", cfg)
