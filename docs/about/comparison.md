---
title: "Comparison"
description: "How easeo compares to building and maintaining SEO metadata by hand."
---

# Comparison: manual vs easeo { #comparison }

| Task | Manual | easeo |
|---|---|---|
| Canonical URL | Construct by hand | `build_seo_payload(entity, path, config)` |
| Open Graph tags | 10+ `<meta>` tags | `payload.og` or `payload.render_html()` |
| Twitter Cards | 6+ `<meta>` tags | `payload.twitter` or `payload.render_html()` |
| JSON-LD schema | Hand-written schema.org JSON | Auto-generated, extensible via registry |
| BreadcrumbList | Manual JSON-LD | `Breadcrumb(name, url)` auto-generates |
| URL normalization | HTTPS, slash, case logic | `URLPolicy` |
| HTML excerpt | Strip tags, decode, truncate | Built-in `body_html` snippet |
| Validation | Manual audit of lengths | `validate_payload()` / `emit_warnings` |
| HTML rendering | A template per tag | `payload.render_html()` |
| Content ETag | Manual hashing | `payload.etag()` |
| Testing | Manual fixtures | Deterministic: `payload == expected_dict` |
| Custom fields | Edit every template | Config-scoped `HookRegistry` |

## Why determinism matters

```python
p1 = build_seo_payload(entity, path, config)
p2 = build_seo_payload(entity, path, config)
assert p1 == p2  # always True
```

Most SEO tooling produces different output for identical input: timestamps,
cache busters, unstable dict ordering. easeo does none of that, so SEO becomes
a build artifact you can commit, diff, cache, and validate in CI.
