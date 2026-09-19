---
title: "Flask"
description: "Flask examples for easeo: context processor, app factory, JSON API, and ETag responses."
---

# Flask { #flask }

The Flask adapter registers a `seo_head` template helper and a `for_entity`
method that returns a dict.

## Simple: one page { #flask-simple }

```python
from flask import Flask, request, render_template
from easeo import SEOConfig
from easeo.adapters.flask import Easeo

app = Flask(__name__)
easeo = Easeo(
    app,
    SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        site_name="Example",
    ),
)

class Post:
    entity_type = "post"
    title = "Hello"
    description = "A post."

@app.get("/blog/hello")
def post():
    return render_template("post.html", entity=Post())
```

```jinja
{# templates/post.html #}
<head>
  {{ easeo_head(entity, request.path) | safe }}
</head>
```

## Complex: app factory and blueprints { #flask-factory }

Construct the adapter without an app, then initialize it once the app exists.
Every blueprint shares the same config.

```python
# app.py
from flask import Flask
from easeo import SEOConfig
from easeo.adapters.flask import Easeo

easeo = Easeo(
    config=SEOConfig(
        canonical_host="shop.example.com",
        public_base_url="https://shop.example.com",
        site_name="Example Shop",
        title_template="{title} - Example Shop",
        default_og_image="https://shop.example.com/assets/og-image.png",
    )
)

def create_app():
    app = Flask(__name__)
    easeo.init_app(app)

    from .views import shop
    app.register_blueprint(shop)

    return app
```

## Complex: a JSON API { #flask-json }

`for_entity` returns a plain dict, which serializes directly:

```python
from flask import Blueprint, jsonify
from easeo import SEOEntity

shop = Blueprint("shop", __name__)

def entity_from_product(product) -> SEOEntity:
    return SEOEntity(
        entity_type="product",
        title=product.name,
        excerpt=product.short_description,
        sku=product.sku,
        price=str(product.price),
        price_currency=product.currency,
        availability="InStock" if product.in_stock else "OutOfStock",
    )

@shop.get("/api/products/<slug>")
def product_json(slug):
    from .models import Product
    product = Product.query.filter_by(slug=slug).first_or_404()
    return jsonify(easeo.for_entity(entity_from_product(product), f"/products/{slug}"))
```

## Complex: ETag responses { #flask-etag }

Return the payload's deterministic ETag and answer `304` when it matches:

```python
from flask import request, jsonify
from easeo import build_seo_payload

@shop.get("/api/products/<slug>/seo")
def product_seo(slug):
    product = Product.query.filter_by(slug=slug).first_or_404()
    payload = build_seo_payload(entity_from_product(product), f"/products/{slug}", easeo.config)
    etag = payload.etag()

    if request.if_none_match and etag in request.if_none_match:
        return "", 304

    response = jsonify(payload.to_dict())
    response.headers["ETag"] = etag
    response.headers["Cache-Control"] = "public, max-age=300"
    return response
```

## Notes { #flask-notes }

* The template helper returns `Markup`, so `| safe` is optional.
* `for_entity` returns a dict; the helper returns HTML.
* `Easeo.init_app` raises `ValueError` when no config was provided.
* Read the config back from the adapter with `easeo.config` for direct
  `build_seo_payload` calls.
