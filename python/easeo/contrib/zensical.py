"""Markdown extension for Zensical: uses easeo to generate SEO metadata.

Register in ``zensical.toml`` under ``[project.markdown_extensions]``::

    [project.markdown_extensions.easeo.contrib.zensical]
    canonical_host = "yoursite.com"
    public_base_url = "https://yoursite.com/"
    site_name = "Your Site"
    title_template = "{title} - Your Site"
    default_og_image = "https://yoursite.com/icon.png"
    publisher_name = "Your Name"
    locale = "en_US"
    twitter_site = "@yourhandle"
    auto_generate_schema = true
    debug_dir = ".seo-debug"

The extension runs during Zensical's Markdown rendering pipeline.  It sets
``page.meta[\"_seo_head\"]`` on every page so your ``overrides/main.html``
template can inject the rendered tags into ``<head>``.
"""

from __future__ import annotations

import json
import re
import warnings
from pathlib import Path
from typing import Any

from markdown import Extension
from markdown.preprocessors import Preprocessor

from easeo import SEOConfig, SEOEntity, build_seo_payload
from easeo._easeo_native import ConfigurationError, SEOImage, URLPolicy

try:
    from zensical.extensions.context import ContextPreprocessor
except (ImportError, AttributeError):
    ContextPreprocessor = None
    import warnings
    warnings.warn(
        "easeo.contrib.zensical: zensical.extensions.context not available. "
        "The easeo zensical extension will not generate SEO metadata. "
        "Install zensical to enable SEO generation.",
        stacklevel=2,
    )


_HEADING_RE = re.compile(r"^#\s+(.+)", re.MULTILINE)


def _extract_excerpt(body: str, max_chars: int = 160) -> str:
    body = re.sub(r"^#\s+.*\n?", "", body, count=1).strip()
    para = re.split(r"\n\s*\n", body, maxsplit=1)[0].strip()
    clean = re.sub(r"[`*_\[\]()>|#{}]", "", para)
    clean = re.sub(r"\s+", " ", clean).strip()
    if not clean:
        return ""
    if len(clean) <= max_chars:
        return clean
    break_at = clean.rfind(" ", 0, max_chars)
    return clean[:break_at] + "..." if break_at > 0 else clean[:max_chars] + "..."


def _build_seo_config(**kwargs: Any) -> SEOConfig:
    canonical_host = kwargs.get("canonical_host")
    public_base_url = kwargs.get("public_base_url")
    if not canonical_host:
        raise ConfigurationError(
            "easeo.contrib.zensical: 'canonical_host' is required"
        )
    if not public_base_url:
        raise ConfigurationError(
            "easeo.contrib.zensical: 'public_base_url' is required"
        )

    default_og_image = kwargs.get("default_og_image")
    og_image = SEOImage(url=default_og_image) if default_og_image else None

    trailing_slash = kwargs.get("trailing_slash", "always")
    url_policy = URLPolicy(
        enforce_https=True,
        lowercase_paths=True,
        trailing_slash=trailing_slash,
        collapse_duplicate_slashes=True,
        strip_tracking_params=True,
    )

    return SEOConfig(
        canonical_host=canonical_host,
        public_base_url=public_base_url,
        url_policy=url_policy,
        site_name=kwargs.get("site_name"),
        title_template=kwargs.get("title_template", "{title}"),
        default_og_image=og_image,
        publisher_name=kwargs.get("publisher_name"),
        locale=kwargs.get("locale"),
        twitter_site=kwargs.get("twitter_site"),
        auto_generate_schema=kwargs.get("auto_generate_schema", True),
    )


class EaseoPreprocessor(Preprocessor):
    """Build easeo SEO payload and store rendered tags in page meta."""

    name = "easeo"

    def __init__(self, md: Any, seo_config: SEOConfig, debug_dir: str | None) -> None:
        super().__init__(md)
        self.seo_config = seo_config
        self.debug_dir = debug_dir

    def _first_heading(self, lines: list[str]) -> str:
        for line in lines:
            m = _HEADING_RE.match(line)
            if m:
                return re.sub(r"[`*_]", "", m.group(1).strip())
        return ""

    def run(self, lines: list[str]) -> list[str]:
        if ContextPreprocessor is None:
            return lines

        ctx = ContextPreprocessor.from_markdown(self.md)
        if not ctx:
            return lines

        page = ctx.page
        project_root = ctx.config.get("root_dir", "")
        url = getattr(page, "url", None) or "/"
        if not url.endswith("/"):
            url += "/"

        # Title: frontmatter title, or seo.title (pre-build compat), or first H1
        title = page.meta.get("title", "") or ""
        if not title:
            seo_block = page.meta.get("seo", {})
            if isinstance(seo_block, dict):
                title = seo_block.get("title", "") or ""
        if not title:
            title = self._first_heading(lines)
        if not title:
            title = self.seo_config.to_dict().get("site_name") or ""

        # Description: frontmatter, seo.description, or excerpt from body
        description = page.meta.get("description", "") or ""
        if not description:
            seo_block = page.meta.get("seo", {})
            if isinstance(seo_block, dict):
                description = seo_block.get("description", "") or ""
        if not description and lines:
            description = _extract_excerpt("\n".join(lines))

        page_path = getattr(page, "path", "") or ""
        is_home = url.rstrip("/") == "" or page_path.rstrip("/").endswith("index")

        entity = SEOEntity(
            entity_type="home" if is_home else "page",
            title=title,
            excerpt=description or None,
        )

        payload = build_seo_payload(entity, url, self.seo_config)
        if payload is None:
            return lines

        page.meta["_seo_head"] = payload.render_html()

        if self.debug_dir and project_root:
            try:
                slug = page_path.strip("/").replace("/", "_") or "index"
                debug_path = Path(project_root) / self.debug_dir / f"{slug}.json"
                debug_path.parent.mkdir(parents=True, exist_ok=True)
                debug_path.write_text(
                    json.dumps(payload.to_dict(), indent=2, ensure_ascii=False)
                )
            except OSError as e:
                warnings.warn(
                    f"easeo.contrib.zensical: failed to write debug output: {e}",
                    stacklevel=2,
                )

        return lines


class EaseoExtension(Extension):
    """Zensical Markdown extension that uses easeo for SEO metadata.

    Register in ``zensical.toml`` under ``[project.markdown_extensions]``.
    Stores ``page.meta[\"_seo_head\"]`` with the full rendered HTML.
    """

    name = "easeo.contrib.zensical"

    def __init__(self, **kwargs: Any) -> None:
        super().__init__()
        self._debug_dir = kwargs.pop("debug_dir", None)
        self._seo_config = _build_seo_config(**kwargs)

    def extendMarkdown(self, md: Any) -> None:
        md.registerExtension(self)
        preprocessor = EaseoPreprocessor(md, self._seo_config, self._debug_dir)
        md.preprocessors.register(preprocessor, preprocessor.name, 100)


def makeExtension(**kwargs: Any) -> EaseoExtension:
    """Factory required by Python Markdown's extension loading."""
    return EaseoExtension(**kwargs)
