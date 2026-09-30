"""
input_validation_engine — A reusable, zero-dependency input validation library.

Validators available:
- :class:`~input_validation_engine.validators.EmailValidator` — RFC 5322-style email addresses
- :class:`~input_validation_engine.validators.PhoneValidator` — E.164 international phone numbers
- :class:`~input_validation_engine.validators.UrlValidator`   — HTTP/HTTPS/FTP URLs
- :class:`~input_validation_engine.validators.DateValidator`  — ISO 8601 dates (YYYY-MM-DD)
- :class:`~input_validation_engine.validators.UuidValidator`  — RFC 4122 UUIDs (v1–v5)
- :class:`~input_validation_engine.validators.CreditCardValidator` — Luhn-checked card numbers
- :class:`~input_validation_engine.validators.IpValidator`    — IPv4 and IPv6 addresses
- :class:`~input_validation_engine.validators.PostalCodeValidator` — Country-specific postal codes
- :class:`~input_validation_engine.validators.UsernameValidator`  — Configurable username rules
- :class:`~input_validation_engine.validators.PasswordValidator`  — Configurable password strength

Quick Example::

    from input_validation_engine.validators import EmailValidator, EmailConfig

    EmailValidator.validate("alice@example.com", EmailConfig())  # OK
"""

from input_validation_engine.errors import ValidationError
from input_validation_engine.validator import Validator
from input_validation_engine.validators import (
    AllowedSchemes,
    CardNetwork,
    Country,
    CreditCardConfig,
    CreditCardValidator,
    DateConfig,
    DateValidator,
    EmailConfig,
    EmailValidator,
    IpConfig,
    IpVersion,
    IpValidator,
    PasswordConfig,
    PasswordValidator,
    PhoneConfig,
    PhoneValidator,
    PostalCodeConfig,
    PostalCodeValidator,
    UuidConfig,
    UuidValidator,
    UrlConfig,
    UrlValidator,
    UsernameConfig,
    UsernameValidator,
)

__all__ = [
    "ValidationError",
    "Validator",
    # validators
    "EmailConfig", "EmailValidator",
    "PhoneConfig", "PhoneValidator",
    "AllowedSchemes", "UrlConfig", "UrlValidator",
    "DateConfig", "DateValidator",
    "UuidConfig", "UuidValidator",
    "CardNetwork", "CreditCardConfig", "CreditCardValidator",
    "IpConfig", "IpVersion", "IpValidator",
    "Country", "PostalCodeConfig", "PostalCodeValidator",
    "UsernameConfig", "UsernameValidator",
    "PasswordConfig", "PasswordValidator",
]
