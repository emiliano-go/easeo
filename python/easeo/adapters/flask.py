"""easeo adapter for Flask."""

from __future__ import annotations

from typing import Any


class Easeo:
    """Flask SEO helper.

    Usage::

        from flask import Flask
        from easeo.adapters.flask import Easeo
        from easeo import SEOConfig

        app = Flask(__name__)
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        easeo = Easeo(app, config)
    """

    def __init__(self, app: Any = None, config: Any = None) -> None:
        self.config = config
        if app is not None:
            self.init_app(app)

    def init_app(self, app: Any) -> None:
        """Initialize with a Flask app."""
        if self.config is None:
            raise ValueError("config must be provided before init_app")

        app.config["EASEO_CONFIG"] = self.config

        @app.context_processor
        def easeo_context():
            def seo_head(entity: Any, route: str) -> str:
                from easeo import SEOEntity, build_seo_payload
                seo_entity = SEOEntity(
                    entity_type=getattr(entity, "entity_type", "page"),
                    title=getattr(entity, "title", None),
                    excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
                )
                payload = build_seo_payload(seo_entity, route, self.config)
                return payload.render_html()
            return dict(easeo_head=seo_head)

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
