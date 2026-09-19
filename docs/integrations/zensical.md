---
title: "Zensical"
description: "Generate per-page SEO metadata for a Zensical docs site with easeo."
---

# Zensical { #zensical }

easeo ships a Zensical markdown extension that generates SEO metadata for
every docs page at build time. This documentation site uses it.

## Install { #install }

```bash
pip install "easeo[zensical]"
```

## Configure { #configure }

Add the extension under `[project.markdown_extensions]` in `zensical.toml`:

```toml
[project.markdown_extensions]
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  site_name = "Example",
  title_template = "{title} - Example",
  publisher_name = "Your Name",
  locale = "en_US",
  twitter_site = "@yourhandle",
  auto_generate_schema = true,
}
```

## Social card { #social-card }

Every page needs an `og:image` for link previews. Set one explicitly:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  default_og_image = "https://example.com/assets/og-image.png",
  default_og_image_width = 1200,
  default_og_image_height = 630,
  default_og_image_alt = "Example",
}
```

If `default_og_image` is not set, the extension **autodetects** a conventional
social card under the docs directory and uses the first one that exists:

```
assets/og-image.png        assets/social-card.png     assets/social.png
assets/banner.png          assets/images/og-image.png assets/images/social.png
overrides/og-image.png     overrides/banner.png       og-image.png
```

Detection resolves PNG dimensions automatically. When nothing is configured or
found, the build emits a single warning explaining how to fix it, so a
missing preview image never ships silently.

Disable either behavior with `og_image_autodetect = false` or
`og_image_warn = false`.

## Site search

For the homepage `WebSite` schema, set a search URL template to add a
`SearchAction`. Use `{search_term_string}` as the placeholder:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  search_url_template = "https://example.com/?q={search_term_string}",
}
```

Without a template, no `SearchAction` is emitted, because a search action that
points at a non-existent endpoint does more harm than good.

## Validation warnings

Set `emit_warnings = true` to surface validation issues as Python warnings
during the build. Each issue is reported with its rule id (for example
`EASEO108` for a missing Open Graph image). The full rule list is in
[Validation](../concepts/validation.md).

## Inject into the head { #inject }

The extension sets `page.meta["_seo_head"]` with the rendered tags. Emit it in
your theme override:

```html
{# overrides/main.html #}
{% block site_meta %}
{% if page.meta and page.meta._seo_head %}
{{ page.meta._seo_head | safe }}
{% else %}
{{ super() }}
{% endif %}
{% endblock %}
```

## Title and description sources { #sources }

The extension resolves each field in order:

* **Title**: front matter `title`, then `seo.title`, then the first H1, then
  the site name.
* **Description**: front matter `description`, then `seo.description`, then an
  excerpt extracted from the page body.

## Debugging { #debugging }

Set `debug_dir` to dump the resolved payload for each page:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  debug_dir = ".seo-debug",
}
```

## Notes { #notes }

* The extension needs `easeo` installed in the same environment as Zensical.
* `markdown` is pulled in by the extra.
