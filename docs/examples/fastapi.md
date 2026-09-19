---
title: "FastAPI"
description: "FastAPI examples for easeo: JSON responses, Jinja2 templates, async routes, and ETag caching."
---

# FastAPI { #fastapi }

The FastAPI adapter wraps an `SEOConfig` and builds a payload per route.

## Simple: a JSON response { #fastapi-simple }

```python
from fastapi import FastAPI
from easeo import SEOConfig
from easeo.adapters.fastapi import EaseoSEO

app = FastAPI()
seo = EaseoSEO(SEOConfig(canonical_host="example.com", public_base_url="https://example.com"))

class Product:
    entity_type = "product"
    title = "Widget"
    description = "A widget."

@app.get("/products/widget")
def widget():
    return seo.for_entity(Product(), "/products/widget")
```

## Complex: Jinja2 template { #fastapi-template }

`for_entity` returns a dict, so it drops straight into a template context.

```python
from fastapi import FastAPI, Request
from fastapi.templating import Jinja2Templates
from easeo import SEOConfig, SEOEntity, build_seo_payload

app = FastAPI()
templates = Jinja2Templates(directory="templates")

config = SEOConfig(
    canonical_host="shop.example.com",
    public_base_url="https://shop.example.com",
    site_name="Example Shop",
    title_template="{title} - Example Shop",
)

def entity_from_product(product) -> SEOEntity:
    return SEOEntity(
        entity_type="product",
        title=product.name,
        excerpt=product.short_description,
        featured_image=product.image_url,
        sku=product.sku,
        price=str(product.price),
        price_currency=product.currency,
        availability="InStock" if product.in_stock else "OutOfStock",
    )

@app.get("/products/{slug}")
def product_page(slug: str, request: Request):
    product = get_product(slug)  # your data source
    payload = build_seo_payload(entity_from_product(product), f"/products/{slug}", config)
    return templates.TemplateResponse(
        "product.html",
        {"request": request, "product": product, "seo_html": payload.render_html()},
    )
```

```jinja
{# templates/product.html #}
<head>
  {{ seo_html | safe }}
</head>
```

`EaseoSEO.for_entity` returns a dict for JSON responses; for templates, build
the payload with `build_seo_payload` and pass `render_html()`.

## Complex: async route { #fastapi-async }

The build releases the GIL. Use `build_seo_payload_async` when a request builds
several payloads:

```python
from easeo import build_seo_payload_async

@app.get("/search")
async def search(q: str):
    payload = await build_seo_payload_async(
        SEOEntity(entity_type="search", title=f"Search: {q}"),
        f"/search?q={q}",
        config,
    )
    return payload.to_dict()
```

## Complex: ETag caching { #fastapi-etag }

Every payload has a deterministic hash, so it makes a perfect ETag. Return
`304 Not Modified` when the client already has the current payload.

```python
from fastapi import Header, Response

@app.get("/products/{slug}")
def product_page(slug: str, response: Response, if_none_match: str | None = Header(default=None)):
    product = get_product(slug)
    payload = build_seo_payload(entity_from_product(product), f"/products/{slug}", config)
    etag = payload.etag()

    if if_none_match == etag:
        return Response(status_code=304)

    response.headers["ETag"] = etag
    response.headers["Cache-Control"] = "public, max-age=300"
    return payload.to_dict()
```

## Notes { #fastapi-notes }

* `EaseoSEO(None)` raises `ValueError`; a config is required.
* `for_entity` accepts any object with `entity_type`, `title`, and
  `description` or `excerpt` attributes. A missing `entity_type` defaults to
  `page`.
* See the [FastAPI integration page](../integrations/fastapi.md).
