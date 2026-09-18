---
title: "Product Page"
---

# Recipe: Product Page

A product with SKU, price, availability, and a breadcrumb trail.

```python
from easeo import SEOConfig, SEOEntityBuilder, build_seo_payload

config = SEOConfig(
    canonical_host="shop.example.com",
    public_base_url="https://shop.example.com",
    site_name="Example Shop",
)

entity = (
    SEOEntityBuilder("product")
    .title("Wireless Headphones")
    .excerpt("Noise-cancelling over-ear headphones.")
    .sku("WH-1000")
    .price("79.99", currency="USD")
    .availability("InStock")
    .featured_image("https://cdn.example.com/wh1000.jpg", width=1200, height=630)
    .breadcrumb("Home", "/")
    .breadcrumb("Audio", "/audio")
    .build()
)

payload = build_seo_payload(entity, "/audio/wireless-headphones", config)
```

Resolved JSON-LD:

```json
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Wireless Headphones",
  "offers": {
    "@type": "Offer",
    "price": "79.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock"
  }
}
```

Factory shortcut:

```python
from easeo import from_product

entity = from_product(
    name="Wireless Headphones",
    sku="WH-1000",
    price=79.99,
    currency="USD",
    availability="InStock",
)
```
