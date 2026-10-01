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
    """Creates a published blog post entity.

    Args:
        title: Post title.
        body_html: Post content as HTML.
        slug: URL slug.
        author: Author display name. Omitted when empty.
        excerpt: Short description. Derived from ``body_html`` when omitted.
        breadcrumbs: Breadcrumb dicts with ``name`` and ``url`` keys.

    Returns:
        A ``post`` entity with ``status="published"``.
    """
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
    """Creates a published product entity.

    Args:
        name: Product title.
        sku: Stock keeping unit.
        price: Price, converted to a string.
        currency: ISO currency code.
        availability: Availability value, for example ``"InStock"``.
        description: Short description.
        breadcrumbs: Breadcrumb dicts with ``name`` and ``url`` keys.

    Returns:
        A ``product`` entity with ``status="published"``.
    """
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
    """Creates a published FAQ page entity from question and answer dicts.

    Args:
        questions: Dicts with ``question`` and ``answer`` keys.
        title: Page title.
        description: Short description.
        breadcrumbs: Breadcrumb dicts with ``name`` and ``url`` keys.

    Returns:
        A ``faq`` entity with ``status="published"``.
    """
    return SEOEntity(
        entity_type="faq",
        title=title,
        excerpt=description,
        faq_items=[FAQItem(question=q["question"], answer=q["answer"]) for q in questions],
        status="published",
        breadcrumbs=_breadcrumbs(breadcrumbs),
    )
