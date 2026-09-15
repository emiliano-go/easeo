# Migration from seoslug

## Overview

easeo is the Rust rewrite of seoslug. The core API is similar but the implementation is now Rust with Python and JavaScript bindings.

## API Mapping

| seoslug | easeo |
|---------|-------|
| `seoslug.SEOConfig` | `easeo.SEOConfig` |
| `seoslug.SEOEntity` | `easeo.SEOEntity` |
| `seoslug.build_seo_payload()` | `easeo.build_seo_payload()` |
| `seoslug.URLPolicy` | `easeo.URLPolicy` |
| `seoslug.SchemaRegistry` | Rust-only in easeo; use `SEOOverrides(schema_jsonld={...})` from Python |
| `seoslug.hook` | Removed (hooks conflict with determinism) |
| `seoslug.factories` | May be added later |

## What Changed

- Rust core instead of pure Python
- Contract system added
- detrack absorbed into core
- Hooks system removed
- Framework adapters are separate packages

## What Stays the Same

- `build_seo_payload(entity, route, config)` signature
- Deterministic output
- Framework-agnostic core
- `render_html()`, `to_dict()`, `hash()`, `etag()`
- Schema registry

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
