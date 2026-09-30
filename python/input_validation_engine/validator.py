"""
validator.py — Abstract base class for all validators.

Mirrors the Rust ``Validator`` trait:

    pub trait Validator {
        type Config: Default;
        fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError>;
    }

In Python the ``Config`` is a plain dataclass passed as a keyword argument.
Validators are stateless classes with a single ``validate`` classmethod.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, TypeVar

C = TypeVar("C")


class Validator(ABC, Generic[C]):
    """
    Abstract base for every validator.

    Subclasses must implement :meth:`validate`.
    """

    @classmethod
    @abstractmethod
    def validate(cls, input: str, config: C) -> None:
        """
        Validate *input* against *config*.

        Returns ``None`` on success.
        Raises a :class:`~input_validation_engine.errors.ValidationError`
        subclass on failure.
        """
