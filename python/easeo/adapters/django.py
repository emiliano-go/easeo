"""easeo adapter for Django."""

from __future__ import annotations

import html
import warnings
from typing import Any

from django import template
from django.conf import settings
from django.utils.safestring import mark_safe

register = template.Library()


def _image(value: Any):
    """Accept an SEOImage, a URL string, or a mapping for default_og_image."""
    from easeo import SEOImage

    if value is None or isinstance(value, SEOImage):
        return value
    if isinstance(value, str):
        return SEOImage(url=value)
    if isinstance(value, dict):
        return SEOImage(**value)
    raise ValueError("EASEO['default_og_image'] must be a URL, dict, or SEOImage")


def _robots(value: Any):
    """Accept a Robots instance or a mapping such as {"index": True}."""
    from easeo import Robots

    if value is None or isinstance(value, Robots):
        return value
    if isinstance(value, dict):
        return Robots(**value)
    raise ValueError("EASEO robots settings must be a dict or Robots instance")


def _policy(value: Any):
    """Accept a URLPolicy instance or a mapping of policy fields."""
    from easeo import URLPolicy

    if value is None or isinstance(value, URLPolicy):
        return value
    if isinstance(value, dict):
        return URLPolicy(**value)
    raise ValueError("EASEO['url_policy'] must be a dict or URLPolicy instance")


def _get_config():
    """Build an SEOConfig from the ``EASEO`` Django setting.

    Every ``SEOConfig`` field is supported. ``default_og_image`` accepts a URL
    string, a mapping, or an ``SEOImage``. ``url_policy``, ``default_robots``
    and ``search_robots`` accept mappings or their value types.
    """
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
        url_policy=_policy(config_data.get("url_policy")),
        default_robots=_robots(config_data.get("default_robots")),
        search_robots=_robots(config_data.get("search_robots")),
        default_og_image=_image(config_data.get("default_og_image")),
        publisher_name=config_data.get("publisher_name"),
        publisher_logo=config_data.get("publisher_logo"),
        locale=config_data.get("locale"),
        locale_alternate=config_data.get("locale_alternate"),
        twitter_site=config_data.get("twitter_site"),
        auto_generate_schema=config_data.get("auto_generate_schema", True),
        emit_warnings=config_data.get("emit_warnings", False),
        search_url_template=config_data.get("search_url_template"),
    )


def _get_route(context: Any) -> str:
    """Extract route path from Django template context."""
    route = context.get("request")
    if route is None:
        return "/"
    return getattr(route, "path", "/")


@register.simple_tag(takes_context=True)
def easeo_head(context: Any, entity: Any, route: str) -> str:
    """Renders SEO head tags for a Django template.

    Usage::

        {% load easeo_tags %}
        <head>
            {% easeo_head entity request.path %}
        </head>

    Args:
        context: Template context, provided by the tag.
        entity: Object exposing ``entity_type``, ``title``, and ``excerpt``
            or ``description`` attributes.
        route: Route path for the page.

    Returns:
        Safe HTML for the full head snippet.
    """
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return mark_safe(payload.render_html())


@register.simple_tag(takes_context=True)
def easeo_title(context: Any, entity: Any) -> str:
    """Renders just the title tag.

    Args:
        context: Template context, provided by the tag.
        entity: Object exposing ``entity_type``, ``title``, and ``excerpt``
            or ``description`` attributes.

    Returns:
        Safe HTML for the title tag.
    """
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    route_path = _get_route(context)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    return mark_safe(f"<title>{html.escape(payload.title)}</title>")


@register.simple_tag(takes_context=True)
def easeo_meta(context: Any, entity: Any) -> str:
    """Renders the meta description tag.

    Args:
        context: Template context, provided by the tag.
        entity: Object exposing ``entity_type``, ``title``, and ``excerpt``
            or ``description`` attributes.

    Returns:
        Safe HTML for the meta description tag.
    """
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    route_path = _get_route(context)
    config = _get_config()
    payload = build_seo_payload(seo_entity, route_path, config)
    escaped = html.escape(payload.description, quote=True)
    return mark_safe(f'<meta name="description" content="{escaped}" />')


def seo_head(entity: Any, route: str, config: Any = None) -> str:
    """Renders the SEO head snippet outside a template tag.

    Args:
        entity: Object exposing ``entity_type``, ``title``, and ``excerpt``
            or ``description`` attributes.
        route: Route path for the page.
        config: Optional configuration. Falls back to the ``EASEO`` Django
            setting when omitted.

    Returns:
        The rendered head HTML.
    """
    from easeo import build_seo_payload
    from easeo.adapters._common import build_entity

    seo_entity = build_entity(entity)
    if config is None:
        config = _get_config()
    payload = build_seo_payload(seo_entity, route, config)
    return payload.render_html()
