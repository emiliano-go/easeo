---
title: "Zensical"
description: "Generate per-page SEO metadata for a Zensical docs site with easeo."
---

# Zensical { #zensical }

easeo ships a Zensical markdown extension that generates SEO metadata for
every docs page at build time. This documentation site uses it.

## Prerequisites { #prerequisites }

* Python 3.10 or newer.
* Zensical installed in the same environment, so the extension can import it.

## Install { #install }

```bash
pip install "easeo[zensical]"
```

## Quick start { #configure }

Add the extension under `[project.markdown_extensions]` in `zensical.toml`:

```toml
[project.markdown_extensions]
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  site_name = "Example",
  title_template = "{title} - Example",
  twitter_site = "@yourhandle",
  auto_generate_schema = true,
}
```

## Options { #options }

| Option | Type | Default | Description |
|---|---|---|---|
| `canonical_host` | `str` | required | Hostname only, no scheme or path |
| `public_base_url` | `str` | required | Absolute site URL used to resolve canonicals |
| `site_name` | `str` | `None` | `og:site_name` and the last title fallback |
| `title_template` | `str` | `"{title}"` | Title pattern; must contain `{title}` |
| `publisher_name` | `str` | `None` | Publisher name in the JSON-LD |
| `publisher_logo` | `str` | `None` | Publisher logo URL in the JSON-LD |
| `locale` | `str` | `None` | `og:locale` |
| `twitter_site` | `str` | `None` | `twitter:site` |
| `auto_generate_schema` | `bool` | `true` | Generate JSON-LD from the page |
| `emit_warnings` | `bool` | `false` | Emit validation warnings during the build |
| `search_url_template` | `str` | `None` | Adds a `SearchAction` to the homepage `WebSite` node |
| `default_og_image` | `str` | `None` | Social card URL |
| `default_og_image_width` | `int` | `None` | `og:image:width` |
| `default_og_image_height` | `int` | `None` | `og:image:height` |
| `default_og_image_alt` | `str` | `None` | `og:image:alt` |
| `og_image_autodetect` | `bool` | `true` | Find a conventional card when none is set |
| `og_image_warn` | `bool` | `true` | Warn once when no card is configured or found |
| `trailing_slash` | `str` | `"always"` | URL policy for canonical paths |
| `debug_dir` | `str` | `None` | Write a resolved payload per page |

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

Suppress the theme's own `<title>` block so only the generated one is emitted:

```html
{% block htmltitle %}
{% if not (page.meta and page.meta._seo_head) %}
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

```text
assets/og-image.png        assets/social-card.png     assets/social.png
assets/banner.png          assets/images/og-image.png assets/images/social.png
overrides/og-image.png     overrides/banner.png       og-image.png
```

Detection resolves PNG dimensions automatically. When nothing is configured or
found, the build emits a single warning explaining how to fix it, so a
missing preview image never ships silently.

Disable either behavior with `og_image_autodetect = false` or
`og_image_warn = false`.

## Site search { #search }

For the homepage `WebSite` schema, a search URL template adds a `SearchAction`.
Use `{search_term_string}` as the placeholder:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  search_url_template = "https://example.com/search?q={search_term_string}",
}
```

Only set this when the URL actually runs a search, because a `SearchAction`
that points nowhere does more harm than good. Without a template, no
`SearchAction` is emitted.

!!! note "Zensical's built-in search"

    Zensical search is a client-side overlay with no URL endpoint, so it has
    nothing to point a `SearchAction` at. Leave `search_url_template` unset
    unless you run a separate server-side search.

## Validation warnings { #validation }

Set `emit_warnings = true` to surface validation issues as Python warnings
during the build. Each issue is reported with its rule id (for example
`EASEO108` for a missing Open Graph image). The full rule list is in
[Validation](../concepts/validation.md).

## Debugging { #debugging }

Set `debug_dir` to dump the resolved payload for each page:

```toml
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  debug_dir = ".seo-debug",
}
```

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `No module named 'easeo'` | `easeo` is not installed in Zensical's environment |
| `zensical.extensions.context not available` warning | Zensical is not importable; SEO tags are skipped |
| Two `<title>` tags | The theme's `htmltitle` block is not suppressed |
| No `og:image` | No `default_og_image` and no conventional card; see the build warning |
| `SearchAction` points nowhere | `search_url_template` is set but the site has no server search |

## Related { #related }

* [Zensical example](../examples/zensical.md)
* [Validation](../concepts/validation.md)
* [Deploying the Docs](../about/deploying-docs.md)
