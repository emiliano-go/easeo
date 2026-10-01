"""easeo adapter for FastAPI."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from easeo import SEOConfig


class EaseoSEO:
    """FastAPI SEO helper.

    Usage::

        from fastapi import FastAPI
        from easeo import SEOConfig
        from easeo.adapters.fastapi import EaseoSEO

        app = FastAPI()
        easeo = EaseoSEO(SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
        ))

        @app.get("/blog/{slug}")
        def post(slug: str):
            return easeo.for_entity(Post(...), f"/blog/{slug}")
    """

    def __init__(self, config: SEOConfig) -> None:
        """Creates the helper with a site-wide configuration.

        Args:
            config: Site-wide configuration.

        Raises:
            ValueError: If ``config`` is ``None``.
        """
        if config is None:
            raise ValueError(
                "SEOConfig is required. Pass a valid SEOConfig instance."
            )
        self.config = config

    def for_entity(self, entity: Any, route: str) -> dict:
        """Builds the SEO payload for an entity at a route.

        Args:
            entity: Object exposing ``entity_type``, ``title``, and
                ``excerpt`` or ``description`` attributes.
            route: Route path, for example ``"/blog/hello"``.

        Returns:
            The payload as a plain dictionary.
        """
        from easeo import build_seo_payload
        from easeo.adapters._common import build_entity

        seo_entity = build_entity(entity)
        payload = build_seo_payload(seo_entity, route, self.config)
        return payload.to_dict()
