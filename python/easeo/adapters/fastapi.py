"""easeo adapter for FastAPI."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from easeo import SEOConfig


class EaseoSEO:
    """FastAPI SEO helper."""

    def __init__(self, config: SEOConfig) -> None:
        self.config = config

    def for_entity(self, entity: Any, route: str) -> dict:
        """Build SEO payload for a given entity and route."""
        from easeo import SEOEntity, build_seo_payload

        seo_entity = SEOEntity(
            entity_type=getattr(entity, "entity_type", "page"),
            title=getattr(entity, "title", None),
            excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
        )
        payload = build_seo_payload(seo_entity, route, self.config)
        return payload.to_dict()
