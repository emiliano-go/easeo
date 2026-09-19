"""Tests for the framework adapters.

Each test skips when its framework is not installed, so the suite stays
runnable on a bare checkout. CI installs the adapter extras so these run.
"""

import importlib.util

import pytest

from easeo import SEOConfig

CONFIG = SEOConfig(canonical_host="example.com", public_base_url="https://example.com")


def _require(module: str) -> None:
    if importlib.util.find_spec(module) is None:
        pytest.skip(f"{module} not installed")


def _entity():
    class Entity:
        entity_type = "post"
        title = "Hello"
        description = "A post."

    return Entity()


# ---------------------------------------------------------------------------
# FastAPI
# ---------------------------------------------------------------------------

class TestFastAPIAdapter:
    def test_for_entity_returns_dict(self):
        _require("fastapi")
        from easeo.adapters.fastapi import EaseoSEO

        data = EaseoSEO(CONFIG).for_entity(_entity(), "/blog/hello")
        assert isinstance(data, dict)
        assert data["canonical"] == "https://example.com/blog/hello"

    def test_none_config_raises(self):
        _require("fastapi")
        from easeo.adapters.fastapi import EaseoSEO

        with pytest.raises(ValueError):
            EaseoSEO(None)

    def test_missing_entity_type_defaults_to_page(self):
        _require("fastapi")
        from easeo.adapters.fastapi import EaseoSEO

        class Bare:
            title = "Bare"
            description = "d"

        data = EaseoSEO(CONFIG).for_entity(Bare(), "/x")
        assert data["canonical"] == "https://example.com/x"


# ---------------------------------------------------------------------------
# Flask
# ---------------------------------------------------------------------------

class TestFlaskAdapter:
    def test_for_entity_returns_dict(self):
        _require("flask")
        from easeo.adapters.flask import Easeo

        data = Easeo(config=CONFIG).for_entity(_entity(), "/blog/hello")
        assert data["canonical"] == "https://example.com/blog/hello"

    def test_init_app_requires_config(self):
        _require("flask")
        flask = importlib.import_module("flask")
        from easeo.adapters.flask import Easeo

        with pytest.raises(ValueError):
            Easeo().init_app(flask.Flask("test"))

    def test_context_processor_helper(self):
        _require("flask")
        flask = importlib.import_module("flask")
        from flask import render_template_string
        from easeo.adapters.flask import Easeo

        app = flask.Flask("test")
        Easeo(app, CONFIG)
        with app.test_request_context("/blog/hello"):
            html = render_template_string(
                "{{ easeo_head(entity, '/blog/hello') }}", entity=_entity()
            )
            assert "https://example.com/blog/hello" in html


# ---------------------------------------------------------------------------
# Django
# ---------------------------------------------------------------------------

DJANGO_EASEO = {
    "canonical_host": "shop.example.com",
    "public_base_url": "https://shop.example.com",
    "site_name": "Example Shop",
    "title_template": "{title} - Example Shop",
    "url_policy": {"trailing_slash": "never", "allowed_query_params": ["page"]},
    "default_robots": {"index": True, "follow": True},
    "default_og_image": {
        "url": "https://shop.example.com/assets/og-image.png",
        "width": 1200,
        "height": 630,
        "alt": "Example Shop",
    },
    "locale": "en_US",
    "twitter_site": "@exampleshop",
    "auto_generate_schema": True,
    "search_url_template": "https://shop.example.com/search?q={search_term_string}",
}


def _configure_django():
    _require("django")
    import django
    from django.conf import settings

    if not settings.configured:
        settings.configure(EASEO=DJANGO_EASEO)
        django.setup()


class TestDjangoAdapter:
    def test_forwards_full_config(self):
        _configure_django()
        from easeo.adapters.django import _get_config

        data = _get_config().to_dict()
        assert data["site_name"] == "Example Shop"
        assert data["default_og_image"]["url"] == "https://shop.example.com/assets/og-image.png"
        assert data["default_og_image"]["width"] == 1200
        assert data["twitter_site"] == "@exampleshop"
        assert data["search_url_template"] == (
            "https://shop.example.com/search?q={search_term_string}"
        )

    def test_og_image_accepts_url_string(self):
        _configure_django()
        from easeo.adapters import django as adapter

        assert adapter._image("https://example.com/og.png").url == "https://example.com/og.png"

    def test_seo_head_function(self):
        _configure_django()
        from easeo.adapters.django import seo_head

        html = seo_head(_entity(), "/blog/hello")
        assert "https://shop.example.com/blog/hello" in html
