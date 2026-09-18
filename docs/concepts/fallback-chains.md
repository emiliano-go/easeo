---
title: "Fallback Chains"
---

# Fallback Chains { #fallback-chains }

Every field resolves through a priority chain; the first non-empty value wins.
Set site-wide defaults in `SEOConfig`, override per entity in `SEOEntity`, and
fine-tune per page with `SEOOverrides`.

## General precedence

1. **`SEOOverrides`**: per-call overrides (highest)
2. **`SEOEntity`**: content entity fields
3. **`SEOConfig`**: site-wide defaults
4. **Hardcoded defaults**: library fallbacks (lowest)

## title

1. `SEOOverrides.meta_title`
2. `SEOEntity.title`
3. `"Untitled"`

The config `title_template` is then applied unless `skip_title_template=True`.

```python
config = SEOConfig(..., title_template="{title} - My Site")
# "My Post" -> "My Post - My Site"
```

## description

1. `SEOOverrides.meta_description`
2. `SEOEntity.excerpt`
3. Auto snippet from `SEOEntity.body_html` (max 160 chars)
4. `""`

The body snippet strips scripts and styles, normalizes whitespace, and
truncates on a character boundary with an ellipsis.

## canonical

1. `SEOOverrides.canonical_url`
2. Normalized route path (full URL normalization pipeline)

## robots

1. `SEOOverrides.robots`
2. Entity-derived default:
   - `entity_type == "search"` → `config.search_robots` (default `noindex,follow`)
   - `entity.status == "published"` → `index,follow`
   - otherwise → `config.default_robots` (default `index,follow`)

## Open Graph

| Field | Chain |
|---|---|
| `og:title` | `SEOOverrides.og_title` > resolved title |
| `og:description` | `SEOOverrides.og_description` > resolved description |
| `og:image` | `SEOOverrides.og_image` > `SEOEntity.featured_image` > `SEOConfig.default_og_image` |

The resolved image cascades to `twitter:image`.

## Twitter

| Field | Chain |
|---|---|
| `twitter:card` | `SEOOverrides.twitter_card` > `"summary_large_image"` |
| `twitter:title` | `SEOOverrides.twitter_title` > resolved `og:title` |
| `twitter:description` | `SEOOverrides.twitter_description` > resolved `og:description` |
| `twitter:image` | `SEOOverrides.twitter_image` > resolved `og:image` |

## schema_jsonld

1. `SEOOverrides.omit_schema` → `None`
2. `SEOOverrides.schema_jsonld` (normalized to a list when needed)
3. **`SchemaRegistry` generator** matching the resolved `@type` (Python/JS)
4. Auto-generated schema (when `config.auto_generate_schema`)

Breadcrumbs from `entity.breadcrumbs` are always appended as a
`BreadcrumbList`, and hooks run last over the assembled payload.

## Summary table

| Field | Chain |
|---|---|
| title | Override > Entity > `"Untitled"` + template |
| description | Override > Excerpt > Body snippet > `""` |
| canonical | Override > Normalized route |
| robots | Override > Entity status default |
| og:title | Override > Resolved title |
| og:description | Override > Resolved description |
| og:image | Override > Entity image > Config default |
| twitter:card | Override > `"summary_large_image"` |
| twitter:image | Override > resolved og:image |
| schema_jsonld | Override > Registry > auto-generated + breadcrumbs |
