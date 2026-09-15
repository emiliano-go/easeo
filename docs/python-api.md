# Python API Reference

## Core Types

- `SEOConfig` — Site-wide configuration
- `SEOEntity` — Content entity input
- `SEOOverrides` — Per-entity overrides (highest precedence)
- `SEOPayload` — Generated SEO output
- `URLPolicy` — URL normalization policy
- `Robots` — Robots directive
- `SEOImage` — Structured image
- `Breadcrumb` — Breadcrumb item
- `FAQItem` — FAQ question/answer
- `SEOContract` — SEO contract
- `SEOContractConfig` — Contract configuration
- `SEOIssue` — Validation issue
- `SEOEntityBuilder` — Fluent builder for `SEOEntity`

## Core Functions

- `build_seo_payload(entity, route, config)` — Build SEO payload
- `build_seo_payload_with_overrides(entity, route, config, overrides)` — Build with per-page overrides
- `build_seo_payload_dict(entity, route, config)` — Build, returned as a plain dict
- `build_seo_contract(config)` — Build contract
- `validate_payload(payload)` — Validate payload, returns `list[SEOIssue]`
- `normalize_path(path, policy)` — Normalize URL path
- `normalize_public_url(url, config)` — Build canonical URL
- `clean_url(url)` — Remove tracking params; returns `{url, removed_params, cleaned_params}` dicts
- `clean_query(query)` — Remove tracking params from a query string

## SEOEntityBuilder

Fluent sugar over the `SEOEntity` constructor:

```python
from easeo import SEOEntityBuilder

entity = (
    SEOEntityBuilder("post")
    .title("Hello World")
    .excerpt("An example post.")
    .featured_image("https://example.com/hero.jpg", width=1200, height=630, alt="Hero")
    .breadcrumb("Home", "/")
    .breadcrumb("Blog", "/blog")
    .faq_item("What is easeo?", "Deterministic SEO payloads.")
    .build()
)
```

Methods: `slug`, `title`, `excerpt`, `body_html`, `status`, `featured_image`, `published_at`, `updated_at`, `author_name`, `sku`, `price(amount, currency=None)`, `availability`, `address`, `same_as`, `breadcrumb(name, url)`, `faq_item(question, answer)`, `build()`.

## Custom JSON-LD schemas

`SchemaRegistry.register()` is Rust-only — Python callables cannot be stored in the Rust registry, so the Python method raises `NotImplementedError` by design. Pass custom JSON-LD per page instead:

```python
from easeo import SEOOverrides, build_seo_payload_with_overrides

payload = build_seo_payload_with_overrides(
    entity, "/podcast/ep-1", config,
    SEOOverrides(schema_jsonld={"@context": "https://schema.org", "@type": "Podcast", "name": "My Podcast"}),
)
```

## Framework adapters

- FastAPI: `from easeo.adapters.fastapi import EaseoSEO`
- Django: `from easeo.adapters.django import seo_head`
- Flask: `from easeo.adapters.flask import Easeo`
- Zensical: `easeo.contrib.zensical` markdown extension (see `zensical.toml` example in the repo root)

Install the matching extra: `pip install easeo[fastapi]` (or `django` / `flask` / `zensical` / `all`).
