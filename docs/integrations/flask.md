---
title: "Flask"
description: "Use the easeo Flask context processor and for_entity helper."
---

# Flask { #flask }

The Flask adapter registers a context processor and exposes a `for_entity`
helper.

## Prerequisites { #prerequisites }

* Python 3.10 or newer.
* Flask 2.3 or newer.

## Install { #install }

```bash
pip install "easeo[flask]"
```

## Quick start { #usage }

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

## API { #api }

| Member | Description |
|---|---|
| `Easeo(app, config)` | Construct and register on an app |
| `Easeo(config=config)` | Construct without an app |
| `easeo.init_app(app)` | Register on an app later. Raises `ValueError` without a config |
| `easeo.config` | The wrapped config, for use with `build_seo_payload` |
| `easeo.for_entity(entity, route)` | Build a payload and return it as a dict |

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

## Patterns { #patterns }

### App factory and blueprints { #factory }

See the [factory example](../examples/flask.md#flask-factory).

### JSON API { #json }

`for_entity` returns a dict, so it serializes directly. See the
[JSON example](../examples/flask.md#flask-json).

### ETag responses { #etag }

Return the payload's deterministic ETag and answer `304` when it matches. See
the [ETag example](../examples/flask.md#flask-etag).

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `config must be provided before init_app` | `Easeo()` was created without a config |
| Helper missing in templates | `init_app` did not run, so the context processor is not registered |
| Head appears escaped | The helper returns `Markup`; avoid an extra `| escape` filter |

## Notes { #notes }

* `for_entity` returns a dict; the template helper returns HTML.
* Entities need `entity_type`, `title`, and `description`; a missing
  `entity_type` defaults to `page`.

## Related { #related }

* [Flask example](../examples/flask.md)
* [Python API reference](../reference/python-api.md)
