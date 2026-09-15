"""easeo adapter for Django."""

from __future__ import annotations

import html
import warnings
from typing import Any

from django import template
from django.conf import settings
from django.utils.safestring import mark_safe

register = template.Library()


def _get_config():
    """Get SEO config from Django settings."""
    from easeo import SEOConfig

    config_data = getattr(settings, "EASEO", None)
    if not config_data:
        warnings.warn(
            "easeo.adapters.django: No EASEO setting found. "
            "Add EASEO = {'canonical_host': '...', 'public_base_url': '...'} to your Django settings.",
            stacklevel=2,
        )
    config_data = config_data or {}
    return SEOConfig(
        canonical_host=config_data.get("canonical_host", "localhost"),
        public_base_url=config_data.get("public_base_url", "http://localhost"),
        site_name=config_data.get("site_name"),
        title_template=config_data.get("title_template"),
    )


def _get_route(context: Any) -> str:
    """Extract route path from Django template context."""
    route = context.get("request")
    if route is None:
        return "/"
    return getattr(route, "path", "/")


@register.simple_tag(takes_context=True)
def easeo_head(context: Any, entity: Any, route: str) -> str:
    """Render SEO <head> tags for a Django template.

    Usage::

        {% load easeo_tags %}
        <head>
            {% easeo_head entity request.path %}
        </head>
    """
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return mark_safe(payload.render_html())


@register.simple_tag(takes_context=True)
def easeo_title(context: Any, entity: Any) -> str:
    """Render just the <title> tag."""
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    route_path = _get_route(context)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    return mark_safe(f"<title>{html.escape(payload.title)}</title>")


@register.simple_tag(takes_context=True)
def easeo_meta(context: Any, entity: Any) -> str:
    """Render meta description tag."""
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    route_path = _get_route(context)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    escaped = html.escape(payload.description, quote=True)
    return mark_safe(f'<meta name="description" content="{escaped}" />')


def seo_head(entity: Any, route: str, config: Any = None) -> str:
    """Render SEO <head> tags (function-based API)."""
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    if config is None:
        config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return payload.render_html()
