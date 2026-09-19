---
title: "Blog Post"
description: "A published article with a hero image, author, and breadcrumbs."
---

# Recipe: Blog Post

A published article with a hero image, author, and breadcrumbs.

```python
from easeo import (
    SEOConfig, SEOEntityBuilder, SEOImage, Breadcrumb, build_seo_payload,
)

config = SEOConfig(
    canonical_host="blog.example.com",
    public_base_url="https://blog.example.com",
    site_name="Example Blog",
    title_template="{title} - Example Blog",
    locale="en_US",
    twitter_site="@example",
)

entity = (
    SEOEntityBuilder("post")
    .title("Introducing easeo")
    .excerpt("Deterministic SEO payloads for content platforms.")
    .author_name("Jane Doe")
    .published_at("2026-01-15")
    .featured_image("https://cdn.example.com/hero.jpg", width=1200, height=630, alt="Hero")
    .breadcrumb("Home", "/")
    .breadcrumb("Blog", "/blog")
    .build()
)

payload = build_seo_payload(entity, "/blog/introducing-easeo", config)
print(payload.render_html())
```

The resolved payload:

- **title** → `"Introducing easeo - Example Blog"`
- **og:type** → `article` (post maps to Article)
- **schema_jsonld** → `Article` plus an appended `BreadcrumbList`
- **og:image** with `width`/`height`/`alt` from the structured image

Factory shortcut:

```python
from easeo import from_blog_post

entity = from_blog_post(
    title="Introducing easeo",
    body_html="<p>Full article body…</p>",
    author="Jane Doe",
    breadcrumbs=[{"name": "Blog", "url": "/blog"}],
)
```
