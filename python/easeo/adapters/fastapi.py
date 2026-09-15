"""easeo adapter for FastAPI."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from easeo import SEOConfig


class EaseoSEO:
    """FastAPI SEO helper."""

    def __init__(self, config: SEOConfig) -> None:
        if config is None:
            raise ValueError(
                "SEOConfig is required. Pass a valid SEOConfig instance."
            )
        self.config = config

    def for_entity(self, entity: Any, route: str) -> dict:
        """Build SEO payload for a given entity and route."""
        from easeo import build_seo_payload
        from easeo.adapters._common import build_entity

        seo_entity = build_entity(entity)
        payload = build_seo_payload(seo_entity, route, self.config)
        return payload.to_dict()
