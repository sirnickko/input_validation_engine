"""
tests/test_date.py — Unit tests for DateValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidDay,
    InvalidFormat,
    InvalidMonth,
    NotALeapYear,
)
from input_validation_engine.validators import DateConfig, DateValidator

cfg = DateConfig()


def test_valid_regular():
    DateValidator.validate("2024-06-15", cfg)

def test_valid_leap_feb29():
    DateValidator.validate("2024-02-29", cfg)

def test_invalid_not_leap():
    with pytest.raises(NotALeapYear):
        DateValidator.validate("2023-02-29", cfg)

def test_invalid_month_13():
    with pytest.raises(InvalidMonth):
        DateValidator.validate("2024-13-01", cfg)

def test_invalid_day_31_april():
    with pytest.raises(InvalidDay):
        DateValidator.validate("2024-04-31", cfg)

def test_wrong_format():
    with pytest.raises(InvalidFormat):
        DateValidator.validate("2024/06/15", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        DateValidator.validate("", cfg)

def test_valid_400_year_leap():
    DateValidator.validate("2000-02-29", cfg)

def test_invalid_100_year_not_leap():
    with pytest.raises(NotALeapYear):
        DateValidator.validate("1900-02-29", cfg)
