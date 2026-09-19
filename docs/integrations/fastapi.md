---
title: "FastAPI"
description: "Use the easeo FastAPI adapter for synchronous and async routes."
---

# FastAPI { #fastapi }

The FastAPI adapter wraps an `SEOConfig` and builds payloads per route.

## Install { #install }

```bash
pip install "easeo[fastapi]"
```

## Usage { #usage }

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

@app.get("/products/{slug}")
def product(slug: str):
    return seo.for_entity(product, f"/products/{slug}")
```

`for_entity(entity, route)` returns a plain dict, ready for a JSON response or
a template.

## Async endpoints { #async }

The payload build is fast and releases the GIL. If you build many payloads in a
request, or want to keep the event loop free, use the async builder:

```python
from easeo import build_seo_payload_async

@app.get("/products/{slug}")
async def product(slug: str):
    return (await build_seo_payload_async(product, f"/products/{slug}", config)).to_dict()
```

## Notes { #notes }

* `EaseoSEO(None)` raises `ValueError`; a config is required.
* `for_entity` accepts any object with `entity_type`, `title`, and
  `description` or `excerpt` attributes. Missing `entity_type` defaults to
  `page`.
