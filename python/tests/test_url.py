"""
tests/test_url.py — Unit tests for UrlValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    MissingHost,
    MissingScheme,
    UnsupportedScheme,
)
from input_validation_engine.validators import UrlConfig, UrlValidator

cfg = UrlConfig()


def test_valid_https():
    UrlValidator.validate("https://www.rust-lang.org", cfg)

def test_valid_http_with_path():
    UrlValidator.validate("http://example.com/some/path?q=1#anchor", cfg)

def test_valid_ftp():
    UrlValidator.validate("ftp://files.example.com/file.txt", cfg)

def test_valid_localhost():
    UrlValidator.validate("http://localhost/", cfg)

def test_missing_scheme():
    with pytest.raises(MissingScheme):
        UrlValidator.validate("www.example.com", cfg)

def test_unsupported_scheme():
    with pytest.raises(UnsupportedScheme):
        UrlValidator.validate("ws://example.com", cfg)

def test_missing_host():
    with pytest.raises(MissingHost):
        UrlValidator.validate("https://", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        UrlValidator.validate("", cfg)
