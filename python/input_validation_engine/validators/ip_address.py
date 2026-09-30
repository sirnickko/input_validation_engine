"""
validators/ip_address.py — IP address validator (IPv4 and IPv6).
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto

from input_validation_engine.errors import (
    EmptyInput,
    InvalidFormat,
    InvalidIPv4Format,
    InvalidIPv4Octet,
    InvalidIPv6Format,
    InvalidIPv6Segment,
)
from input_validation_engine.validator import Validator


class IpVersion(Enum):
    """Which IP versions are accepted."""

    V4 = auto()
    """Only IPv4."""
    V6 = auto()
    """Only IPv6."""
    BOTH = auto()
    """Either IPv4 or IPv6."""


@dataclass
class IpConfig:
    """Configuration for :class:`IpValidator`."""

    version: IpVersion = IpVersion.BOTH
    """Which version(s) to accept (default: ``BOTH``)."""


def _validate_ipv4(input: str) -> None:
    octets = input.split(".")
    if len(octets) != 4:
        raise InvalidIPv4Format()
    for octet in octets:
        try:
            val = int(octet)
        except ValueError:
            raise InvalidIPv4Octet()
        if val > 255:
            raise InvalidIPv4Octet()
        # Disallow leading zeros (e.g. "01") — ambiguous in some contexts
        if len(octet) > 1 and octet.startswith("0"):
            raise InvalidIPv4Octet()


def _validate_ipv6(input: str) -> None:
    double_colon_count = input.count("::")
    if double_colon_count > 1:
        raise InvalidIPv6Format()

    if double_colon_count == 1:
        idx = input.index("::")
        left = input[:idx]
        right = input[idx + 2:]
    else:
        left = input
        right = ""

    groups: list[str] = []
    if left:
        groups.extend(left.split(":"))
    if right:
        groups.extend(right.split(":"))

    max_groups = 7 if double_colon_count == 1 else 8
    min_groups = 0 if double_colon_count == 1 else 8

    if not (min_groups <= len(groups) <= max_groups):
        raise InvalidIPv6Format()

    for seg in groups:
        if not seg or len(seg) > 4:
            raise InvalidIPv6Segment()
        try:
            int(seg, 16)
        except ValueError:
            raise InvalidIPv6Segment()


class IpValidator(Validator[IpConfig]):
    """
    Validates IPv4 and IPv6 addresses.

    Example::

        IpValidator.validate("192.168.1.1", IpConfig())   # OK
        IpValidator.validate("::1",         IpConfig())   # OK
        IpValidator.validate("999.0.0.1",   IpConfig())   # raises InvalidIPv4Octet
    """

    @classmethod
    def validate(cls, input: str, config: IpConfig = IpConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()

        is_v4 = "." in input and ":" not in input
        is_v6 = ":" in input

        version = config.version

        if version == IpVersion.V4:
            if not is_v4:
                raise InvalidIPv4Format()
            _validate_ipv4(input)
        elif version == IpVersion.V6:
            if is_v4:
                raise InvalidIPv6Format()
            if not is_v6:
                raise InvalidIPv6Format()
            _validate_ipv6(input)
        else:  # BOTH
            if is_v4:
                _validate_ipv4(input)
            elif is_v6:
                _validate_ipv6(input)
            else:
                raise InvalidFormat(
                    field="IP address", reason="could not determine IP version"
                )
