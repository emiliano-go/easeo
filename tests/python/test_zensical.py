"""Tests for Zensical Markdown extension: easeo.contrib.zensical."""

import sys
import json
import types
import warnings
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest


def _fake_png(width: int, height: int) -> bytes:
    """Minimal bytes with a valid PNG signature and IHDR for _png_size."""
    return (
        b"\x89PNG\r\n\x1a\n"
        + (13).to_bytes(4, "big")
        + b"IHDR"
        + width.to_bytes(4, "big")
        + height.to_bytes(4, "big")
    )


# -- Proper module-level mocks for external deps --

_markdown_mod = types.ModuleType("markdown")
_markdown_mod.Extension = type("Extension", (), {"__init__": lambda self, **kw: None,
                                                  "extendMarkdown": lambda self, md: None})
_markdown_preprocs = types.ModuleType("markdown.preprocessors")
_markdown_preprocs.Preprocessor = type("Preprocessor", (), {"__init__": lambda self, md: None,
                                                            "run": lambda self, lines: lines})
_markdown_ext = types.ModuleType("markdown.extensions")

_zensical_mod = types.ModuleType("zensical")
_zensical_ext = types.ModuleType("zensical.extensions")

_zensical_ctx = types.ModuleType("zensical.extensions.context")
def _fake_from_markdown(md):
    return None

_zensical_ctx.ContextPreprocessor = type(
    "ContextPreprocessor", (),
    {"from_markdown": staticmethod(_fake_from_markdown)},
)

sys.modules["markdown"] = _markdown_mod
sys.modules["markdown.preprocessors"] = _markdown_preprocs
sys.modules["markdown.extensions"] = _markdown_ext
sys.modules["zensical"] = _zensical_mod
sys.modules["zensical.extensions"] = _zensical_ext
sys.modules["zensical.extensions.context"] = _zensical_ctx


from easeo._easeo_native import ConfigurationError
from easeo.contrib import zensical as zmod
from easeo.contrib.zensical import (
    _build_seo_config,
    _extract_excerpt,
    makeExtension,
    EaseoExtension,
    EaseoPreprocessor,
)

ContextPreprocessor = zmod.ContextPreprocessor


# ---------------------------------------------------------------------------
# _extract_excerpt
# ---------------------------------------------------------------------------

class TestExtractExcerpt:
    def test_normal_text(self):
        body = "# Title\n\nThis is the first paragraph. It should be extracted.\n\nSecond paragraph."
        result = _extract_excerpt(body)
        assert result == "This is the first paragraph. It should be extracted."

    def test_empty_text(self):
        assert _extract_excerpt("") == ""

    def test_short_text(self):
        body = "# Hello\n\nHi"
        result = _extract_excerpt(body, max_chars=50)
        assert result == "Hi"

    def test_heading_removed(self):
        body = "# Title\n\nFirst paragraph."
        result = _extract_excerpt(body)
        assert "Title" not in result

    def test_whitespace_only(self):
        assert _extract_excerpt("# T\n\n   \n\n  ") == ""


# ---------------------------------------------------------------------------
# _build_seo_config
# ---------------------------------------------------------------------------

class TestBuildSeoConfig:
    def test_basic(self):
        config = _build_seo_config(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        d = config.to_dict()
        assert d["canonical_host"] == "example.com"

    def test_missing_canonical_host(self):
        with pytest.raises(ConfigurationError, match="canonical_host"):
            _build_seo_config(public_base_url="https://example.com")

    def test_missing_public_base_url(self):
        with pytest.raises(ConfigurationError, match="public_base_url"):
            _build_seo_config(canonical_host="example.com")

    def test_with_options(self):
        config = _build_seo_config(
            canonical_host="example.com",
            public_base_url="https://example.com",
            site_name="Site",
            title_template="{title} - Site",
            default_og_image="https://example.com/og.png",
            publisher_name="Pub",
            locale="en_US",
            twitter_site="@test",
            auto_generate_schema=False,
        )
        d = config.to_dict()
        assert d["site_name"] == "Site"
        assert d["auto_generate_schema"] is False

    def test_og_image_dimensions_and_alt(self):
        config = _build_seo_config(
            canonical_host="example.com",
            public_base_url="https://example.com",
            default_og_image="https://example.com/og.png",
            default_og_image_width=1200,
            default_og_image_height=630,
            default_og_image_alt="Example card",
        )
        image = config.to_dict()["default_og_image"]
        assert image["url"] == "https://example.com/og.png"
        assert image["width"] == 1200
        assert image["height"] == 630
        assert image["alt"] == "Example card"

    def test_search_url_template_passthrough(self):
        config = _build_seo_config(
            canonical_host="example.com",
            public_base_url="https://example.com",
            search_url_template="https://example.com/?q={search_term_string}",
        )
        assert config.to_dict()["search_url_template"] == (
            "https://example.com/?q={search_term_string}"
        )


# ---------------------------------------------------------------------------
# EaseoExtension
# ---------------------------------------------------------------------------

class TestEaseoExtension:
    def test_init_with_debug_dir(self):
        ext = EaseoExtension(
            canonical_host="example.com",
            public_base_url="https://example.com",
            debug_dir=".debug",
        )
        assert ext._debug_dir == ".debug"

    def test_init_without_debug_dir(self):
        ext = EaseoExtension(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        assert ext._debug_dir is None

    def test_extend_markdown(self):
        ext = EaseoExtension(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        md = MagicMock()
        ext.extendMarkdown(md)
        assert md.preprocessors.register.called


# ---------------------------------------------------------------------------
# EaseoPreprocessor._first_heading
# ---------------------------------------------------------------------------

class TestFirstHeading:
    def test_found(self):
        pre = EaseoPreprocessor(None, None, None)
        result = pre._first_heading(["# Hello World"])
        assert result == "Hello World"

    def test_strips_inline_markdown(self):
        pre = EaseoPreprocessor(None, None, None)
        result = pre._first_heading(["# **Bold** `code`"])
        assert "**" not in result

    def test_not_found(self):
        pre = EaseoPreprocessor(None, None, None)
        assert pre._first_heading(["No heading"]) == ""

    def test_empty_lines(self):
        pre = EaseoPreprocessor(None, None, None)
        assert pre._first_heading([]) == ""


# ---------------------------------------------------------------------------
# EaseoPreprocessor.run
# ---------------------------------------------------------------------------

class _FakePage:
    def __init__(self, url="/p", path="p", meta=None):
        self.url = url
        self.path = path
        self.meta = meta or {}


class _FakeContext:
    def __init__(self, page=None, config=None):
        self.page = page or _FakePage()
        self.config = config or {}


def _make_ctx(from_md=None):
    """Replacement for ContextPreprocessor.from_markdown."""
    return _FakeContext()


class TestPreprocessorRun:
    def _setup(self, meta=None, url="/p", path="p", config_=None):
        ctx = _FakeContext(page=_FakePage(url=url, path=path, meta=meta or {}),
                           config=config_ or {})
        zmod.ContextPreprocessor.from_markdown = staticmethod(lambda md: ctx)
        pre = EaseoPreprocessor(None, None, None)
        pre.seo_config = _build_seo_config(canonical_host="ex.com", public_base_url="https://ex.com")
        pre.md = MagicMock()
        return pre, ctx

    def test_with_context(self):
        pre, ctx = self._setup(meta={"title": "MyPage"})
        lines = ["body"]
        result = pre.run(lines)
        assert result is lines

    def test_title_in_meta(self):
        pre, ctx = self._setup(meta={"title": "MyPage"})
        pre.run(["body"])
        assert "MyPage" in ctx.page.meta["_seo_head"]

    def test_title_from_seo_block(self):
        pre, ctx = self._setup(meta={"seo": {"title": "SeoTitle"}})
        pre.run(["body"])
        assert "SeoTitle" in ctx.page.meta["_seo_head"]

    def test_title_from_first_heading(self):
        pre, ctx = self._setup(meta={})
        pre.run(["# Hello World", "body"])
        assert "Hello World" in ctx.page.meta["_seo_head"]

    def test_canonical_https(self):
        pre, ctx = self._setup(url="/page", meta={"title": "Page"})
        pre.run(["body"])
        assert "https://ex.com/page" in ctx.page.meta["_seo_head"]

    def test_homepage_route(self):
        pre, ctx = self._setup(url="/", path="index", meta={"title": "Home"})
        pre.run(["body"])
        assert "ex.com/" in ctx.page.meta["_seo_head"]

    def test_debug_dir_writes_json(self, tmp_path):
        pre, ctx = self._setup(
            url="/test", path="test", meta={"title": "Test"},
            config_={"root_dir": str(tmp_path)},
        )
        pre.debug_dir = str(tmp_path / "debug")
        pre.run(["body"])
        debug_file = tmp_path / "debug" / "test.json"
        assert debug_file.exists()
        data = json.loads(debug_file.read_text())
        assert data["title"] != ""

    def test_title_from_site_name(self):
        pre, ctx = self._setup(meta={})
        pre.seo_config = _build_seo_config(
            canonical_host="ex.com",
            public_base_url="https://ex.com",
            site_name="MySite",
        )
        pre.run(["no heading here"])
        assert "MySite" in ctx.page.meta["_seo_head"]


# ---------------------------------------------------------------------------
# makeExtension
# ---------------------------------------------------------------------------

class TestMakeExtension:
    def test_factory(self):
        ext = makeExtension(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        assert isinstance(ext, EaseoExtension)


# ---------------------------------------------------------------------------
# ImportError fallback
# ---------------------------------------------------------------------------

def test_context_preprocessor_import_error():
    import importlib
    import easeo.contrib.zensical as zmod
    with patch.dict(sys.modules, {"zensical.extensions.context": None}):
        importlib.reload(zmod)
        from easeo.contrib.zensical import EaseoPreprocessor as EP
        pre = EP(None, None, None)
        assert pre.run(["x"]) == ["x"]
    importlib.reload(zmod)


def test_extract_excerpt_force_cut_no_space():
    text = "a" * 200
    body = "# T\n\n" + text
    result = _extract_excerpt(body, max_chars=10)
    assert result == "a" * 10 + "..."


def test_preprocessor_run_ctx_empty_returns_lines():
    zmod.ContextPreprocessor.from_markdown = staticmethod(lambda md: None)
    pre = EaseoPreprocessor(None, None, None)
    pre.seo_config = _build_seo_config(
        canonical_host="ex.com", public_base_url="https://ex.com"
    )
    pre.md = MagicMock()
    lines = ["body"]
    assert pre.run(lines) is lines


def test_preprocessor_run_payload_none_returns_lines():
    ctx = _FakeContext(page=_FakePage(meta={"title": "Test"}))
    zmod.ContextPreprocessor.from_markdown = staticmethod(lambda md: ctx)
    pre = EaseoPreprocessor(None, None, None)
    pre.seo_config = _build_seo_config(
        canonical_host="ex.com", public_base_url="https://ex.com"
    )
    pre.md = MagicMock()
    with patch("easeo.contrib.zensical.build_seo_payload", return_value=None):
        lines = ["body"]
        assert pre.run(lines) is lines


# ---------------------------------------------------------------------------
# Open Graph image autodetect + warning
# ---------------------------------------------------------------------------

class TestOgImageHandling:
    def _pre(self, seo_config, *, autodetect=True, warn=True, has_default=False):
        pre = EaseoPreprocessor(
            None,
            seo_config,
            None,
            has_default_og_image=has_default,
            og_image_autodetect=autodetect,
            og_image_warn=warn,
        )
        pre.md = MagicMock()
        return pre

    def _ctx(self, tmp_path, *, docs_dir="docs", meta=None):
        ctx = _FakeContext(
            page=_FakePage(url="/p", path="p", meta=meta or {"title": "Page"}),
            config={"root_dir": str(tmp_path), "docs_dir": docs_dir},
        )
        zmod.ContextPreprocessor.from_markdown = staticmethod(lambda md: ctx)
        return ctx

    def test_autodetect_uses_conventional_asset(self, tmp_path):
        assets = tmp_path / "docs" / "assets"
        assets.mkdir(parents=True)
        (assets / "og-image.png").write_bytes(_fake_png(1200, 630))
        config = _build_seo_config(
            canonical_host="ex.com", public_base_url="https://ex.com"
        )
        pre = self._pre(config)
        ctx = self._ctx(tmp_path)
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            pre.run(["body"])
        head = ctx.page.meta["_seo_head"]
        assert "https://ex.com/assets/og-image.png" in head
        assert 'og:image:width" content="1200' in head
        assert 'og:image:height" content="630' in head

    def test_warns_when_missing(self, tmp_path):
        (tmp_path / "docs").mkdir()
        config = _build_seo_config(
            canonical_host="ex.com", public_base_url="https://ex.com"
        )
        pre = self._pre(config)
        self._ctx(tmp_path)
        with pytest.warns(UserWarning, match="default_og_image"):
            pre.run(["body"])

    def test_explicit_config_wins_over_autodetect(self, tmp_path):
        assets = tmp_path / "docs" / "assets"
        assets.mkdir(parents=True)
        (assets / "og-image.png").write_bytes(_fake_png(1200, 630))
        config = _build_seo_config(
            canonical_host="ex.com",
            public_base_url="https://ex.com",
            default_og_image="https://ex.com/custom.png",
        )
        pre = self._pre(config, has_default=True)
        ctx = self._ctx(tmp_path)
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            pre.run(["body"])
        head = ctx.page.meta["_seo_head"]
        assert "https://ex.com/custom.png" in head
        assert "assets/og-image.png" not in head

    def test_warning_can_be_disabled(self, tmp_path):
        (tmp_path / "docs").mkdir()
        config = _build_seo_config(
            canonical_host="ex.com", public_base_url="https://ex.com"
        )
        pre = self._pre(config, warn=False)
        self._ctx(tmp_path)
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            pre.run(["body"])

    def test_autodetect_can_be_disabled(self, tmp_path):
        assets = tmp_path / "docs" / "assets"
        assets.mkdir(parents=True)
        (assets / "og-image.png").write_bytes(_fake_png(1200, 630))
        config = _build_seo_config(
            canonical_host="ex.com", public_base_url="https://ex.com"
        )
        pre = self._pre(config, autodetect=False, warn=False)
        ctx = self._ctx(tmp_path)
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            pre.run(["body"])
        assert "assets/og-image.png" not in ctx.page.meta["_seo_head"]


class TestPngSize:
    def test_reads_dimensions(self, tmp_path):
        path = tmp_path / "card.png"
        path.write_bytes(_fake_png(800, 418))
        assert zmod._png_size(path) == (800, 418)

    def test_non_png_returns_none(self, tmp_path):
        path = tmp_path / "card.txt"
        path.write_bytes(b"not a png at all")
        assert zmod._png_size(path) is None
