"""Tests for easeo.SEOEntityBuilder (fluent entity construction)."""

import pytest


class TestBuilderBasics:
    def test_minimal_build(self):
        from easeo import SEOEntityBuilder

        entity = SEOEntityBuilder("post").title("Hello").build()
        assert entity.entity_type == "post"
        assert entity.title == "Hello"

    def test_entity_type_required(self):
        from easeo import SEOEntityBuilder

        with pytest.raises(ValueError):
            SEOEntityBuilder("")
        with pytest.raises(ValueError):
            SEOEntityBuilder(None)

    def test_chaining_returns_self(self):
        from easeo import SEOEntityBuilder

        builder = SEOEntityBuilder("page")
        assert builder.title("T") is builder
        assert builder.excerpt("E") is builder

    def test_title_not_required(self):
        from easeo import SEOEntityBuilder

        entity = SEOEntityBuilder("page").build()
        assert entity.title is None


class TestBuilderFields:
    def test_scalar_fields(self):
        from easeo import SEOEntityBuilder

        entity = (
            SEOEntityBuilder("product")
            .slug("widget")
            .title("Widget")
            .excerpt("A useful widget.")
            .status("published")
            .published_at("2026-01-01")
            .updated_at("2026-02-01")
            .author_name("Jane")
            .sku("W-001")
            .price("29.99", currency="USD")
            .availability("in stock")
            .address("123 Main St")
            .build()
        )
        assert entity.title == "Widget"
        assert entity.entity_type == "product"

    def test_price_without_currency(self):
        from easeo import SEOEntityBuilder

        entity = SEOEntityBuilder("product").price("9.99").build()
        assert entity.title is None  # construction succeeds without currency

    def test_featured_image(self):
        from easeo import SEOEntityBuilder

        entity = (
            SEOEntityBuilder("post")
            .featured_image("https://example.com/hero.jpg", width=1200, height=630, alt="Hero")
            .build()
        )
        payload_dict = entity.to_dict()
        assert payload_dict["featured_image"]["url"] == "https://example.com/hero.jpg"
        assert payload_dict["featured_image"]["width"] == 1200
        assert payload_dict["featured_image"]["alt"] == "Hero"

    def test_breadcrumbs_accumulate(self):
        from easeo import SEOEntityBuilder

        entity = (
            SEOEntityBuilder("post")
            .breadcrumb("Home", "/")
            .breadcrumb("Blog", "/blog")
            .build()
        )
        crumbs = entity.to_dict()["breadcrumbs"]
        assert [c["name"] for c in crumbs] == ["Home", "Blog"]

    def test_faq_items_accumulate(self):
        from easeo import SEOEntityBuilder

        entity = (
            SEOEntityBuilder("faq")
            .faq_item("Q1?", "A1.")
            .faq_item("Q2?", "A2.")
            .build()
        )
        items = entity.to_dict()["faq_items"]
        assert len(items) == 2
        assert items[0]["question"] == "Q1?"

    def test_same_as_accumulates(self):
        from easeo import SEOEntityBuilder

        entity = (
            SEOEntityBuilder("organization")
            .same_as("https://twitter.com/example")
            .same_as("https://github.com/example")
            .build()
        )
        assert len(entity.to_dict()["same_as"]) == 2

    def test_empty_collections_omitted(self):
        from easeo import SEOEntityBuilder

        entity = SEOEntityBuilder("page").title("T").build()
        data = entity.to_dict()
        assert not data.get("breadcrumbs")
        assert not data.get("faq_items")
        assert not data.get("same_as")


class TestBuilderWithPayload:
    def test_built_entity_produces_payload(self):
        from easeo import SEOConfig, SEOEntityBuilder, build_seo_payload

        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        entity = (
            SEOEntityBuilder("post")
            .title("Hello World")
            .excerpt("An example post.")
            .breadcrumb("Home", "/")
            .breadcrumb("Blog", "/blog")
            .build()
        )
        payload = build_seo_payload(entity, "/blog/hello", config)
        assert payload.title == "Hello World"
        assert payload.canonical == "https://example.com/blog/hello"
        assert "BreadcrumbList" in payload.render_html()


class TestSchemaRegistry:
    def test_register_and_apply_generator(self):
        from easeo import SEOConfig, SEOEntity, SchemaRegistry, build_seo_payload

        registry = SchemaRegistry()

        @registry.register("Article")
        def article(entity, config, canonical, title, description, og_image):
            return {"@context": "https://schema.org", "@type": "PodcastEpisode", "name": title}

        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            schema_registry=registry,
        )
        entity = SEOEntity(entity_type="post", title="Ep 1")
        payload = build_seo_payload(entity, "/podcast/1", config)
        assert payload.schema_jsonld["@type"] == "PodcastEpisode"

    def test_registry_scoped_to_config(self):
        from easeo import SEOConfig, SEOEntity, SchemaRegistry, build_seo_payload

        registry = SchemaRegistry()
        registry.register("Article", lambda *a: {"@type": "Custom"})
        with_registry = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            schema_registry=registry,
        )
        plain = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        entity = SEOEntity(entity_type="post", title="T")
        assert build_seo_payload(entity, "/x", with_registry).schema_jsonld["@type"] == "Custom"
        assert build_seo_payload(entity, "/x", plain).schema_jsonld["@type"] == "Article"

    def test_unregister_restores_builtin(self):
        from easeo import SEOConfig, SEOEntity, SchemaRegistry, build_seo_payload

        registry = SchemaRegistry()
        registry.register("Article", lambda *a: {"@type": "Custom"})
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            schema_registry=registry,
        )
        entity = SEOEntity(entity_type="post", title="T")
        registry.unregister("Article")
        assert build_seo_payload(entity, "/x", config).schema_jsonld["@type"] == "Article"

    def test_has_and_list_types(self):
        from easeo import SchemaRegistry

        registry = SchemaRegistry()
        assert registry.has("Podcast") is False
        assert isinstance(registry.list_types(), list)
        registry.register("Podcast", lambda *a: {"@type": "Podcast"})
        assert registry.has("Podcast") is True
        assert registry.list_types() == ["Podcast"]

    def test_non_dict_generator_raises(self):
        from easeo import SEOConfig, SEOEntity, SchemaRegistry, build_seo_payload

        registry = SchemaRegistry()
        registry.register("Article", lambda *a: "not a dict")
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            schema_registry=registry,
        )
        with pytest.raises(ValueError, match="must return a dict"):
            build_seo_payload(SEOEntity(entity_type="post", title="T"), "/x", config)
