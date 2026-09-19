---
title: "Python"
description: "Python examples for easeo, from a minimal payload to hooks, registries, async, and contracts."
---

# Python { #python }

## Simple: a minimal payload { #python-simple }

```python
from easeo import SEOConfig, SEOEntity, build_seo_payload

config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
)

entity = SEOEntity(
    entity_type="post",
    title="Hello World",
    excerpt="An example post.",
)

payload = build_seo_payload(entity, "/blog/hello", config)
print(payload.render_html())
```

## Simple: a structured entity { #python-builder }

```python
from easeo import SEOEntityBuilder

entity = (
    SEOEntityBuilder("post")
    .title("Introducing easeo")
    .excerpt("Deterministic SEO payloads.")
    .featured_image("https://cdn.example.com/hero.jpg", width=1200, height=630, alt="Hero")
    .author_name("Jane Doe")
    .published_at("2026-01-15")
    .breadcrumb("Home", "/")
    .breadcrumb("Blog", "/blog")
    .build()
)
```

## Simple: per-call overrides { #python-overrides }

```python
from easeo import SEOOverrides, build_seo_payload

payload = build_seo_payload(
    entity,
    "/blog/hello",
    config,
    SEOOverrides(
        meta_title="A one-off title",
        skip_title_template=True,
        twitter_creator="@easeo",
    ),
)
```

## Simple: serialize { #python-serialize }

```python
payload.to_dict()      # canonical snake_case dict
payload.to_json()      # canonical JSON string
payload.hash()         # SHA-256 hex
payload.etag()         # quoted HTTP ETag
```

## Complex: a full site config { #python-full-config }

```python
from easeo import (
    SEOConfig,
    URLPolicy,
    Robots,
    SEOImage,
)

config = SEOConfig(
    canonical_host="shop.example.com",
    public_base_url="https://shop.example.com",
    url_policy=URLPolicy(
        enforce_https=True,
        lowercase_paths=True,
        trailing_slash="never",
        collapse_duplicate_slashes=True,
        strip_tracking_params=True,
        allowed_query_params=["page", "q"],
    ),
    site_name="Example Shop",
    title_template="{title} - Example Shop",
    default_robots=Robots(index=True, follow=True),
    search_robots=Robots(index=False, follow=True),
    default_og_image=SEOImage(
        url="https://shop.example.com/assets/og-image.png",
        width=1200,
        height=630,
        alt="Example Shop",
    ),
    publisher_name="Example Shop",
    publisher_logo="https://shop.example.com/assets/logo.png",
    locale="en_US",
    locale_alternate=["es_UY", "pt_BR"],
    twitter_site="@exampleshop",
    auto_generate_schema=True,
    search_url_template="https://shop.example.com/search?q={search_term_string}",
)
```

## Complex: build from real content objects { #python-real-content }

A small helper that turns a model instance into an entity, plus overrides for
the one field that needs to differ:

```python
from easeo import SEOEntity, SEOOverrides, build_seo_payload

def entity_from_product(product):
    return SEOEntity(
        entity_type="product",
        title=product.name,
        excerpt=product.short_description,
        featured_image=product.image.url if product.image else None,
        sku=product.sku,
        price=str(product.price),
        price_currency=product.currency,
        availability="InStock" if product.in_stock else "OutOfStock",
        breadcrumbs=[
            # Breadcrumb(name=..., url=...) entries
        ],
        updated_at=product.updated_at.isoformat(),
    )

payload = build_seo_payload(
    entity_from_product(product),
    f"/products/{product.slug}",
    config,
    SEOOverrides(og_image=product.social_card) if product.social_card else None,
)

cache_headers = {
    "ETag": payload.etag(),
    "Cache-Control": "public, max-age=300",
}
```

## Complex: hooks for a site-wide schema { #python-hooks }

A hook adds an `Organization` node to every payload. Hooks are attached to the
config, so the builder stays a pure function of its inputs:

```python
from easeo import HookRegistry, SEOConfig

hooks = HookRegistry()

@hooks.hook("post_process")
def inject_organization(payload, entity, config):
    org = {
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": config.publisher_name or "Example",
        "url": config.public_base_url,
    }
    existing = payload.get("schema_jsonld")
    if isinstance(existing, list):
        payload["schema_jsonld"] = [org, *existing]
    elif existing is not None:
        payload["schema_jsonld"] = [org, existing]
    else:
        payload["schema_jsonld"] = org
    return payload

config = SEOConfig(..., hooks=hooks)
```

See [Hooks](../guides/hooks.md) for ordering, scoping, and error behavior.

## Complex: a registered schema generator { #python-registry }

Register one generator per schema type and it replaces the built-in schema
whenever the resolved `@type` matches:

```python
from easeo import SchemaRegistry, SEOConfig

registry = SchemaRegistry()

@registry.register("Article")
def podcast_episode(entity, config, canonical, title, description, og_image):
    return {
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        "name": title,
        "url": canonical,
        "description": description,
        "associatedMedia": {"@type": "AudioObject", "contentUrl": entity.body_html},
    }

config = SEOConfig(..., schema_registry=registry)
```

Return `None` from a generator to fall back to the built-in schema for that
page. See [Custom JSON-LD](../guides/custom-schemas.md).

## Complex: async with FastAPI { #python-async }

```python
from easeo import build_seo_payload_async

async def get_seo(product, route):
    payload = await build_seo_payload_async(
        entity_from_product(product), route, config
    )
    return payload.to_dict()
```

The Rust core releases the GIL, so the build runs off the event loop.

## Complex: contracts and validation { #python-contracts }

```python
from easeo import (
    SEOContractConfig,
    SEOContractRule,
    SEOExpectation,
    build_seo_contract,
)

contract = build_seo_contract(
    SEOContractConfig(
        canonical_host="shop.example.com",
        scheme="https",
        defaults=SEOExpectation(
            title=SEOExpectation(min_length=20, max_length=60),
            description=SEOExpectation(min_length=70, max_length=160),
            og_required=True,
            schema_required=True,
        ),
        rules=[
            SEOContractRule(
                match="/blog/*",
                expect=SEOExpectation(schema_types=["Article"]),
            ),
            SEOContractRule(
                match="/search",
                expect=SEOExpectation(indexable=False),
            ),
        ],
    )
)

contract.write(".easeo/contract.json")
```

Validate a built payload against best practices:

```python
from easeo import validate_payload

for issue in validate_payload(payload):
    print(issue.rule_id, issue.severity, issue.message)
```

Set `emit_warnings=True` on the config to have those issues emitted as Python
warnings during the build. See [Validation](../concepts/validation.md) for the
rule list.

## Complex: determinism check in tests { #python-tests }

```python
def test_seo_is_stable():
    a = build_seo_payload(entity, "/blog/hello", config)
    b = build_seo_payload(entity, "/blog/hello", config)
    assert a == b
    assert a.hash() == b.hash()

def test_seo_snapshot():
    payload = build_seo_payload(entity, "/blog/hello", config)
    assert payload == expected_payload  # committed fixture
```

## Related { #related }

* [Python API reference](../reference/python-api.md)
* [Fallback chains](../concepts/fallback-chains.md)
* [Recipes](../recipes/index.md)
