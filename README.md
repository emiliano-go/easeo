<p align="center">
  <strong style="font-size: 2.5em;">easeo</strong>
</p>

<p align="center">
  <strong>Deterministic SEO metadata generation for content platforms.</strong>
</p>

<p align="center">
  Generate canonical URLs, titles, descriptions, robots directives, Open Graph,
  Twitter Cards, and JSON-LD from content entities. One Rust core with Python,
  Node.js, and Rust APIs.
</p>

<p align="center">
  <a href="https://pypi.org/project/easeo/">
    <img src="https://img.shields.io/pypi/v/easeo?logo=pypi&logoColor=white&style=for-the-badge&cacheSeconds=300" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@easeo/core">
    <img src="https://img.shields.io/npm/v/@easeo/core?logo=npm&logoColor=white&style=for-the-badge&cacheSeconds=300" alt="npm">
  </a>
  <a href="https://crates.io/crates/easeo-core">
    <img src="https://img.shields.io/crates/v/easeo-core?logo=rust&logoColor=white&style=for-the-badge&cacheSeconds=300" alt="crates.io">
  </a>
  <a href="https://www.python.org/downloads/">
    <img src="https://img.shields.io/badge/Python-3.10%2B-3776AB?logo=python&logoColor=white&style=for-the-badge" alt="Python">
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.88%2B-000000?logo=rust&logoColor=white&style=for-the-badge" alt="Rust">
  </a>
  <a href="https://github.com/emiliano-go/easeo/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/emiliano-go/easeo/ci.yml?branch=master&style=for-the-badge&logo=github&label=Tests" alt="Tests">
  </a>
  <a href="https://easeo.emiliano-go.com/">
    <img src="https://img.shields.io/badge/Docs-easeo.emiliano--go.com-8A2BE2?style=for-the-badge&logo=readthedocs" alt="Docs">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-10AC84?style=for-the-badge" alt="License">
  </a>
</p>

## What is easeo

`easeo` is a deterministic SEO metadata generator. It takes a content entity, the route it lives at, and a site-wide configuration, and returns a complete SEO payload: canonical URL, title, description, robots directives, Open Graph, Twitter Cards, and JSON-LD.

All logic lives in the Rust core (`easeo-core`). The Python and Node.js packages wrap the same engine, so the same entity, route, and config produce identical output in every language. The cross-language conformance suite asserts that byte for byte.

The [tutorial](https://easeo.emiliano-go.com/tutorial/) walks through installation and a first payload. [Concepts](https://easeo.emiliano-go.com/concepts/) explains the entity model, fallback chains, URL normalization, and validation rules.

```text
Content Entity
      |
      v
   easeo
      |
      +-- canonical URL
      +-- title & description
      +-- robots directives
      +-- Open Graph
      +-- Twitter Cards
      +-- JSON-LD
      |
      v
 Deterministic Payload
```

## Quick start

Every binding exposes the same call: pass an entity, a route, and a config, and get a payload back. The full [tutorial](https://easeo.emiliano-go.com/tutorial/) covers configuration, overrides, rendering, and contracts.

### Python

```bash
pip install easeo
```

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

```bash
npm install @easeo/core
```

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

```bash
cargo add easeo-core
```

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

## Why easeo

Other SEO libraries do too much (scoring, keyword analysis, content rewriting), while `easeo` does one thing and does it well: generate deterministic SEO metadata from content entities.

Given the same inputs, `easeo` always produces the same output. No timestamps, no randomness, no environment reads, no hidden I/O. This makes SEO metadata something you can:

- snapshot test: assert against expected payloads
- hash: generate stable ETags
- cache: safely memoize results
- diff: detect changes across builds
- validate in CI: commit your SEO intent as code with [contracts](https://easeo.emiliano-go.com/guides/contracts-in-ci/)

See [Why easeo](https://easeo.emiliano-go.com/about/why-easeo/) for the design goals and a [comparison](https://easeo.emiliano-go.com/about/comparison/) with other SEO tooling.

## What easeo is not

- Not a scoring or keyword analysis tool.
- Not a crawler or site auditor.
- Not a content rewriter or generator.
- Not a runtime service. It is a pure function over its inputs.
- Not a full page renderer. It renders a head snippet, not a document.

## Features

- 11 entity types: home, post, page, video, taxonomy, search, product, organization, local_business, faq, other. See the [entity model](https://easeo.emiliano-go.com/concepts/entity-model/)
- URL normalization: HTTPS enforcement, lowercase paths, trailing slash control, duplicate slash collapse, tracking parameter removal. See [URL normalization](https://easeo.emiliano-go.com/concepts/url-normalization/)
- JSON-LD schemas: WebSite, WebPage, Article, Product, Organization, LocalBusiness, FAQPage, BreadcrumbList, plus custom schemas via registry. See [JSON-LD schemas](https://easeo.emiliano-go.com/concepts/schemas/) and [custom JSON-LD](https://easeo.emiliano-go.com/guides/custom-schemas/)
- Open Graph: full og:type, title, description, image, locale, audio, video
- Twitter Cards: summary_large_image by default, with image alt, site, creator
- Robots directives: index/noindex, follow/nofollow, max-snippet, max-image-preview, max-video-preview
- SEO contracts: machine-readable SEO intent for CI validation. See the [contract reference](https://easeo.emiliano-go.com/reference/contracts/)
- Validation: built-in checks for title length, description, canonical format, OG image, robots directives. See [validation](https://easeo.emiliano-go.com/concepts/validation/)
- Hashing: SHA-256 payload hashing and ETag generation
- HTML rendering: complete head snippet with meta tags, OG, Twitter, JSON-LD
- Extension points: config-scoped hooks and schema registries. See [hooks](https://easeo.emiliano-go.com/guides/hooks/)

Learn more in the [guides](https://easeo.emiliano-go.com/guides/) and start from a [recipe](https://easeo.emiliano-go.com/recipes/) such as a blog post, product page, or search results page.

## Packages

| Package | Registry | Purpose |
|---------|----------|---------|
| `easeo` | PyPI | Python bindings for the Rust core |
| `easeo[fastapi]`, `easeo[django]`, `easeo[flask]`, `easeo[zensical]` | PyPI extras | Framework adapters and the docs extension |
| `@easeo/core` | npm | Node.js bindings for the Rust core |
| `easeo-core` | crates.io | Pure Rust engine with no I/O dependencies |

## Integrations

Per-framework setup guides live in the [Integrations docs](https://easeo.emiliano-go.com/integrations/).

### JavaScript / TypeScript

| Framework | Package | API |
|-----------|---------|-----|
| Next.js | `@easeo/next` | `easeoMetadata()` returns a Next.js Metadata object |
| Astro | `@easeo/astro` | Build-time integration with contract emission |
| Vite | `@easeo/vite` | `transformIndexHtml` hook |
| Nuxt | `@easeo/nuxt` | `useEaseoSeo()` composable |
| SvelteKit | `@easeo/sveltekit` | `buildEaseoPayload()` and `<EaseoHead />` |
| React | `@easeo/react` | `<EaseoHead />` component |

### Python

| Framework | Import |
|-----------|--------|
| FastAPI | `from easeo.adapters.fastapi import EaseoSEO` |
| Django | `from easeo.adapters.django import seo_head` |
| Flask | `from easeo.adapters.flask import Easeo` |
| Zensical | `from easeo.contrib.zensical import EaseoExtension` |

## API

| Symbol | Description |
|--------|-------------|
| `build_seo_payload(entity, route_path, config, overrides=None)` | Builds the deterministic `SEOPayload` |
| `SEOConfig` | Site-wide configuration: host, base URL, defaults, hooks, registries |
| `SEOEntity` | Content entity: type, title, excerpt, images, dates, breadcrumbs, FAQ, product fields |
| `SEOOverrides` | Highest-precedence per-call overrides |
| `SEOPayload` | Output with `render_html()`, `to_dict()`, `to_json()`, `hash()`, and `etag()` |
| `URLPolicy` | Canonical URL normalization policy |
| `build_seo_contract` | Builds a machine-readable SEO contract |
| `validate_payload` | Runs the built-in validation checks |
| `HookRegistry`, `SchemaRegistry` | Config-scoped post-processing and JSON-LD generators |
| `SEOEntityBuilder`, `from_blog_post`, `from_product`, `from_faq` | Ergonomic entity constructors |

Names are shown in Python. JavaScript uses camelCase and Rust uses snake_case. Full signatures live in the [Python](https://easeo.emiliano-go.com/reference/python-api/), [JavaScript](https://easeo.emiliano-go.com/reference/javascript-api/), and [Rust](https://easeo.emiliano-go.com/reference/rust-api/) API references.

## Documentation

- [Tutorial](https://easeo.emiliano-go.com/tutorial/): installation, first payload, rendering, contracts, configuration
- [Concepts](https://easeo.emiliano-go.com/concepts/): determinism, entity and payload models, fallback chains, URL normalization, schemas, validation
- [Guides](https://easeo.emiliano-go.com/guides/): custom JSON-LD, hooks, contracts in CI, framework recipes
- [Reference](https://easeo.emiliano-go.com/reference/python-api/): Python, JavaScript, and Rust APIs, contracts, errors
- [Integrations](https://easeo.emiliano-go.com/integrations/): per-framework setup
- [Recipes](https://easeo.emiliano-go.com/recipes/): blog post, product page, category page, search results
- [Examples](https://easeo.emiliano-go.com/examples/): core and framework examples for every binding

LLM-friendly versions: [llms.txt](https://easeo.emiliano-go.com/llms.txt) and [llms-full.txt](https://easeo.emiliano-go.com/llms-full.txt).

## Building

```bash
# Rust core
cargo build --workspace --release

# Python bindings
pip install maturin
maturin develop -m crates/easeo-python/Cargo.toml

# Node bindings
npm install -g @napi-rs/cli@^3
cd packages/core
napi build --platform --release --manifest-path ../../crates/easeo-node/Cargo.toml
cp ../../crates/easeo-node/*.node .
```

## Testing

```bash
# Rust
cargo test --workspace

# Python
pytest tests/python/

# JavaScript
node --test tests/javascript/*.cjs

# Cross-language conformance
python tests/conformance/test_conformance.py
```

## Next steps

- Follow the [tutorial](https://easeo.emiliano-go.com/tutorial/) from installation to first payload
- Commit SEO intent with [contracts in CI](https://easeo.emiliano-go.com/guides/contracts-in-ci/)
- Extend structured data with [custom schemas](https://easeo.emiliano-go.com/guides/custom-schemas/)
- Read how fields resolve in [fallback chains](https://easeo.emiliano-go.com/concepts/fallback-chains/)
- Browse [examples](https://easeo.emiliano-go.com/examples/) for every binding and framework

## Contributors

- [Emiliano Gandini Outeda](https://github.com/emiliano-go): creator and maintainer

## License

MIT
