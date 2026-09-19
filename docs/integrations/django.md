---
title: "Django"
description: "Use easeo with Django template tags and settings-based configuration."
---

# Django { #django }

The Django adapter provides template tags and a function API that read their
config from Django settings.

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
from easeo.adapters.django import seo_head, easeo_title, easeo_meta

html = seo_head(entity, "/blog/post")
```

Pass an explicit config as the third argument to bypass settings:

```python
from easeo import SEOConfig

html = seo_head(entity, "/blog/post", SEOConfig(...))
```

## Notes { #notes }

* Output is marked safe because the payload escapes its values.
* Entities need `entity_type`, `title`, and `description`; missing
  `entity_type` defaults to `page`.
