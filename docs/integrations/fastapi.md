---
title: "FastAPI"
description: "Use the easeo FastAPI adapter for synchronous and async routes."
---

# FastAPI { #fastapi }

The FastAPI adapter wraps an `SEOConfig` and builds payloads per route.

## Prerequisites { #prerequisites }

* Python 3.10 or newer.
* FastAPI 0.100 or newer.

## Install { #install }

```bash
pip install "easeo[fastapi]"
```

## Quick start { #usage }

```python
from fastapi import FastAPI
from easeo import SEOConfig
from easeo.adapters.fastapi import EaseoSEO

app = FastAPI()

seo = EaseoSEO(
    SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )
)

class Product:
    entity_type = "product"
    title = "Widget"
    description = "A widget."

@app.get("/products/widget")
def product():
    return seo.for_entity(Product(), "/products/widget")
```

`for_entity(entity, route)` returns a plain dict, ready for a JSON response or
a template.

## API { #api }

| Member | Description |
|---|---|
| `EaseoSEO(config)` | Wrap a config. Raises `ValueError` when it is `None` |
| `seo.config` | The wrapped config, for use with `build_seo_payload` |
| `seo.for_entity(entity, route)` | Build a payload and return it as a dict |

The adapter reads `entity_type` (defaulting to `page`), `title`, and
`excerpt` or `description` from the object it is given.

## Async endpoints { #async }

The payload build is fast and releases the GIL. If you build many payloads in a
request, or want to keep the event loop free, use the async builder:

```python
from easeo import build_seo_payload_async

@app.get("/products/{slug}")
async def product(slug: str):
    return (await build_seo_payload_async(product, f"/products/{slug}", config)).to_dict()
```

## Patterns { #patterns }

### Templates { #templates }

Build the payload and pass `render_html()` to the template. See the
[Jinja2 example](../examples/fastapi.md#fastapi-template).

### ETag caching { #etag }

Every payload has a deterministic hash, so it makes a perfect ETag. Return
`304` when the client already has the current payload. See the
[ETag example](../examples/fastapi.md#fastapi-etag).

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `ValueError: SEOConfig is required` | `EaseoSEO` was constructed with `None` |
| `for_entity` cannot find a title | The object lacks `title` or `description` attributes |
| Social preview has no image | No `default_og_image` and no entity image; see [EASEO108](../concepts/validation.md) |

## Related { #related }

* [FastAPI example](../examples/fastapi.md)
* [Python API reference](../reference/python-api.md)
