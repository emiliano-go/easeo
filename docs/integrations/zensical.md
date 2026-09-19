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
