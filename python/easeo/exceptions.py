"""Public exception hierarchy for easeo.

All errors inherit from both ``EaseoError`` and ``ValueError``, so existing
code that catches ``ValueError`` keeps working (seoslug compatibility) while
``except EaseoError`` catches every easeo error.

The concrete types are defined by the native extension, which builds them
with multiple inheritance at import time. This module simply re-exports them
so ``from easeo.exceptions import ...`` is stable.
"""

from __future__ import annotations

from easeo._easeo_native import (
    ConfigurationError,
    ContractError,
    EaseoError,
    EntityError,
    InvalidUrlError,
    SchemaError,
)

__all__ = [
    "EaseoError",
    "InvalidUrlError",
    "ConfigurationError",
    "EntityError",
    "SchemaError",
    "ContractError",
]
