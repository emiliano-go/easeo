"""Fluent builder for SEOEntity.

Pure convenience sugar over the SEOEntity constructor — no new behavior::

    from easeo import SEOEntityBuilder

    entity = (
        SEOEntityBuilder("post")
        .title("Hello World")
        .excerpt("An example post.")
        .featured_image("https://example.com/hero.jpg", width=1200, height=630)
        .breadcrumb("Home", "/")
        .breadcrumb("Blog", "/blog")
        .build()
    )
"""

from __future__ import annotations

from easeo._easeo_native import Breadcrumb, FAQItem, SEOEntity, SEOImage

__all__ = ["SEOEntityBuilder"]


class SEOEntityBuilder:
    """Fluent builder for :class:`SEOEntity`.

    Every method returns ``self`` so calls can be chained; ``build()``
    produces the immutable entity. ``entity_type`` is required and must be
    a non-empty string.
    """

    def __init__(self, entity_type: str) -> None:
        if not isinstance(entity_type, str) or not entity_type.strip():
            raise ValueError(
                f"entity_type must be a non-empty string, got {entity_type!r}"
            )
        self._entity_type = entity_type
        self._fields: dict = {}
        self._breadcrumbs: list[Breadcrumb] = []
        self._faq_items: list[FAQItem] = []
        self._same_as: list[str] = []

    def slug(self, value: str) -> SEOEntityBuilder:
        self._fields["slug"] = value
        return self

    def title(self, value: str) -> SEOEntityBuilder:
        self._fields["title"] = value
        return self

    def excerpt(self, value: str) -> SEOEntityBuilder:
        self._fields["excerpt"] = value
        return self

    def body_html(self, value: str) -> SEOEntityBuilder:
        self._fields["body_html"] = value
        return self

    def status(self, value: str) -> SEOEntityBuilder:
        self._fields["status"] = value
        return self

    def featured_image(
        self,
        url: str,
        *,
        width: int | None = None,
        height: int | None = None,
        alt: str | None = None,
    ) -> SEOEntityBuilder:
        self._fields["featured_image"] = SEOImage(
            url, width=width, height=height, alt=alt
        )
        return self

    def published_at(self, value: str) -> SEOEntityBuilder:
        self._fields["published_at"] = value
        return self

    def updated_at(self, value: str) -> SEOEntityBuilder:
        self._fields["updated_at"] = value
        return self

    def author_name(self, value: str) -> SEOEntityBuilder:
        self._fields["author_name"] = value
        return self

    def sku(self, value: str) -> SEOEntityBuilder:
        self._fields["sku"] = value
        return self

    def price(self, amount: str, currency: str | None = None) -> SEOEntityBuilder:
        self._fields["price"] = amount
        if currency is not None:
            self._fields["price_currency"] = currency
        return self

    def availability(self, value: str) -> SEOEntityBuilder:
        self._fields["availability"] = value
        return self

    def address(self, value: str) -> SEOEntityBuilder:
        self._fields["address"] = value
        return self

    def same_as(self, url: str) -> SEOEntityBuilder:
        self._same_as.append(url)
        return self

    def breadcrumb(self, name: str, url: str) -> SEOEntityBuilder:
        self._breadcrumbs.append(Breadcrumb(name, url))
        return self

    def faq_item(self, question: str, answer: str) -> SEOEntityBuilder:
        self._faq_items.append(FAQItem(question, answer))
        return self

    def build(self) -> SEOEntity:
        fields = dict(self._fields)
        if self._breadcrumbs:
            fields["breadcrumbs"] = list(self._breadcrumbs)
        if self._faq_items:
            fields["faq_items"] = list(self._faq_items)
        if self._same_as:
            fields["same_as"] = list(self._same_as)
        return SEOEntity(entity_type=self._entity_type, **fields)
