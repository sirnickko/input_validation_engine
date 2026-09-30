"""
tests/test_ip_address.py — Unit tests for IpValidator.
"""

import pytest
from input_validation_engine.errors import (
    EmptyInput,
    InvalidIPv4Format,
    InvalidIPv4Octet,
    InvalidIPv6Format,
)
from input_validation_engine.validators import IpConfig, IpValidator

cfg = IpConfig()


# ── IPv4 ──────────────────────────────────────────────────────────────────────

def test_valid_ipv4():
    IpValidator.validate("192.168.1.1", cfg)

def test_valid_ipv4_boundary():
    IpValidator.validate("0.0.0.0", cfg)
    IpValidator.validate("255.255.255.255", cfg)

def test_invalid_ipv4_octet():
    with pytest.raises(InvalidIPv4Octet):
        IpValidator.validate("256.0.0.1", cfg)

def test_invalid_ipv4_too_few():
    with pytest.raises(InvalidIPv4Format):
        IpValidator.validate("192.168.1", cfg)

def test_invalid_ipv4_leading_zero():
    with pytest.raises(InvalidIPv4Octet):
        IpValidator.validate("192.168.01.1", cfg)


# ── IPv6 ──────────────────────────────────────────────────────────────────────

def test_valid_ipv6_full():
    IpValidator.validate("2001:0db8:85a3:0000:0000:8a2e:0370:7334", cfg)

def test_valid_ipv6_compressed():
    IpValidator.validate("::1", cfg)
    IpValidator.validate("2001:db8::1", cfg)

def test_invalid_ipv6_double_double_colon():
    with pytest.raises(InvalidIPv6Format):
        IpValidator.validate("::1::1", cfg)

def test_empty():
    with pytest.raises(EmptyInput):
        IpValidator.validate("", cfg)
