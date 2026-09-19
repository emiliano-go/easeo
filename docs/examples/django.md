---
title: "Django"
description: "Django examples for easeo: settings, template tags, mapped entities, and ETag views."
---

# Django { #django }

The Django adapter reads its configuration from the `EASEO` setting and renders
tags from templates.

## Simple: settings and a template tag { #django-simple }

```python
# settings.py
EASEO = {
    "canonical_host": "example.com",
    "public_base_url": "https://example.com",
    "site_name": "Example",
    "title_template": "{title} - Example",
}
```

```python
# models.py
class Post(models.Model):
    title = models.CharField(max_length=200)
    description = models.TextField()
```

```django
{# templates/post.html #}
{% load easeo_tags %}
<!doctype html>
<html>
  <head>
    {% easeo_head post request.path %}
  </head>
  <body><h1>{{ post.title }}</h1></body>
</html>
```

The adapter reads `entity_type` (defaulting to `page`), `title`, and
`description` or `excerpt` from the object it is given.

## Simple: granular tags { #django-tags }

```django
{% load easeo_tags %}
<head>
  {% easeo_title post %}
  {% easeo_meta post %}
</head>
```

## Complex: full settings { #django-full-settings }

Every `SEOConfig` field is supported. `default_og_image` accepts a URL string
or a mapping; `url_policy`, `default_robots`, and `search_robots` accept
mappings.

```python
# settings.py
EASEO = {
    "canonical_host": "shop.example.com",
    "public_base_url": "https://shop.example.com",
    "site_name": "Example Shop",
    "title_template": "{title} - Example Shop",
    "url_policy": {
        "enforce_https": True,
        "trailing_slash": "never",
        "allowed_query_params": ["page", "q"],
    },
    "default_robots": {"index": True, "follow": True},
    "search_robots": {"index": False, "follow": True},
    "default_og_image": {
        "url": "https://shop.example.com/assets/og-image.png",
        "width": 1200,
        "height": 630,
        "alt": "Example Shop",
    },
    "publisher_name": "Example Shop",
    "locale": "en_US",
    "twitter_site": "@exampleshop",
    "auto_generate_schema": True,
    "search_url_template": "https://shop.example.com/search?q={search_term_string}",
}
```

## Complex: map a model to an entity { #django-entity }

For product pages you want the schema fields, so build an `SEOEntity` rather
than relying on attribute defaults:

```python
# views.py
from django.shortcuts import render
from easeo import SEOEntity, build_seo_payload
from .models import Product

def entity_from_product(product: Product) -> SEOEntity:
    return SEOEntity(
        entity_type="product",
        title=product.name,
        excerpt=product.short_description,
        featured_image=product.image_url,
        sku=product.sku,
        price=str(product.price),
        price_currency=product.currency,
        availability="InStock" if product.in_stock else "OutOfStock",
        updated_at=product.updated_at.isoformat(),
        breadcrumbs=[
            # Breadcrumb(name=..., url=...) entries
        ],
    )

def product_detail(request, slug):
    product = Product.objects.get(slug=slug)
    payload = build_seo_payload(entity_from_product(product), request.path, config_from_settings())
    return render(request, "product.html", {"product": product, "seo_html": payload.render_html()})
```

```django
{# templates/product.html #}
<head>{{ seo_html | safe }}</head>
```

## Complex: an ETag response { #django-etag }

Wrap the payload in a conditional response so repeat visitors get a `304`:

```python
from django.http import HttpResponseNotModified, JsonResponse
from easeo import build_seo_payload

def product_json(request, slug):
    product = Product.objects.get(slug=slug)
    payload = build_seo_payload(entity_from_product(product), request.path, config_from_settings())
    etag = payload.etag()

    if request.headers.get("If-None-Match") == etag:
        return HttpResponseNotModified()

    response = JsonResponse(payload.to_dict())
    response.headers["ETag"] = etag
    response.headers["Cache-Control"] = "public, max-age=300"
    return response
```

## Notes { #django-notes }

* If `EASEO` is missing, the adapter warns and falls back to `localhost`.
* Tags return `mark_safe` output because the payload escapes its values.
* The function API `seo_head(entity, route, config=None)` bypasses settings
  when you pass a config.
