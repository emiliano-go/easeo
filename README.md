<h1 align="center">easeo</h1>

<p align="center">
  <strong>Deterministic SEO metadata generation for content platforms.</strong>
</p>

<p align="center">
  <a href="https://pypi.org/project/easeo/">
    <img src="https://img.shields.io/pypi/v/easeo?logo=pypi&logoColor=white&style=for-the-badge" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@easeo/core">
    <img src="https://img.shields.io/npm/v/@easeo/core?logo=npm&logoColor=white&style=for-the-badge" alt="npm">
  </a>
  <a href="https://www.python.org/downloads/">
    <img src="https://img.shields.io/pypi/pyversions/easeo?logo=python&logoColor=white&style=for-the-badge" alt="Python">
  </a>
  <a href="https://github.com/emiliano-go/easeo">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust core">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/pypi/l/easeo?logo=opensourceinitiative&logoColor=white&style=for-the-badge" alt="License">
  </a>
</p>

## Install

```bash
# Python
pip install easeo

# Node.js
npm install @easeo/core
```

## Quick start

### Python

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

### JavaScript / TypeScript

```typescript
import { buildSeoPayload } from "@easeo/core";

const payload = buildSeoPayload(
  { entityType: "post", title: "Hello World", description: "An example post." },
  "/blog/hello",
  { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
);

console.log(payload.renderHtml());
```

### Rust

```rust
use easeo_core::{SEOConfig, SEOEntity, EntityType, build_seo_payload};

let config = SEOConfig {
    canonical_host: "example.com".into(),
    public_base_url: "https://example.com".into(),
    ..Default::default()
};

let entity = SEOEntity {
    entity_type: EntityType::Post,
    title: Some("Hello World".into()),
    excerpt: Some("An example post.".into()),
    ..Default::default()
};

let payload = build_seo_payload(&entity, "/blog/hello", &config).unwrap();
assert_eq!(payload.canonical, "https://example.com/blog/hello");
```

## Why easeo?

Other SEO libraries do too much (scoring, keyword analysis, content rewriting), while `easeo` does one thing and does it well: **generate deterministic SEO metadata from content entities**.

Given the same inputs, `easeo` always produces the same output. No timestamps, no randomness, no environment reads, no hidden I/O. This makes SEO metadata something you can:

- **snapshot test** — assert against expected payloads
- **hash** — generate stable ETags
- **cache** — safely memoize results
- **diff** — detect changes across builds
- **validate in CI** — commit your SEO intent as code

```text
Content Entity
      │
      ▼
   easeo
      │
      ├── canonical URL
      ├── title & description
      ├── robots directives
      ├── Open Graph
      ├── Twitter Cards
      └── JSON-LD
      │
      ▼
 Deterministic Payload
```

## Architecture

- **Rust core** (`easeo-core`) — all logic, zero dependencies
- **Python bindings** (`easeo-python`) — via PyO3
- **Node bindings** (`easeo-node`) — via napi-rs
- **Framework integrations** — Next.js, Astro, Vite, Nuxt, SvelteKit, React

The Rust core guarantees identical behavior across languages. Same entity, same route, same config → same payload everywhere.

## Features

- **11 entity types**: home, post, page, video, taxonomy, search, product, organization, local_business, faq, other
- **URL normalization**: HTTPS enforcement, lowercase paths, trailing slash control, duplicate slash collapse, tracking parameter removal
- **JSON-LD schemas**: Article, Product, Organization, LocalBusiness, FAQPage, WebPage, BreadcrumbList, plus custom schemas via registry
- **Open Graph**: full og:type, title, description, image, locale, audio, video
- **Twitter Cards**: summary_large_image by default, with image alt, site, creator
- **Robots directives**: index/noindex, follow/nofollow, max-snippet, max-image-preview, max-video-preview
- **SEO contracts**: machine-readable SEO intent for CI validation
- **Validation**: built-in checks for title length, description, canonical format, OG image, robots directives
- **Hashing**: SHA-256 payload hashing and ETag generation
- **HTML rendering**: complete `<head>` snippet with meta tags, OG, Twitter, JSON-LD

## Framework integrations

| Framework | Package | API |
|-----------|---------|-----|
| Next.js | `@easeo/next` | `easeoMetadata()` returns Next.js Metadata object |
| Astro | `@easeo/astro` | Build-time integration with contract emission |
| Vite | `@easeo/vite` | `transformIndexHtml` hook |
| Nuxt | `@easeo/nuxt` | `useEaseoSeo()` composable |
| SvelteKit | `@easeo/sveltekit` | `buildEaseoPayload()` function |
| React | `@easeo/react` | `<EaseoHead />` component |

## Python framework adapters

| Framework | Import |
|-----------|--------|
| FastAPI | `from easeo.adapters.fastapi import EaseoSEO` |
| Django | `from easeo.adapters.django import seo_head` |
| Flask | `from easeo.adapters.flask import Easeo` |
| Zensical | `from easeo.contrib.zensical import EaseoExtension` |

## API

### `build_seo_payload(entity, route_path, config)`

The core function. Takes an entity, route, and config. Returns a deterministic `SEOPayload`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `SEOEntity` | Content entity with type, title, excerpt, images, etc. |
| `route_path` | `str` | Route path, e.g. `"/blog/hello"` |
| `config` | `SEOConfig` | Site-wide configuration (host, base URL, defaults) |

**Returns:** `SEOPayload` → structured payload with all SEO fields.

---

### `SEOConfig`

Site-wide configuration.

| Field | Type | Description |
|-------|------|-------------|
| `canonical_host` | `str` | Canonical hostname, e.g. `"example.com"` |
| `public_base_url` | `str` | Full base URL for path resolution |
| `default_title` | `str \| None` | Fallback title when entity has none |
| `default_description` | `str \| None` | Fallback description |
| `default_og_image` | `str \| None` | Fallback OG image URL |
| `title_template` | `str \| None` | Template for title, e.g. `"%s | My Site"` |
| `robots` | `Robots \| None` | Default robots directives |
| `schema_type_map` | `dict \| None` | Override entity-to-Schema.org type mapping |

---

### `SEOEntity`

Content entity representation.

| Field | Type | Description |
|-------|------|-------------|
| `entity_type` | `str` | One of: `home`, `post`, `page`, `video`, `taxonomy`, `search`, `product`, `organization`, `local_business`, `faq`, `other` |
| `title` | `str \| None` | Page title |
| `excerpt` | `str \| None` | Short description |
| `featured_image` | `str \| None` | Primary image URL |
| `url` | `str \| None` | Explicit URL override |
| `og` | `OGOverrides \| None` | Open Graph overrides |
| `twitter` | `TwitterOverrides \| None` | Twitter Card overrides |
| `breadcrumbs` | `list[Breadcrumb] \| None` | Breadcrumb trail |
| `schema_extra` | `dict \| None` | Additional JSON-LD properties |

---

### `SEOPayload`

The output. Structured, hashable, renderable.

| Field | Type | Description |
|-------|------|-------------|
| `title` | `str` | Resolved title |
| `description` | `str` | Resolved description |
| `canonical` | `str` | Normalized canonical URL |
| `robots` | `Robots` | Resolved robots directives |
| `og` | `OGPayload` | Open Graph metadata |
| `twitter` | `TwitterPayload` | Twitter Card metadata |
| `schema_jsonld` | `dict \| None` | JSON-LD structured data |

**Methods:**

| Method | Description |
|--------|-------------|
| `render_html()` | Full `<head>` HTML snippet |
| `to_dict()` | Payload as dictionary |
| `hash()` | SHA-256 hash of the payload |
| `etag()` | HTTP ETag string |

---

### `SchemaRegistry`

Register custom Schema.org types for arbitrary entity types.

```python
from easeo import SchemaRegistry

registry = SchemaRegistry()
registry.register("Podcast", lambda entity, config, canonical, title, desc, og: {
    "@type": "Podcast",
    "name": title,
})
```

---

### `SEOEntityBuilder`

Fluent builder for complex entities.

```python
from easeo import SEOEntityBuilder

entity = (
    SEOEntityBuilder("post")
    .title("Hello World")
    .excerpt("An example post.")
    .featured_image("https://example.com/hero.jpg")
    .breadcrumb("Home", "/")
    .breadcrumb("Blog", "/blog")
    .build()
)
```

---

### `URLPolicy`

Controls canonical URL normalization.

```python
from easeo import URLPolicy

policy = URLPolicy(
    enforce_https=True,
    lowercase_paths=True,
    trailing_slash="never",
    collapse_duplicate_slashes=True,
    strip_tracking_params=True,
    allowed_query_params=["page", "q"],
)
```

---

### `build_seo_payload_async(entity, route_path, config)`

Async variant. Useful for async framework integrations.

```python
payload = await build_seo_payload_async(entity, "/blog/hello", config)
```

---

### `build_seo_payload_dict(entity, route_path, config)`

Returns the payload as a plain dictionary. Useful for JSON serialization.

```python
data = build_seo_payload_dict(entity, "/blog/hello", config)
```

---

## Fallback resolution

easeo resolves SEO fields through an explicit precedence chain:

```text
SEOOverrides       ← highest priority
       ↓
SEOEntity
       ↓
SEOConfig
       ↓
hardcoded defaults ← lowest priority
```

For example:

```text
config.default_og_image
        ↓
entity.featured_image
        ↓
overrides.og_image
```

This avoids both "everything is global" and "everything is magically inferred" patterns.

## Development

### Prerequisites

- Rust (stable)
- Python 3.10+
- Node.js 18+

### Build

```bash
# Rust core
cargo build --release

# Python bindings
pip install maturin
maturin develop -p easeo-python

# Node bindings
cd crates/easeo-node
napi build --release
```

### Test

```bash
# Rust tests
cargo test -p easeo-core

# Python conformance
python tests/conformance/test_conformance.py
```

## License

MIT
