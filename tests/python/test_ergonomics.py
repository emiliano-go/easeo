"""Tests for the user-facing ergonomics layer: dict-compatible payloads,
equality, hooks, factories, async building, and the exception hierarchy."""

import asyncio
import warnings

import pytest

from easeo import (
    ConfigurationError,
    EaseoError,
    EntityError,
    HookRegistry,
    SEOConfig,
    SEOEntity,
    build_seo_payload,
    build_seo_payload_async,
    from_blog_post,
    from_faq,
    from_product,
    set_executor,
)


def make_config(**kwargs):
    return SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        **kwargs,
    )


def make_entity():
    return SEOEntity(entity_type="post", title="Hello", excerpt="World")


# ── Payload ergonomics ───────────────────────────────────────────────


class TestPayloadErgonomics:
    def test_equal_to_itself_and_to_dict(self):
        config = make_config()
        entity = make_entity()
        p1 = build_seo_payload(entity, "/x", config)
        p2 = build_seo_payload(entity, "/x", config)
        assert p1 == p2
        assert p1 == p1.to_dict()

    def test_not_equal_to_different_payload(self):
        config = make_config()
        a = build_seo_payload(SEOEntity(entity_type="post", title="A"), "/x", config)
        b = build_seo_payload(SEOEntity(entity_type="post", title="B"), "/x", config)
        assert a != b

    def test_not_equal_to_unrelated_object(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        assert payload != 42
        assert not (payload == "x")

    def test_getitem(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        assert payload["title"] == "Hello"
        assert payload["og"]["type"] == "article"

    def test_getitem_missing_raises_keyerror(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        with pytest.raises(KeyError):
            payload["nope"]

    def test_get_with_default(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        assert payload.get("title") == "Hello"
        assert payload.get("nope", "fallback") == "fallback"
        assert payload.get("nope") is None

    def test_contains(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        assert "title" in payload
        assert "nope" not in payload

    def test_keys_iter_len(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        keys = list(payload)
        assert "title" in keys
        assert set(payload.keys()) == set(keys)
        assert len(payload) == len(keys)


# ── Hooks ────────────────────────────────────────────────────────────


class TestHooks:
    def test_hook_mutates_payload(self):
        hooks = HookRegistry()

        @hooks.hook("post_process")
        def add_generator(payload, entity, config):
            payload["generator"] = "easeo"
            return payload

        config = make_config(hooks=hooks)
        payload = build_seo_payload(make_entity(), "/x", config)
        assert payload["generator"] == "easeo"
        assert payload.to_dict()["generator"] == "easeo"

    def test_hook_can_override_scalar_field(self):
        hooks = HookRegistry()
        hooks.register("post_process", lambda p, e, c: {**p, "robots": "noindex,nofollow"})
        payload = build_seo_payload(make_entity(), "/x", make_config(hooks=hooks))
        assert payload.robots == "noindex,nofollow"

    def test_hooks_run_in_order_last_wins(self):
        hooks = HookRegistry()
        hooks.register("post_process", lambda p, e, c: {**p, "tag": "first"})
        hooks.register("post_process", lambda p, e, c: {**p, "tag": "second"})
        payload = build_seo_payload(make_entity(), "/x", make_config(hooks=hooks))
        assert payload["tag"] == "second"

    def test_no_hooks_config_has_no_extra(self):
        payload = build_seo_payload(make_entity(), "/x", make_config())
        assert "generator" not in payload

    def test_hooks_are_deterministic(self):
        hooks = HookRegistry()
        hooks.register("post_process", lambda p, e, c: {**p, "x": 1})
        config = make_config(hooks=hooks)
        a = build_seo_payload(make_entity(), "/x", config)
        b = build_seo_payload(make_entity(), "/x", config)
        assert a.hash() == b.hash()

    def test_hooks_scoped_per_config(self):
        hooks_a = HookRegistry()
        hooks_a.register("post_process", lambda p, e, c: {**p, "site": "A"})
        hooks_b = HookRegistry()
        hooks_b.register("post_process", lambda p, e, c: {**p, "site": "B"})
        a = build_seo_payload(make_entity(), "/x", make_config(hooks=hooks_a))
        b = build_seo_payload(make_entity(), "/x", make_config(hooks=hooks_b))
        assert a["site"] == "A"
        assert b["site"] == "B"

    def test_hook_returning_none_is_tolerated(self):
        hooks = HookRegistry()
        hooks.register("post_process", lambda p, e, c: None)
        payload = build_seo_payload(make_entity(), "/x", make_config(hooks=hooks))
        assert payload.title == "Hello"

    def test_hook_raising_propagates(self):
        hooks = HookRegistry()

        def boom(payload, entity, config):
            raise RuntimeError("hook boom")

        hooks.register("post_process", boom)
        with pytest.raises(RuntimeError, match="hook boom"):
            build_seo_payload(make_entity(), "/x", make_config(hooks=hooks))

    def test_registry_unregister_and_clear(self):
        hooks = HookRegistry()
        fn = lambda p, e, c: p  # noqa: E731
        hooks.register("post_process", fn)
        assert len(hooks) == 1
        hooks.unregister("post_process", fn)
        assert len(hooks) == 0
        hooks.register("post_process", fn)
        hooks.clear("post_process")
        assert len(hooks) == 0

    def test_register_rejects_bad_input(self):
        hooks = HookRegistry()
        with pytest.raises(ValueError):
            hooks.register("", lambda p, e, c: p)
        with pytest.raises(ValueError):
            hooks.register("post_process", "not callable")


# ── Factories ────────────────────────────────────────────────────────


class TestFactories:
    def test_from_blog_post(self):
        entity = from_blog_post(
            title="Hello",
            body_html="<p>Body</p>",
            author="Jane",
            breadcrumbs=[{"name": "Blog", "url": "/blog"}],
        )
        assert entity.entity_type == "post"

    def test_from_product_converts_price(self):
        entity = from_product("Widget", "W-1", 29.99)
        assert entity.entity_type == "product"

    def test_from_faq(self):
        entity = from_faq([{"question": "Q?", "answer": "A."}])
        assert entity.entity_type == "faq"

    def test_factories_produce_buildable_entities(self):
        config = make_config()
        for entity in (
            from_blog_post("T", "<p>B</p>"),
            from_product("W", "W-1", 1.0),
            from_faq([{"question": "Q", "answer": "A"}]),
        ):
            payload = build_seo_payload(entity, "/x", config)
            assert payload.canonical == "https://example.com/x"


# ── Async ────────────────────────────────────────────────────────────


class TestAsync:
    def test_async_matches_sync(self):
        config = make_config()
        entity = make_entity()
        sync = build_seo_payload(entity, "/x", config)
        result = asyncio.run(build_seo_payload_async(entity, "/x", config))
        assert result.hash() == sync.hash()

    def test_async_with_overrides(self):
        from easeo import SEOOverrides

        config = make_config()
        overrides = SEOOverrides(meta_title="Override")
        result = asyncio.run(
            build_seo_payload_async(make_entity(), "/x", config, overrides)
        )
        assert result.title == "Override"

    def test_set_executor_resets(self):
        set_executor(None)


# ── Exceptions ───────────────────────────────────────────────────────


class TestExceptionHierarchy:
    def test_all_errors_are_value_error(self):
        for exc in (
            EaseoError,
            ConfigurationError,
            EntityError,
        ):
            assert issubclass(exc, ValueError)

    def test_specific_errors_are_easeo_error(self):
        assert issubclass(ConfigurationError, EaseoError)
        assert issubclass(EntityError, EaseoError)

    def test_bad_config_caught_as_value_error(self):
        with pytest.raises(ValueError):
            SEOConfig(canonical_host="", public_base_url="https://example.com")

    def test_bad_config_caught_as_configuration_error(self):
        with pytest.raises(ConfigurationError):
            SEOConfig(canonical_host="", public_base_url="https://example.com")

    def test_bad_config_caught_as_easeo_error(self):
        with pytest.raises(EaseoError):
            SEOConfig(canonical_host="", public_base_url="https://example.com")


# ── Validation warnings ──────────────────────────────────────────────


class TestEmitWarnings:
    def _config(self, **kwargs):
        return SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            **kwargs,
        )

    def test_emits_missing_og_image_warning(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="D")
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            build_seo_payload(entity, "/x", self._config(emit_warnings=True))
        messages = [str(w.message) for w in caught]
        assert any("EASEO108" in m for m in messages)

    def test_no_warning_when_disabled(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="D")
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            build_seo_payload(entity, "/x", self._config(emit_warnings=False))
        assert caught == []

    def test_no_missing_image_warning_when_configured(self):
        from easeo import SEOImage

        entity = SEOEntity(entity_type="page", title="T", excerpt="D")
        config = self._config(
            emit_warnings=True,
            default_og_image=SEOImage(url="https://example.com/og.png"),
        )
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            build_seo_payload(entity, "/x", config)
        assert not any("EASEO108" in str(w.message) for w in caught)


# ── Homepage WebSite schema ──────────────────────────────────────────


class TestHomeSchema:
    def test_home_entity_maps_to_website(self):
        config = SEOConfig(
            canonical_host="example.com", public_base_url="https://example.com"
        )
        payload = build_seo_payload(SEOEntity(entity_type="home", title="Home"), "/", config)
        assert payload.schema_jsonld["@type"] == "WebSite"

    def test_search_action_emitted_when_template_set(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            search_url_template="https://example.com/?q={search_term_string}",
        )
        payload = build_seo_payload(SEOEntity(entity_type="home", title="Home"), "/", config)
        action = payload.schema_jsonld["potentialAction"]
        assert action["@type"] == "SearchAction"
        assert action["target"]["urlTemplate"] == "https://example.com/?q={search_term_string}"

    def test_no_search_action_without_template(self):
        config = SEOConfig(
            canonical_host="example.com", public_base_url="https://example.com"
        )
        payload = build_seo_payload(SEOEntity(entity_type="home", title="Home"), "/", config)
        assert "potentialAction" not in payload.schema_jsonld
