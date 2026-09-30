"""
validators/url.py — URL validator.

Checks that a URL has:
- A valid scheme (``http``, ``https``, ``ftp`` by default)
- ``://`` separator
- A non-empty host
- Optionally a path, query string, and/or fragment
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum, auto
from typing import List, Optional

from input_validation_engine.errors import (
    EmptyInput,
    MissingHost,
    MissingScheme,
    UnsupportedScheme,
)
from input_validation_engine.validator import Validator


class AllowedSchemes(Enum):
    """Which URL schemes are accepted."""

    HTTP_ONLY = auto()
    """Only ``http`` and ``https``."""
    ALL = auto()
    """``http``, ``https``, and ``ftp``."""
    CUSTOM = auto()
    """A caller-supplied list of schemes."""


@dataclass
class UrlConfig:
    """Configuration for :class:`UrlValidator`."""

    allowed_schemes: AllowedSchemes = AllowedSchemes.ALL
    """Scheme allowlist (default: http, https, ftp)."""
    custom_schemes: List[str] = field(default_factory=list)
    """Used when *allowed_schemes* is ``CUSTOM``."""
    require_dot_in_host: bool = True
    """Require the host to contain at least one ``.`` (default: ``True``)."""


class UrlValidator(Validator[UrlConfig]):
    """
    Validates URLs.

    Example::

        UrlValidator.validate("https://www.rust-lang.org/tools", UrlConfig())  # OK
        UrlValidator.validate("not-a-url", UrlConfig())                         # raises MissingScheme
    """

    @classmethod
    def validate(cls, input: str, config: UrlConfig = UrlConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()

        # Find "://"
        sep = input.find("://")
        if sep == -1:
            raise MissingScheme()

        scheme = input[:sep].lower()

        # Validate scheme
        if config.allowed_schemes == AllowedSchemes.HTTP_ONLY:
            allowed = {"http", "https"}
        elif config.allowed_schemes == AllowedSchemes.ALL:
            allowed = {"http", "https", "ftp"}
        else:
            allowed = set(s.lower() for s in config.custom_schemes)

        if scheme not in allowed:
            raise UnsupportedScheme(scheme)

        after_scheme = input[sep + 3:]

        # Host is everything up to '/', '?', '#', or end-of-string
        host_end = len(after_scheme)
        for i, ch in enumerate(after_scheme):
            if ch in ("?", "#", "/"):
                host_end = i
                break
        host = after_scheme[:host_end]

        if not host:
            raise MissingHost()
        if config.require_dot_in_host and "." not in host and host != "localhost":
            raise MissingHost()
