"""easeo adapter for Zensical SSG."""

from __future__ import annotations

from typing import Any


class EaseoExtension:
    """Zensical Markdown extension for easeo SEO generation.

    Usage in zensical.toml:

    [project.markdown_extensions]
    "easeo.contrib.zensical" = {
        canonical_host = "example.com",
        public_base_url = "https://example.com/",
        site_name = "Example",
        title_template = "{title} - Example",
        auto_generate_schema = true
    }
    """

    def __init__(self, **kwargs: Any) -> None:
        from easeo import SEOConfig

        self.config = SEOConfig(
            canonical_host=kwargs.get("canonical_host", "localhost"),
            public_base_url=kwargs.get("public_base_url", "http://localhost"),
            site_name=kwargs.get("site_name"),
            title_template=kwargs.get("title_template", "{title}"),
            auto_generate_schema=kwargs.get("auto_generate_schema", True),
        )

    def extendMarkdown(self, md: Any) -> None:
        """Register the extension with Markdown processor."""
        # Zensical calls this during markdown initialization
        pass

    def run(self, page: Any) -> None:
        """Process a page and inject SEO metadata."""
        from easeo import SEOEntity, build_seo_payload

        title = getattr(page, "title", None) or page.meta.get("title", "")
        description = getattr(page, "description", None) or page.meta.get("description", "")
        slug = getattr(page, "slug", None) or page.meta.get("slug", "")

        entity = SEOEntity(
            entity_type="page",
            title=title,
            excerpt=description,
        )

        route = f"/{slug}/" if slug else "/"
        payload = build_seo_payload(entity, route, self.config)

        # Store the SEO HTML for the template to render
        page.meta["_seo_head"] = payload.render_html()
        page.meta["_seo_payload"] = payload.to_dict()
