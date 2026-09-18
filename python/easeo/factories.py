"""Domain-specific factory functions for :class:`SEOEntity`.

These are pure convenience constructors — no new behavior, just less
ceremony for the common content types.
"""

from __future__ import annotations

from typing import Any

from easeo._easeo_native import Breadcrumb, FAQItem, SEOEntity

__all__ = ["from_blog_post", "from_product", "from_faq"]


def _breadcrumbs(items: list[dict[str, str]] | None) -> list[Breadcrumb] | None:
    if not items:
        return None
    return [Breadcrumb(name=b["name"], url=b["url"]) for b in items]


def from_blog_post(
    title: str,
    body_html: str,
    slug: str | None = None,
    author: str = "",
    excerpt: str | None = None,
    breadcrumbs: list[dict[str, str]] | None = None,
) -> SEOEntity:
    """Create a published blog post entity."""
    return SEOEntity(
        entity_type="post",
        title=title,
        body_html=body_html,
        slug=slug,
        author_name=author or None,
        excerpt=excerpt,
        status="published",
        breadcrumbs=_breadcrumbs(breadcrumbs),
    )


def from_product(
    name: str,
    sku: str,
    price: str | float,
    currency: str = "USD",
    availability: str = "InStock",
    description: str | None = None,
    breadcrumbs: list[dict[str, str]] | None = None,
) -> SEOEntity:
    """Create a published product entity."""
    return SEOEntity(
        entity_type="product",
        title=name,
        sku=sku,
        price=str(price),
        price_currency=currency,
        availability=availability,
        excerpt=description,
        status="published",
        breadcrumbs=_breadcrumbs(breadcrumbs),
    )


def from_faq(
    questions: list[dict[str, Any]],
    title: str = "FAQ",
    description: str | None = None,
    breadcrumbs: list[dict[str, str]] | None = None,
) -> SEOEntity:
    """Create a published FAQ page entity from question/answer dicts."""
    return SEOEntity(
        entity_type="faq",
        title=title,
        excerpt=description,
        faq_items=[FAQItem(question=q["question"], answer=q["answer"]) for q in questions],
        status="published",
        breadcrumbs=_breadcrumbs(breadcrumbs),
    )
