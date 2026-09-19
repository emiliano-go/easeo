---
title: "Python API"
description: "The easeo Python API: types, functions, payload methods, and adapters."
---

# Python API Reference { #python-api-reference }

## Core Types

- `SEOConfig`: Site-wide configuration
- `SEOEntity`: Content entity input
- `SEOOverrides`: Per-entity overrides (highest precedence)
- `SEOPayload`: Generated SEO output
- `URLPolicy`: URL normalization policy
- `Robots`: Robots directive
- `SEOImage`: Structured image
- `Breadcrumb`: Breadcrumb item
- `FAQItem`: FAQ question/answer
- `SEOContract`: SEO contract
- `SEOContractConfig`: Contract configuration
- `SEOIssue`: Validation issue
- `SEOEntityBuilder`: Fluent builder for `SEOEntity`

## Core Functions

- `build_seo_payload(entity, route, config, overrides=None)`: Build SEO payload
- `build_seo_payload_dict(entity, route, config, overrides=None)`: Build, returned as a plain dict
- `build_seo_payload_async(entity, route, config, overrides=None, executor=None)`: Async version (thread-pool offload)
- `build_seo_contract(config)`: Build contract
- `validate_payload(payload)`: Validate payload, returns `list[SEOIssue]`
- `normalize_path(path, policy)`: Normalize URL path
- `normalize_public_url(url, config)`: Build canonical URL
- `clean_url(url)`: Remove tracking params; returns `{url, removed_params, cleaned_params}` dicts
- `clean_query(query)`: Remove tracking params from a query string

`build_seo_payload` accepts per-call `overrides` directly (no separate
`*_with_overrides` call needed). Both spellings exist and behave identically.

## Payload ergonomics

The payload is dict-compatible and comparable, which is what makes snapshot
testing work:

```python
payload = build_seo_payload(entity, "/x", config)

payload["title"]                 # dict-style access
payload.get("title")             # with optional default
"title" in payload               # membership
list(payload)                    # keys
len(payload)                     # field count

payload == build_seo_payload(entity, "/x", config)  # True
payload == payload.to_dict()                         # True
```

## Factories

Convenience constructors for common content types:

```python
from easeo import from_blog_post, from_product, from_faq

from_blog_post(title, body_html, slug=None, author="", excerpt=None, breadcrumbs=None)
from_product(name, sku, price, currency="USD", availability="InStock", description=None)
from_faq(questions, title="FAQ", description=None)
```

## Async

```python
from easeo import build_seo_payload_async

payload = await build_seo_payload_async(entity, "/x", config)
```

The Rust core releases the GIL, so the build genuinely runs off the event
loop. Configure the pool with `set_executor(executor)`.

## Extension points

```python
from easeo import HookRegistry, SchemaRegistry

# Post-process every payload for this config
hooks = HookRegistry()

@hooks.hook("post_process")
def add_generator(payload, entity, config):
    payload["generator"] = "easeo"
    return payload

# Custom JSON-LD per schema type
registry = SchemaRegistry()

@registry.register("Article")
def podcast(entity, config, canonical, title, description, og_image):
    return {"@context": "https://schema.org", "@type": "PodcastEpisode", "name": title}

config = SEOConfig(..., hooks=hooks, schema_registry=registry)
```

Both live on the config, so the builder stays a pure function of its inputs.

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

`SchemaRegistry` accepts Python callables. Attach it to the config and a
generator runs whenever the resolved schema `@type` matches:

```python
from easeo import SchemaRegistry

registry = SchemaRegistry()

@registry.register("Article")
def podcast(entity, config, canonical, title, description, og_image):
    return {"@context": "https://schema.org", "@type": "Podcast", "name": title}

config = SEOConfig(..., schema_registry=registry)
```

Methods: `register`, `unregister`, `get`, `has`, `list_types`.

You can also replace the schema per page with overrides:

```python
from easeo import SEOOverrides, build_seo_payload

payload = build_seo_payload(
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
