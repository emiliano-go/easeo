"""easeo adapter for Django."""

from __future__ import annotations

from typing import Any

from django import template
from django.conf import settings

register = template.Library()


def _get_config():
    """Get SEO config from Django settings."""
    from easeo import SEOConfig
    config_data = getattr(settings, "EASEO", {})
    return SEOConfig(
        canonical_host=config_data.get("canonical_host", "localhost"),
        public_base_url=config_data.get("public_base_url", "http://localhost"),
        site_name=config_data.get("site_name"),
        title_template=config_data.get("title_template"),
    )


@register.simple_tag(takes_context=True)
def easeo_head(context: Any, entity: Any, route: str) -> str:
    """Render SEO <head> tags for a Django template.

    Usage::

        {% load easeo_tags %}
        <head>
            {% easeo_head entity request.path %}
        </head>
    """
    from easeo import SEOEntity, build_seo_payload

    seo_entity = SEOEntity(
        entity_type=getattr(entity, "entity_type", "page"),
        title=getattr(entity, "title", None),
        excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
    )
    config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return payload.render_html()


@register.simple_tag(takes_context=True)
def easeo_title(context: Any, entity: Any) -> str:
    """Render just the <title> tag."""
    from easeo import SEOEntity, build_seo_payload

    seo_entity = SEOEntity(
        entity_type=getattr(entity, "entity_type", "page"),
        title=getattr(entity, "title", None),
        excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
    )
    route = context.get("request", {})
    route_path = getattr(route, "path", "/")
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    return f"<title>{payload.title}</title>"


@register.simple_tag(takes_context=True)
def easeo_meta(context: Any, entity: Any) -> str:
    """Render meta description tag."""
    from easeo import SEOEntity, build_seo_payload

    seo_entity = SEOEntity(
        entity_type=getattr(entity, "entity_type", "page"),
        title=getattr(entity, "title", None),
        excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
    )
    route = context.get("request", {})
    route_path = getattr(route, "path", "/")
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    return f'<meta name="description" content="{payload.description}" />'


def seo_head(entity: Any, route: str, config: Any = None) -> str:
    """Render SEO <head> tags (function-based API)."""
    from easeo import SEOEntity, build_seo_payload

    seo_entity = SEOEntity(
        entity_type=getattr(entity, "entity_type", "page"),
        title=getattr(entity, "title", None),
        excerpt=getattr(entity, "excerpt", None) or getattr(entity, "description", None),
    )
    if config is None:
        config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return payload.render_html()
