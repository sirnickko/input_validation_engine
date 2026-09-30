"""
tests/test_postal_code.py — Unit tests for PostalCodeValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidPostalCodeFormat,
)
from input_validation_engine.validators import Country, PostalCodeConfig, PostalCodeValidator


def test_valid_us_5digit():
    PostalCodeValidator.validate("90210", PostalCodeConfig(country=Country.US))

def test_valid_us_zip4():
    PostalCodeValidator.validate("90210-1234", PostalCodeConfig(country=Country.US))

def test_invalid_us():
    with pytest.raises(InvalidPostalCodeFormat):
        PostalCodeValidator.validate("ABCDE", PostalCodeConfig(country=Country.US))

def test_valid_uk():
    PostalCodeValidator.validate("SW1A 1AA", PostalCodeConfig(country=Country.UK))

def test_invalid_uk():
    with pytest.raises(InvalidPostalCodeFormat):
        PostalCodeValidator.validate("12345", PostalCodeConfig(country=Country.UK))

def test_valid_ca():
    PostalCodeValidator.validate("K1A 0B1", PostalCodeConfig(country=Country.CA))

def test_valid_de():
    PostalCodeValidator.validate("10115", PostalCodeConfig(country=Country.DE))

def test_valid_ke():
    PostalCodeValidator.validate("00100", PostalCodeConfig(country=Country.KE))

def test_empty():
    with pytest.raises(EmptyInput):
        PostalCodeValidator.validate("", PostalCodeConfig(country=Country.US))
