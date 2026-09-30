"""
validators/date.py — ISO 8601 date validator (YYYY-MM-DD).

Rules:
- Format must be exactly ``YYYY-MM-DD``
- Year: 1–9999
- Month: 01–12
- Day: correct for the given month, with leap-year awareness for February
"""

from __future__ import annotations

from dataclasses import dataclass

from input_validation_engine.errors import (
    EmptyInput,
    InvalidDay,
    InvalidFormat,
    InvalidMonth,
    InvalidYear,
    NotALeapYear,
)
from input_validation_engine.validator import Validator

# Days per month for non-leap years
_DAYS_IN_MONTH = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def _is_leap(year: int) -> bool:
    return (year % 4 == 0 and year % 100 != 0) or (year % 400 == 0)


def _days_in_month(month: int, year: int) -> int:
    if month == 2 and _is_leap(year):
        return 29
    return _DAYS_IN_MONTH[month]


@dataclass
class DateConfig:
    """Configuration for :class:`DateValidator`."""

    min_year: int = 1
    """Earliest allowed year (inclusive)."""
    max_year: int = 9999
    """Latest allowed year (inclusive)."""


class DateValidator(Validator[DateConfig]):
    """
    Validates ISO 8601 dates.

    Example::

        DateValidator.validate("2024-02-29", DateConfig())  # OK – leap year
        DateValidator.validate("2023-02-29", DateConfig())  # raises NotALeapYear
    """

    @classmethod
    def validate(cls, input: str, config: DateConfig = DateConfig()) -> None:
        input = input.strip()

        if not input:
            raise EmptyInput()

        parts = input.split("-")
        if len(parts) != 3 or len(parts[0]) != 4 or len(parts[1]) != 2 or len(parts[2]) != 2:
            raise InvalidFormat(field="date", reason="expected YYYY-MM-DD")

        try:
            year = int(parts[0])
        except ValueError:
            raise InvalidYear()
        try:
            month = int(parts[1])
        except ValueError:
            raise InvalidMonth()
        try:
            day = int(parts[2])
        except ValueError:
            raise InvalidDay()

        if not (config.min_year <= year <= config.max_year):
            raise InvalidYear()
        if not (1 <= month <= 12):
            raise InvalidMonth()

        max_day = _days_in_month(month, year)
        if not (1 <= day <= max_day):
            # Distinguish leap-year Feb 29 specifically
            if month == 2 and day == 29:
                raise NotALeapYear()
            raise InvalidDay()
