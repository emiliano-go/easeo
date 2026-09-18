---
title: "Migration from seoslug"
---

# Migration from seoslug { #migration-from-seoslug }

## Overview

easeo is the Rust rewrite of seoslug. The core API is similar but the implementation is now Rust with Python and JavaScript bindings.

## API Mapping

| seoslug | easeo |
|---------|-------|
| `seoslug.SEOConfig` | `easeo.SEOConfig` |
| `seoslug.SEOEntity` | `easeo.SEOEntity` |
| `seoslug.build_seo_payload(entity, route, config, overrides=None)` | same signature |
| `seoslug.build_seo_payload_dict()` | `easeo.build_seo_payload_dict()` |
| `seoslug.build_seo_payload_async()` | `easeo.build_seo_payload_async()` |
| `seoslug.URLPolicy` | `easeo.URLPolicy` |
| `seoslug.SchemaRegistry` | `easeo.SchemaRegistry` (callables supported) |
| `seoslug.hook` / `register_hook` | `easeo.HookRegistry` (config-scoped) |
| `seoslug.factories` | `from_blog_post`, `from_product`, `from_faq` |
| `seoslug.SEOError` | `easeo.EaseoError` |
| `payload["title"]`, `payload == dict` | supported |

## What Changed

- Rust core instead of pure Python
- Contract system added
- detrack absorbed into core
- **Hooks are config-scoped only.** seoslug had a global registry; easeo attaches
  hooks to `SEOConfig` so the builder stays pure and deterministic.
- **`SchemaRegistry.register()` accepts callables.** It is no longer Rust-only.
- Framework adapters are separate packages

## What Stays the Same

- `build_seo_payload(entity, route, config, overrides=None)` signature
- Deterministic output
- Framework-agnostic core
- `render_html()`, `to_dict()`, `hash()`, `etag()`
- Schema registry
- Dict-compatible, comparable payloads
- `except ValueError` still catches easeo errors

## Installation

```bash
# Old
pip install seoslug

# New
pip install easeo
```

## Code Changes

Minimal changes required:

```python
# Old
from seoslug import SEOConfig, SEOEntity, build_seo_payload

# New
from easeo import SEOConfig, SEOEntity, build_seo_payload
```

The API is intentionally compatible.
