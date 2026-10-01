"""Fluent builder for SEOEntity.

Pure convenience sugar over the SEOEntity constructor. No new behavior::

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
        """Creates a builder for the given entity type.

        Args:
            entity_type: One of ``home``, ``post``, ``page``, ``video``,
                ``taxonomy``, ``search``, ``product``, ``organization``,
                ``local_business``, ``faq``, or ``other``.

        Raises:
            ValueError: If ``entity_type`` is not a non-empty string.
        """
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
        """Sets the entity slug.

        Args:
            value: URL slug.

        Returns:
            The builder, for chaining.
        """
        self._fields["slug"] = value
        return self

    def title(self, value: str) -> SEOEntityBuilder:
        """Sets the page title.

        Args:
            value: Page title.

        Returns:
            The builder, for chaining.
        """
        self._fields["title"] = value
        return self

    def excerpt(self, value: str) -> SEOEntityBuilder:
        """Sets the short description used as the meta description.

        Args:
            value: Short description.

        Returns:
            The builder, for chaining.
        """
        self._fields["excerpt"] = value
        return self

    def body_html(self, value: str) -> SEOEntityBuilder:
        """Sets the full content as HTML, used to derive a snippet when no
        excerpt is set.

        Args:
            value: Content HTML.

        Returns:
            The builder, for chaining.
        """
        self._fields["body_html"] = value
        return self

    def status(self, value: str) -> SEOEntityBuilder:
        """Sets the publication status. ``"published"`` (case-insensitive)
        keeps the page indexable; any other value becomes noindex.

        Args:
            value: Publication status.

        Returns:
            The builder, for chaining.
        """
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
        """Sets the primary image for the entity.

        Args:
            url: Absolute image URL.
            width: Image width in pixels.
            height: Image height in pixels.
            alt: Alternative text.

        Returns:
            The builder, for chaining.
        """
        self._fields["featured_image"] = SEOImage(
            url, width=width, height=height, alt=alt
        )
        return self

    def published_at(self, value: str) -> SEOEntityBuilder:
        """Sets the publication date.

        Args:
            value: ISO date or datetime.

        Returns:
            The builder, for chaining.
        """
        self._fields["published_at"] = value
        return self

    def updated_at(self, value: str) -> SEOEntityBuilder:
        """Sets the last update date.

        Args:
            value: ISO date or datetime.

        Returns:
            The builder, for chaining.
        """
        self._fields["updated_at"] = value
        return self

    def author_name(self, value: str) -> SEOEntityBuilder:
        """Sets the author display name.

        Args:
            value: Author name.

        Returns:
            The builder, for chaining.
        """
        self._fields["author_name"] = value
        return self

    def sku(self, value: str) -> SEOEntityBuilder:
        """Sets the product stock keeping unit.

        Args:
            value: Product SKU.

        Returns:
            The builder, for chaining.
        """
        self._fields["sku"] = value
        return self

    def price(self, amount: str, currency: str | None = None) -> SEOEntityBuilder:
        """Sets the product price and optional currency.

        Args:
            amount: Price as a string.
            currency: ISO currency code, for example ``"USD"``.

        Returns:
            The builder, for chaining.
        """
        self._fields["price"] = amount
        if currency is not None:
            self._fields["price_currency"] = currency
        return self

    def availability(self, value: str) -> SEOEntityBuilder:
        """Sets the product availability.

        Args:
            value: Availability value, for example ``"InStock"``.

        Returns:
            The builder, for chaining.
        """
        self._fields["availability"] = value
        return self

    def address(self, value: str) -> SEOEntityBuilder:
        """Sets the postal address used in local business schemas.

        Args:
            value: Street address.

        Returns:
            The builder, for chaining.
        """
        self._fields["address"] = value
        return self

    def same_as(self, url: str) -> SEOEntityBuilder:
        """Appends a profile URL used as ``sameAs`` in organization schemas.

        Args:
            url: Profile or social URL.

        Returns:
            The builder, for chaining.
        """
        self._same_as.append(url)
        return self

    def breadcrumb(self, name: str, url: str) -> SEOEntityBuilder:
        """Appends a breadcrumb entry.

        Args:
            name: Breadcrumb label.
            url: Breadcrumb URL.

        Returns:
            The builder, for chaining.
        """
        self._breadcrumbs.append(Breadcrumb(name, url))
        return self

    def faq_item(self, question: str, answer: str) -> SEOEntityBuilder:
        """Appends a question and answer pair for FAQ schemas.

        Args:
            question: Question text.
            answer: Answer text.

        Returns:
            The builder, for chaining.
        """
        self._faq_items.append(FAQItem(question, answer))
        return self

    def build(self) -> SEOEntity:
        """Builds the entity.

        Returns:
            The immutable entity with all accumulated fields.
        """
        fields = dict(self._fields)
        if self._breadcrumbs:
            fields["breadcrumbs"] = list(self._breadcrumbs)
        if self._faq_items:
            fields["faq_items"] = list(self._faq_items)
        if self._same_as:
            fields["same_as"] = list(self._same_as)
        return SEOEntity(entity_type=self._entity_type, **fields)
