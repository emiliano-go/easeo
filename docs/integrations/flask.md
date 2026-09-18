---
title: "Flask"
---

# Flask { #flask }

The Flask adapter registers a context processor and exposes a `for_entity`
helper.

## Install { #install }

```bash
pip install "easeo[flask]"
```

## Usage { #usage }

```python
from flask import Flask
from easeo import SEOConfig
from easeo.adapters.flask import Easeo

app = Flask(__name__)

easeo = Easeo(
    app,
    SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )
)
```

## Templates { #templates }

The adapter registers a `seo_head(entity, route)` helper:

```jinja
<head>
  {{ easeo_head(entity, request.path) | safe }}
</head>
```

The return value is `Markup`, so the `| safe` filter is optional.

## Direct use { #direct }

```python
payload = easeo.for_entity(entity, "/blog/post")
# returns a plain dict
```

## Deferred init { #deferred }

For application factories, construct without an app and initialize later:

```python
easeo = Easeo(config=config)
easeo.init_app(app)
```

`init_app` without a config raises `ValueError`.

## Notes { #notes }

* `for_entity` returns a dict; the template helper returns HTML.
* Entities need `entity_type`, `title`, and `description`; missing
  `entity_type` defaults to `page`.
