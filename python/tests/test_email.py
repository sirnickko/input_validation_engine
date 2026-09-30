"""
tests/test_email.py — Unit tests for EmailValidator.

Mirrors the Rust tests in ``src/validators/email.rs``.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidDomain,
    InvalidLocalPart,
    InvalidTLD,
    MissingAtSign,
)
from input_validation_engine.validators import EmailConfig, EmailValidator

cfg = EmailConfig()


# ── Valid ─────────────────────────────────────────────────────────────────────

def test_valid_simple():
    EmailValidator.validate("user@example.com", cfg)

def test_valid_plus_tag():
    EmailValidator.validate("user+tag@example.com", cfg)

def test_valid_subdomain():
    EmailValidator.validate("a@b.c.org", cfg)

def test_valid_hyphen_domain():
    EmailValidator.validate("info@my-site.io", cfg)


# ── Invalid ───────────────────────────────────────────────────────────────────

def test_missing_at():
    with pytest.raises(MissingAtSign):
        EmailValidator.validate("userexample.com", cfg)

def test_empty_local():
    with pytest.raises(InvalidLocalPart):
        EmailValidator.validate("@example.com", cfg)

def test_missing_tld():
    with pytest.raises(InvalidDomain):
        EmailValidator.validate("user@example", cfg)

def test_empty_input():
    with pytest.raises(EmptyInput):
        EmailValidator.validate("", cfg)

def test_double_dot_local():
    with pytest.raises(InvalidLocalPart):
        EmailValidator.validate("us..er@example.com", cfg)

def test_numeric_tld():
    with pytest.raises((InvalidTLD, InvalidDomain)):
        EmailValidator.validate("a@b.123", cfg)
