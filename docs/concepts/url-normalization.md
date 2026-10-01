---
title: "URL Normalization"
description: "The canonical URL pipeline: path rules, query filtering, and host validation."
---

# URL Normalization { #url-normalization }

Canonical URLs are built in the Rust core, not in your framework. This keeps
one source of truth: no adapter re-implements trailing slash logic, and Python
and JavaScript agree byte for byte.

## The pipeline { #pipeline }

`normalize_public_url` runs these steps:

1. Parse the input URL or path.
2. Extract the base path from `public_base_url`.
3. Prepend the base path when it is not already present.
4. Normalize the path (see below).
5. Filter query parameters: strip tracking, keep the allowlist.
6. Build `scheme://canonical_host/normalized_path?filtered_query`.

## Path rules { #path-rules }

Controlled by `URLPolicy`:

| Rule | Default | Effect |
|---|---|---|
| Ensure leading slash | always | `/blog` and `blog` both become `/blog` |
| Collapse duplicate slashes | `True` | `/a//b` becomes `/a/b` |
| Lowercase | `False` | Path case is preserved; `/Products/iPhone` stays as written. Set `lowercase_paths=True` to lowercase |
| Trailing slash | `"never"` | `/blog/` becomes `/blog`; use `"always"` or `"preserve"` to change |
| Enforce HTTPS | `True` | `http://` becomes `https://` |

Fragments are always dropped: a canonical URL never carries one, whether the
input is an absolute URL or a relative path.

## Query filtering { #query }

Tracking parameters are stripped using the absorbed detrack engine. The
built-in list is deliberately narrow: UTM parameters, vendor click ids
(`fbclid`, `gclid`, `msclkid`, ...), and unambiguous session identifiers.
Names that applications commonly use for real content (`ref`, `source`,
`tag`, `keyword`, `campaign`, `redirect`, `next`, `timestamp`, ...) are kept
by default. Add site-specific names with `extra_tracking_params`.

Parameter keys are decoded before matching, so `%75tm_source` is recognized as
`utm_source`, while the emitted query preserves the original bytes, order, and
duplicates.

When `allowed_query_params` is non-empty, only those parameters survive, even
when one of them looks like a tracking parameter.

=== "Python"

    ```python
    from easeo import SEOConfig, URLPolicy, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        url_policy=URLPolicy(allowed_query_params=["page", "q"]),
    )

    payload = build_seo_payload(
        SEOEntity(entity_type="search", title="Search"),
        "/search?q=headphones&utm_source=ad&ref=x",
        config,
    )

    print(payload.canonical)
    # "https://example.com/search?q=headphones"
    ```

`utm_source` and `ref` are stripped; `q` is kept.

## Host validation { #host }

`canonical_host` must be host-only. These all fail with `ConfigurationError`:

```text
https://example.com     (scheme)
example.com/path        (path)
example.com:8080        (port)
example.com/#frag       (fragment)
example.com.            (trailing dot)
```

## Sub-path deployments { #subpath }

If `public_base_url` includes a path, that path is used as the base:

```python
config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com/blog/",
)

payload = build_seo_payload(entity, "/post", config)
print(payload.canonical)
# "https://example.com/blog/post"
```

## Raw helpers { #helpers }

| Python | JavaScript | Purpose |
|---|---|---|
| `normalize_path(path, policy)` | `normalizePath(path, options)` | Normalize a path only |
| `normalize_public_url(url, config)` | `normalizePublicUrl(url, config)` | Build a canonical URL |
| `clean_url(url)` | `cleanUrl(url)` | Remove tracking params, return a report |
| `clean_query(query)` | `cleanQuery(query)` | Clean a query string |

## Recap { #recap }

* Normalization lives in the core and is shared by every language.
* `URLPolicy` controls path rules; the allowlist controls query params.
* `canonical_host` is validated to be host-only.
