"""Shared utilities for easeo adapters."""

from __future__ import annotations

from typing import Any


def build_entity(entity: Any) -> Any:
    """Build SEOEntity from a generic entity object.

    Extracts entity_type, title, and excerpt/description from any object
    using getattr with sensible defaults.

    Raises:
        ValueError: If entity_type is not a valid string.
    """
    from easeo import SEOEntity

    entity_type = getattr(entity, "entity_type", None)
    if entity_type is None:
        entity_type = "page"
    elif not isinstance(entity_type, str):
        raise ValueError(
            f"entity_type must be a string, got {type(entity_type).__name__}"
        )

    title = getattr(entity, "title", None)
    excerpt = getattr(entity, "excerpt", None)
    if excerpt is None:
        excerpt = getattr(entity, "description", None)

    return SEOEntity(
        entity_type=entity_type,
        title=title,
        excerpt=excerpt,
    )
