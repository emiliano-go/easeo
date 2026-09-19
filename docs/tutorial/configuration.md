---
title: "Configuration"
description: "Every SEOConfig field, URLPolicy option, and validation rule."
---

# Configuration { #configuration }

`SEOConfig` holds every site-wide value. This page lists all fields with their
types, defaults, and validation rules.

## SEOConfig fields { #seoconfig }

| Field | Type | Default | Description |
|---|---|---|---|
| `canonical_host` | `str` | required | Hostname only; no scheme, path, port, or trailing dot |
| `public_base_url` | `str` | required | Absolute `http(s)` URL used as the base for canonical resolution |
| `url_policy` | `URLPolicy` | defaults | URL normalization rules |
| `site_name` | `str \| None` | `None` | `og:site_name` and title template context |
| `title_template` | `str \| None` | `"{title}"` | Must contain the `{title}` placeholder |
| `default_robots` | `Robots \| None` | `index,follow` | Fallback robots directive |
| `search_robots` | `Robots \| None` | `noindex,follow` | Robots directive for `search` pages |
| `default_og_image` | `SEOImage \| None` | `None` | Fallback Open Graph image |
| `publisher_name` | `str \| None` | `None` | Publisher name for JSON-LD |
| `publisher_logo` | `str \| None` | `None` | Publisher logo URL for JSON-LD |
| `locale` | `str \| None` | `None` | `og:locale` |
| `locale_alternate` | `list[str] \| None` | `None` | `og:locale:alternate` values |
| `twitter_site` | `str \| None` | `None` | `twitter:site` handle |
| `auto_generate_schema` | `bool` | `True` | Generate JSON-LD from the entity type |
| `emit_warnings` | `bool` | `False` | Emit validation warnings |
| `schema_type_map` | `dict \| None` | built-in | Override the entity-type to schema-type map |
| `hooks` | `HookRegistry \| None` | `None` | Config-scoped post-processing |
| `schema_registry` | `SchemaRegistry \| None` | `None` | Config-scoped JSON-LD generators |

The JavaScript field names are the camelCase equivalents: `canonicalHost`,
`publicBaseUrl`, `urlPolicy`, `siteName`, `titleTemplate`, `defaultRobots`,
`searchRobots`, `defaultOgImage`, `publisherName`, `publisherLogo`,
`localeAlternate`, `twitterSite`, `autoGenerateSchema`, `emitWarnings`,
`schemaTypeMap`, `hooks`, `schemaRegistry`.

## Validation rules { #validation }

* `canonical_host` must be host-only. A scheme, path, query, port, or trailing
  dot is rejected with `ConfigurationError`.
* `public_base_url` must be an absolute `http` or `https` URL.
* `title_template` must contain `{title}`.
* `locale_alternate` is deduplicated and stripped.
* `url_policy` must be a `URLPolicy` instance.

## URLPolicy { #urlpolicy }

Controls canonical URL normalization.

| Field | Type | Default |
|---|---|---|
| `enforce_https` | `bool` | `True` |
| `lowercase_paths` | `bool` | `True` |
| `trailing_slash` | `"always" \| "never" \| "preserve"` | `"never"` |
| `collapse_duplicate_slashes` | `bool` | `True` |
| `strip_tracking_params` | `bool` | `True` |
| `allowed_query_params` | `list[str]` | `[]` |

=== "Python"

    ```python
    from easeo import SEOConfig, URLPolicy

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        url_policy=URLPolicy(
            trailing_slash="always",
            allowed_query_params=["page", "q"],
        ),
    )
    ```

=== "JavaScript"

    ```js
    const config = {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
      trailingSlash: "always",
      allowedQueryParams: ["page", "q"],
    };
    ```

## Robots { #robots }

A structured robots directive. All fields are optional.

| Field | Type |
|---|---|
| `index` | `bool` |
| `follow` | `bool` |
| `max_snippet` | `int` |
| `max_image_preview` | `str` |
| `max_video_preview` | `int` |

=== "Python"

    ```python
    from easeo import Robots

    robots = Robots(index=True, follow=False, max_snippet=160)
    # serializes to "index,nofollow,max-snippet:160"
    ```

## Recap { #recap }

* Two required fields: `canonical_host` and `public_base_url`.
* `URLPolicy` controls URL normalization.
* `Robots`, `SEOImage`, `Breadcrumb`, and `FAQItem` are the supporting value
  types.

**Next:** the [Concepts](../concepts/index.md) track explains how the pieces
fit together.
