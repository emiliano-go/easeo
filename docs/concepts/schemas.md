---
title: "JSON-LD Schemas"
---

# JSON-LD Schemas { #schemas }

easeo generates schema.org JSON-LD from the entity type, then lets you replace
or extend it. The result is exposed as `schema_jsonld` (Python) or
`schemaJsonLd` (JavaScript).

## Built-in schemas { #built-in }

| Schema | Built from | Key fields |
|---|---|---|
| `Article` | `post`, `video` | headline, dates, author, publisher |
| `WebPage` | `home`, `page` | name, url, description |
| `VideoObject` | `video` | name, url, description |
| `CollectionPage` | `taxonomy` | name, url |
| `SearchResultsPage` | `search` | name, url |
| `Product` | `product` | sku, offers (price, currency, availability) |
| `Organization` | `organization` | name, url, `sameAs` |
| `LocalBusiness` | `local_business` | name, address, url |
| `FAQPage` | `faq` | `mainEntity` question/answer pairs |
| `BreadcrumbList` | any, when breadcrumbs exist | `itemListElement` |

Breadcrumbs are always appended as a `BreadcrumbList`. When a page has both a
main schema and breadcrumbs, `schema_jsonld` becomes a list of two objects.

## Three ways to control it { #control }

### Auto-generate { #auto }

Leave `auto_generate_schema` at its default of `True`:

```python
config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
    auto_generate_schema=True,
)
```

### Replace for one page { #override }

Use `SEOOverrides` when a single page needs a different shape:

```python
from easeo import SEOOverrides, build_seo_payload

payload = build_seo_payload(
    entity,
    "/podcast/ep-1",
    config,
    SEOOverrides(schema_jsonld={
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        "name": "Episode 1",
    }),
)
```

### Register a generator for a type { #registry }

When every page of a given schema type should use the same shape, register a
generator once on the config:

```python
from easeo import SchemaRegistry

registry = SchemaRegistry()

@registry.register("Article")
def podcast_episode(entity, config, canonical, title, description, og_image):
    return {
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        "name": title,
        "url": canonical,
    }

config = SEOConfig(..., schema_registry=registry)
```

The generator runs whenever the resolved `@type` matches. Return `None` to
fall back to the built-in schema for that page.

## Suppressing the schema { #omit }

```python
overrides = SEOOverrides(omit_schema=True)
```

## Recap { #recap }

* Schemas are generated from the entity type.
* Breadcrumbs are appended automatically.
* Replace per page with `SEOOverrides`, or per type with `SchemaRegistry`.
