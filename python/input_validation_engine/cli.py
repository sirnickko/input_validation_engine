#!/usr/bin/env python3
"""
input_validator — CLI for the Input Validation Engine (Python port).

Usage:
    python -m input_validation_engine.cli --email "user@example.com"
    python -m input_validation_engine.cli --phone "+12025550173"
    python -m input_validation_engine.cli --url "https://rust-lang.org"
    python -m input_validation_engine.cli --date "2024-02-29"
    python -m input_validation_engine.cli --uuid "550e8400-e29b-41d4-a716-446655440000"
    python -m input_validation_engine.cli --card "4111111111111111"
    python -m input_validation_engine.cli --ip "192.168.1.1"
    python -m input_validation_engine.cli --postal "90210" --country US
    python -m input_validation_engine.cli --username "nick_99"
    python -m input_validation_engine.cli --password "Secure1pass!"
    python -m input_validation_engine.cli --interactive
"""

from __future__ import annotations

import sys

from input_validation_engine.errors import ValidationError
from input_validation_engine.validators import (
    CreditCardConfig,
    CreditCardValidator,
    DateConfig,
    DateValidator,
    EmailConfig,
    EmailValidator,
    IpConfig,
    IpValidator,
    PasswordConfig,
    PasswordValidator,
    PhoneConfig,
    PhoneValidator,
    PostalCodeConfig,
    PostalCodeValidator,
    Country,
    UuidConfig,
    UuidValidator,
    UrlConfig,
    UrlValidator,
    UsernameConfig,
    UsernameValidator,
)

# ── ANSI colour helpers ───────────────────────────────────────────────────────

def green(s: str) -> str:  return f"\x1b[32m{s}\x1b[0m"
def red(s: str) -> str:    return f"\x1b[31m{s}\x1b[0m"
def bold(s: str) -> str:   return f"\x1b[1m{s}\x1b[0m"
def cyan(s: str) -> str:   return f"\x1b[36m{s}\x1b[0m"
def yellow(s: str) -> str: return f"\x1b[33m{s}\x1b[0m"


def print_result(label: str, value: str, error: ValidationError | None) -> None:
    status = green("✅ VALID") if error is None else red("❌ INVALID")
    print(f'{bold(label)} {cyan(f"{chr(34)}{value}{chr(34)}")} → {status}')
    if error is not None:
        print(f"   {yellow('↳')} {error}")


def _country_from_str(code: str) -> Country | None:
    mapping = {
        "US": Country.US,
        "UK": Country.UK,
        "CA": Country.CA,
        "DE": Country.DE,
        "AU": Country.AU,
        "KE": Country.KE,
    }
    return mapping.get(code.upper())


def run_single(kind: str, value: str, country_code: str = "US") -> None:
    error: ValidationError | None = None

    try:
        if kind == "email":
            label = "Email   "
            EmailValidator.validate(value, EmailConfig())
        elif kind == "phone":
            label = "Phone   "
            PhoneValidator.validate(value, PhoneConfig())
        elif kind == "url":
            label = "URL     "
            UrlValidator.validate(value, UrlConfig())
        elif kind == "date":
            label = "Date    "
            DateValidator.validate(value, DateConfig())
        elif kind == "uuid":
            label = "UUID    "
            UuidValidator.validate(value, UuidConfig())
        elif kind == "card":
            label = "Card    "
            CreditCardValidator.validate(value, CreditCardConfig())
        elif kind == "ip":
            label = "IP      "
            IpValidator.validate(value, IpConfig())
        elif kind == "postal":
            label = "Postal  "
            country = _country_from_str(country_code)
            if country is None:
                print(red(f"Unknown country code: {country_code}"))
                return
            PostalCodeValidator.validate(value, PostalCodeConfig(country=country))
        elif kind == "username":
            label = "Username"
            UsernameValidator.validate(value, UsernameConfig())
        elif kind == "password":
            label = "Password"
            PasswordValidator.validate(value, PasswordConfig.strict())
        else:
            print(red(f"Unknown type: {kind}"))
            return
    except ValidationError as e:
        error = e
        # label might not be set if exception raised before assignment
        if "label" not in dir():
            label = kind.capitalize()

    print_result(label, value, error)


# ── Demo run ─────────────────────────────────────────────────────────────────

def demo_all() -> None:
    print(f"\n{bold('═══════ Input Validation Engine — Demo Run ═══════')}\n")

    samples: list[tuple[str, str]] = [
        ("email",    "alice@example.com"),
        ("email",    "not-an-email"),
        ("phone",    "+12025550173"),
        ("phone",    "12025550173"),
        ("url",      "https://www.rust-lang.org/tools"),
        ("url",      "not-a-url"),
        ("date",     "2024-02-29"),
        ("date",     "2023-02-29"),
        ("uuid",     "550e8400-e29b-41d4-a716-446655440000"),
        ("uuid",     "not-a-uuid"),
        ("card",     "4111111111111111"),
        ("card",     "1234567890123456"),
        ("ip",       "192.168.1.1"),
        ("ip",       "256.0.0.1"),
        ("postal",   "90210"),
        ("username", "nick_99"),
        ("username", "admin"),
        ("password", "Secure1pass!"),
        ("password", "weak"),
    ]

    for kind, value in samples:
        run_single(kind, value, "US")
    print()


# ── Interactive REPL ──────────────────────────────────────────────────────────

def interactive_mode() -> None:
    print(f"\n{bold('═══════ Input Validation Engine — Interactive Mode ═══════')}")
    print("Commands: email | phone | url | date | uuid | card | ip | postal | username | password | quit\n")

    while True:
        try:
            line = input(f"{cyan('validator>')} ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\nGoodbye!")
            break

        parts = line.split(None, 2)  # max 3 tokens

        if not parts:
            continue

        if parts[0] in ("quit", "exit", "q"):
            print("Goodbye!")
            break
        elif len(parts) == 2:
            run_single(parts[0], parts[1])
        elif len(parts) == 3:
            run_single(parts[0], parts[1], parts[2])
        else:
            print(yellow("Usage: <type> <value> [country]"))


# ── Help ──────────────────────────────────────────────────────────────────────

def print_help() -> None:
    print(bold("input-validator — Input Validation Engine CLI (Python)"))
    print("\nUsage: python -m input_validation_engine.cli [OPTION] <value>\n")
    rows = [
        ("--email <value>",    "Validate an email address"),
        ("--phone <value>",    "Validate a phone number (E.164)"),
        ("--url <value>",      "Validate a URL"),
        ("--date <value>",     "Validate an ISO 8601 date"),
        ("--uuid <value>",     "Validate a UUID"),
        ("--card <value>",     "Validate a credit card number"),
        ("--ip <value>",       "Validate an IP address"),
        ("--postal <value>",   "Validate a postal code"),
        ("--username <value>", "Validate a username"),
        ("--password <value>", "Validate a password"),
        ("--country <code>",   "Set country for postal (default US)"),
        ("--interactive",      "Interactive REPL mode"),
    ]
    for flag, desc in rows:
        print(f"  {flag:<22} {desc}")


# ── main ──────────────────────────────────────────────────────────────────────

def main() -> None:
    args = sys.argv[1:]

    if not args:
        demo_all()
        return

    i = 0
    ran = False
    country = "US"

    while i < len(args):
        arg = args[i]

        if arg in ("--interactive", "-i"):
            interactive_mode()
            ran = True

        elif arg in ("--help", "-h"):
            print_help()
            ran = True

        elif arg == "--country":
            i += 1
            if i < len(args):
                country = args[i]

        elif arg.startswith("--"):
            kind = arg[2:]
            i += 1
            if i < len(args):
                run_single(kind, args[i], country)
                ran = True
            else:
                print(red(f"Flag '{arg}' needs a value."))
        else:
            print(red(f"Unknown flag: {arg}. Use --help."))

        i += 1

    if not ran:
        demo_all()


if __name__ == "__main__":
    main()
