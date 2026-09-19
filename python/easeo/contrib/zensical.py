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

from easeo import SEOConfig, SEOEntity, SEOOverrides, build_seo_payload
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

# Conventional social-card locations, relative to the docs directory. The
# extension uses the first one that exists when no default_og_image is set.
_OG_IMAGE_CANDIDATES = (
    "assets/og-image.png",
    "assets/social-card.png",
    "assets/social.png",
    "assets/banner.png",
    "assets/images/og-image.png",
    "assets/images/social.png",
    "overrides/og-image.png",
    "overrides/banner.png",
    "og-image.png",
)


def _png_size(path: Path) -> tuple[int, int] | None:
    """Read width/height from a PNG's IHDR chunk without extra dependencies."""
    try:
        with path.open("rb") as handle:
            header = handle.read(24)
    except OSError:
        return None
    if len(header) < 24 or header[:8] != b"\x89PNG\r\n\x1a\n" or header[12:16] != b"IHDR":
        return None
    width = int.from_bytes(header[16:20], "big")
    height = int.from_bytes(header[20:24], "big")
    if width <= 0 or height <= 0:
        return None
    return width, height


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
    if default_og_image:
        og_image = SEOImage(
            url=default_og_image,
            width=kwargs.get("default_og_image_width"),
            height=kwargs.get("default_og_image_height"),
            alt=kwargs.get("default_og_image_alt"),
        )
    else:
        og_image = None

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
        emit_warnings=kwargs.get("emit_warnings", False),
        search_url_template=kwargs.get("search_url_template"),
    )


class EaseoPreprocessor(Preprocessor):
    """Build easeo SEO payload and store rendered tags in page meta."""

    name = "easeo"

    def __init__(
        self,
        md: Any,
        seo_config: SEOConfig,
        debug_dir: str | None,
        *,
        has_default_og_image: bool = False,
        og_image_autodetect: bool = True,
        og_image_warn: bool = True,
    ) -> None:
        super().__init__(md)
        self.seo_config = seo_config
        self.debug_dir = debug_dir
        self._has_default_og_image = has_default_og_image
        self._og_image_autodetect = og_image_autodetect
        self._og_image_warn = og_image_warn
        self._og_image: SEOImage | None = None
        self._og_image_resolved = False

    def _resolve_og_image(self, project_root: str, docs_dir: str) -> SEOImage | None:
        """Return an autodetected social-card image, or None.

        Looks for a conventional file under the docs directory when no
        ``default_og_image`` was configured. Warns once when nothing is found.
        """
        if self._og_image_resolved:
            return self._og_image
        self._og_image_resolved = True

        if self._has_default_og_image or not self._og_image_autodetect:
            return None

        base = Path(project_root) if project_root else Path.cwd()
        docs_path = Path(docs_dir) if Path(docs_dir).is_absolute() else base / docs_dir

        for candidate in _OG_IMAGE_CANDIDATES:
            path = docs_path / candidate
            if not path.is_file():
                continue
            public_base = self.seo_config.to_dict().get("public_base_url") or ""
            url = f"{public_base.rstrip('/')}/{candidate}"
            size = _png_size(path)
            width, height = size if size else (None, None)
            self._og_image = SEOImage(url=url, width=width, height=height)
            return self._og_image

        if self._og_image_warn:
            warnings.warn(
                "easeo.contrib.zensical: no 'default_og_image' is configured and no "
                f"conventional social card was found under '{docs_dir}' "
                f"({', '.join(_OG_IMAGE_CANDIDATES)}). Social shares will have no "
                "preview image. Set default_og_image in zensical.toml or add "
                f"{docs_dir}/assets/og-image.png.",
                stacklevel=2,
            )
        return None

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

        docs_dir = ctx.config.get("docs_dir") or "docs"
        og_image = self._resolve_og_image(project_root, docs_dir)
        overrides = SEOOverrides(og_image=og_image) if og_image is not None else None

        payload = build_seo_payload(entity, url, self.seo_config, overrides)
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
        self._has_default_og_image = bool(kwargs.get("default_og_image"))
        self._og_image_autodetect = kwargs.pop("og_image_autodetect", True)
        self._og_image_warn = kwargs.pop("og_image_warn", True)
        self._seo_config = _build_seo_config(**kwargs)

    def extendMarkdown(self, md: Any) -> None:
        md.registerExtension(self)
        preprocessor = EaseoPreprocessor(
            md,
            self._seo_config,
            self._debug_dir,
            has_default_og_image=self._has_default_og_image,
            og_image_autodetect=self._og_image_autodetect,
            og_image_warn=self._og_image_warn,
        )
        md.preprocessors.register(preprocessor, preprocessor.name, 100)


def makeExtension(**kwargs: Any) -> EaseoExtension:
    """Factory required by Python Markdown's extension loading."""
    return EaseoExtension(**kwargs)
