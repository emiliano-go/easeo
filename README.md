# easeo

Deterministic SEO payload generation for content platforms.

## What it does

easeo takes a content entity, a route, and a configuration, then produces a
deterministic SEO payload (title, description, canonical URL, robots directives,
Open Graph, Twitter Cards, JSON-LD schema).

Same input always produces the same output. No timestamps, no randomness, no
environment dependency.

## Architecture

- **Rust core** (`easeo-core`) handles all logic
- **Python bindings** (`easeo-python`) via PyO3
- **Node bindings** (`easeo-node`) via napi-rs
- **Framework integrations** for Next.js, Astro, Vite, Nuxt, SvelteKit, React

## Quick start

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

## Features

- **11 entity types**: home, post, page, video, taxonomy, search, other,
  product, organization, local\_business, faq
- **URL normalization**: HTTPS enforcement, lowercase paths, trailing slash
  control, duplicate slash collapse, tracking parameter removal
- **JSON-LD schemas**: Article, Product, Organization, LocalBusiness, FAQPage,
  WebPage, BreadcrumbList, plus custom schemas via registry
- **Open Graph**: full og:type, title, description, image, locale, audio, video
- **Twitter Cards**: summary\_large\_image by default, with image alt, site,
  creator
- **Robots directives**: index/noindex, follow/nofollow, max-snippet,
  max-image-preview, max-video-preview
- **SEO contracts**: machine-readable SEO intent for CI validation
- **Validation**: built-in checks for title length, description, canonical
  format, OG image, robots directives
- **Hashing**: SHA-256 payload hashing and ETag generation
- **HTML rendering**: complete head snippet with meta tags, OG, Twitter, JSON-LD

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

## Installation

### Python

```bash
pip install easeo
```

### npm

```bash
npm install @easeo/core
```

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
