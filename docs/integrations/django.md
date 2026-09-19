---
title: "Django"
description: "Use easeo with Django template tags and settings-based configuration."
---

# Django { #django }

The Django adapter provides template tags and a function API that read their
config from Django settings.

## Prerequisites { #prerequisites }

* Python 3.10 or newer.
* Django 4.0 or newer.

## Install { #install }

```bash
pip install "easeo[django]"
```

## Configure { #configure }

```python
# settings.py
EASEO = {
    "canonical_host": "example.com",
    "public_base_url": "https://example.com",
    "site_name": "Example",
    "title_template": "{title} - Example",
}
```

If `EASEO` is missing, the adapter warns and falls back to `localhost`.

## Options { #options }

Every `SEOConfig` field is supported in `EASEO`:

| Setting key | Type |
|---|---|
| `canonical_host` | `str` (required) |
| `public_base_url` | `str` (required) |
| `site_name` | `str` |
| `title_template` | `str` |
| `url_policy` | `dict` or `URLPolicy` |
| `default_robots` | `dict` or `Robots` |
| `search_robots` | `dict` or `Robots` |
| `default_og_image` | URL string, `dict`, or `SEOImage` |
| `publisher_name`, `publisher_logo` | `str` |
| `locale`, `locale_alternate`, `twitter_site` | `str` / `list[str]` |
| `auto_generate_schema`, `emit_warnings` | `bool` |
| `search_url_template` | `str` |

## Template tags { #tags }

Register the library and call the tags:

```django
{% load easeo_tags %}
<head>
  {% easeo_head entity request.path %}
</head>
```

| Tag | Output |
|---|---|
| `{% easeo_head entity route %}` | Full `<head>` block |
| `{% easeo_title entity %}` | Just the `<title>` tag |
| `{% easeo_meta entity %}` | Just the meta description |

`easeo_title` and `easeo_meta` read the route from the request in the template
context.

## Function API { #function }

```python
from easeo.adapters.django import seo_head

html = seo_head(entity, "/blog/post")
```

Pass an explicit config as the third argument to bypass settings:

```python
from easeo import SEOConfig

html = seo_head(entity, "/blog/post", SEOConfig(...))
```

## Patterns { #patterns }

### Map a model to an entity { #entity }

For schema-rich pages (products, articles), build an `SEOEntity` explicitly. See
the [model mapping example](../examples/django.md#django-entity).

### ETag responses { #etag }

Return the payload's deterministic ETag and answer `304` when it matches. See
the [ETag example](../examples/django.md#django-etag).

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `No EASEO setting found` warning | `EASEO` is not defined in settings |
| Tags render nothing | `easeo_tags` was not loaded, or the entity lacks a title |
| Head appears escaped | The tag returns safe HTML; make sure your base template uses `{{ ... }}` not `{{ ... | escape }}` |

## Notes { #notes }

* Output is marked safe because the payload escapes its values.
* Entities need `entity_type`, `title`, and `description`; a missing
  `entity_type` defaults to `page`.

## Related { #related }

* [Django example](../examples/django.md)
* [Python API reference](../reference/python-api.md)
