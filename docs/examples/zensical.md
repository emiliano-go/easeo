---
title: "Zensical"
description: "Zensical examples for easeo: minimal setup, a full config, social cards, and validation."
---

# Zensical { #zensical }

The `easeo.contrib.zensical` markdown extension generates SEO metadata for every
docs page at build time. This documentation site uses it.

## Simple: minimal setup { #zensical-simple }

```toml
# zensical.toml
[project.markdown_extensions]
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  site_name = "Example",
}
```

Emit the generated head from your theme override:

```html
{# docs/overrides/main.html #}
{% block site_meta %}
{% if page.meta and page.meta._seo_head %}
{{ page.meta._seo_head | safe }}
{% else %}
{{ super() }}
{% endif %}
{% endblock %}
```

## Complex: a full configuration { #zensical-full }

```toml
# zensical.toml
[project.markdown_extensions]
"easeo.contrib.zensical" = {
  canonical_host = "docs.example.com",
  public_base_url = "https://docs.example.com/",
  site_name = "Example Docs",
  title_template = "{title} - Example Docs",
  publisher_name = "Example",
  publisher_logo = "https://docs.example.com/assets/logo.png",
  locale = "en_US",
  twitter_site = "@example",
  auto_generate_schema = true,
  emit_warnings = true,

  # Social card for link previews.
  default_og_image = "https://docs.example.com/assets/og-image.png",
  default_og_image_width = 1200,
  default_og_image_height = 630,
  default_og_image_alt = "Example Docs",

  # Write a resolved payload per page for debugging.
  debug_dir = ".seo-debug",
}
```

## Complex: social card autodetection { #zensical-autodetect }

If `default_og_image` is not set, the extension looks for a conventional card
under the docs directory and uses the first match. Place one of these:

```text
docs/assets/og-image.png
docs/assets/social-card.png
docs/assets/social.png
docs/assets/banner.png
docs/assets/images/og-image.png
docs/overrides/og-image.png
docs/og-image.png
```

PNG dimensions are read automatically. If nothing is configured or found, the
build emits one warning explaining how to fix it. Disable either behavior:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  og_image_autodetect = false,
  og_image_warn = false,
}
```

## Complex: site search in the WebSite schema { #zensical-search }

When a site has a real query-parameter search endpoint, point a `SearchAction`
at it. The homepage becomes a `WebSite` node with a `SearchAction`:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  search_url_template = "https://example.com/search?q={search_term_string}",
}
```

Do not set this for Zensical's built-in search: it is a client-side overlay
with no URL, so a `SearchAction` would point nowhere.

## Complex: per-page front matter { #zensical-frontmatter }

Each page's title and description come from front matter when present, which
gives precise control over the head and social preview:

```markdown
---
title: "Shipping and returns"
description: "Delivery times, shipping costs, and the return policy."
---

# Shipping and returns
```

Resolution order:

* **Title**: front matter `title`, then `seo.title`, then the first H1, then the
  site name.
* **Description**: front matter `description`, then `seo.description`, then an
  excerpt of the page body.

## Notes { #zensical-notes }

* The extension needs `easeo` installed in the same environment as Zensical.
* `emit_warnings = true` surfaces validation issues (for example `EASEO108`,
  a missing Open Graph image) during the build.
* See the [Zensical integration page](../integrations/zensical.md) for the
  complete option list.
