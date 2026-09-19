---
title: "Category Page"
description: "A taxonomy listing page with a title template and default OG image."
---

# Recipe: Category Page

A taxonomy listing page that uses a site-wide default OG image and a title
template.

```python
from easeo import SEOConfig, SEOEntity, build_seo_payload

config = SEOConfig(
    canonical_host="shop.example.com",
    public_base_url="https://shop.example.com",
    site_name="Example Shop",
    title_template="{title} - Example Shop",
    default_og_image="https://cdn.example.com/default-og.jpg",
)

entity = SEOEntity(
    entity_type="taxonomy",
    title="Audio",
    excerpt="All audio products.",
    breadcrumbs=[
        # Breadcrumb dataclasses or the builder helpers both work
    ],
)

payload = build_seo_payload(entity, "/audio", config)
```

Key points:

- `entity_type="taxonomy"` maps to a `CollectionPage` schema.
- The `title_template` applies automatically → `"Audio - Example Shop"`.
- `default_og_image` fills `og:image` when the entity has no image.
- With `trailing_slash="always"` in a `URLPolicy`, `/audio` becomes
  `https://shop.example.com/audio/`.
