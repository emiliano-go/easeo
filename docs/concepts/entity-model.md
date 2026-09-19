---
title: "Entity Model"
description: "The SEOEntity fields, entity types, and which output each field feeds."
---

# Entity Model { #entity-model }

An `SEOEntity` is the content you already have, reshaped into the fields easeo
knows how to use. Only `entity_type` is required; every other field is
optional.

## All fields { #fields }

| Field | Type | Feeds |
|---|---|---|
| `entity_type` | `str` (required) | Meta plus schema selection |
| `title` | `str \| None` | Title, `og:title`, schema |
| `excerpt` | `str \| None` | Description, `og:description` |
| `body_html` | `str \| None` | Description snippet when no excerpt |
| `slug` | `str \| None` | Metadata only, not emitted |
| `status` | `str \| None` | Robots (`"published"` allows indexing) |
| `featured_image` | `SEOImage \| str \| None` | `og:image` and schema image |
| `published_at` | `str \| None` | Schema `datePublished` |
| `updated_at` | `str \| None` | Schema `dateModified` |
| `author_name` | `str \| None` | Schema author |
| `breadcrumbs` | `list[Breadcrumb] \| None` | `BreadcrumbList` JSON-LD |
| `sku` | `str \| None` | Product schema |
| `price` | `str \| None` | Product schema |
| `price_currency` | `str \| None` | Product schema |
| `availability` | `str \| None` | Product schema |
| `same_as` | `list[str] \| None` | Organization `sameAs` |
| `address` | `str \| None` | LocalBusiness address |
| `faq_items` | `list[FAQItem] \| None` | `FAQPage` schema |

## Entity types { #types }

The `entity_type` drives two things: the Open Graph type and the default
schema mapping.

| Entity type | OG type | Schema |
|---|---|---|
| `home` | `website` | `WebPage` |
| `post` | `article` | `Article` |
| `page` | `website` | `WebPage` |
| `video` | `article` | `VideoObject` |
| `taxonomy` | `website` | `CollectionPage` |
| `search` | `website` | `SearchResultsPage` |
| `product` | `website` | `Product` |
| `organization` | `website` | `Organization` |
| `local_business` | `website` | `LocalBusiness` |
| `faq` | `website` | `FAQPage` |
| `other` | `website` | none |

Override the mapping with `schema_type_map` on the config, or replace a single
page's schema with `SEOOverrides.schema_jsonld`.

## Building an entity { #building }

Three equivalent ways:

=== "Constructor"

    ```python
    from easeo import SEOEntity

    entity = SEOEntity(
        entity_type="post",
        title="Hello",
        excerpt="A post.",
    )
    ```

=== "Builder"

    ```python
    from easeo import SEOEntityBuilder

    entity = (
        SEOEntityBuilder("post")
        .title("Hello")
        .excerpt("A post.")
        .breadcrumb("Home", "/")
        .build()
    )
    ```

=== "Factory"

    ```python
    from easeo import from_blog_post

    entity = from_blog_post(title="Hello", body_html="<p>A post.</p>")
    ```

## Normalization { #normalization }

Optional string fields are stripped; empty strings become `None`. Lists such
as `same_as` are deduplicated. This keeps the output stable regardless of
whitespace in your source data.

## Recap { #recap }

* `entity_type` selects the OG type and schema.
* Most fields feed more than one output target.
* Constructor, builder, and factories are interchangeable.
