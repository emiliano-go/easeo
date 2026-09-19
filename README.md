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

## Why easeo

Other SEO libraries do too much (scoring, keyword analysis, content rewriting),
while `easeo` does one thing and does it well: **generate deterministic SEO
metadata from content entities**.

Given the same inputs, `easeo` always produces the same output. No timestamps,
no randomness, no environment reads, no hidden I/O. This makes SEO metadata
something you can:

- **snapshot test**: assert against expected payloads
- **hash**: generate stable ETags
- **cache**: safely memoize results
- **diff**: detect changes across builds
- **validate in CI**: commit your SEO intent as code

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

## Architecture

- **Rust core** (`easeo-core`): all logic, no I/O dependencies
- **Python bindings** (`easeo-python`): via PyO3
- **Node bindings** (`easeo-node`): via napi-rs
- **Framework integrations**: Next.js, Astro, Vite, Nuxt, SvelteKit, React,
  FastAPI, Django, Flask, Zensical

The Rust core guarantees identical behavior across languages. Same entity, same
route, same config, same payload everywhere.

## Features

- **11 entity types**: home, post, page, video, taxonomy, search, product,
  organization, local_business, faq, other
- **URL normalization**: HTTPS enforcement, lowercase paths, trailing slash
  control, duplicate slash collapse, tracking parameter removal
- **JSON-LD schemas**: Article, Product, Organization, LocalBusiness, FAQPage,
  WebPage, BreadcrumbList, plus custom schemas via registry
- **Open Graph**: full og:type, title, description, image, locale, audio, video
- **Twitter Cards**: summary_large_image by default, with image alt, site,
  creator
- **Robots directives**: index/noindex, follow/nofollow, max-snippet,
  max-image-preview, max-video-preview
- **SEO contracts**: machine-readable SEO intent for CI validation
- **Validation**: built-in checks for title length, description, canonical
  format, OG image, robots directives
- **Hashing**: SHA-256 payload hashing and ETag generation
- **HTML rendering**: complete `<head>` snippet with meta tags, OG, Twitter,
  JSON-LD
- **Extension points**: config-scoped hooks and schema registries

## Framework integrations

| Framework | Package | API |
|-----------|---------|-----|
| Next.js | `@easeo/next` | `easeoMetadata()` returns a Next.js Metadata object |
| Astro | `@easeo/astro` | Build-time integration with contract emission |
| Vite | `@easeo/vite` | `transformIndexHtml` hook |
| Nuxt | `@easeo/nuxt` | `useEaseoSeo()` composable |
| SvelteKit | `@easeo/sveltekit` | `buildEaseoPayload()` and `<EaseoHead />` |
| React | `@easeo/react` | `<EaseoHead />` component |

## Python framework adapters

| Framework | Import |
|-----------|--------|
| FastAPI | `from easeo.adapters.fastapi import EaseoSEO` |
| Django | `from easeo.adapters.django import seo_head` |
| Flask | `from easeo.adapters.flask import Easeo` |
| Zensical | `from easeo.contrib.zensical import EaseoExtension` |

## API

### `build_seo_payload(entity, route_path, config, overrides=None)`

The core function. Takes an entity, route, and config. Returns a deterministic
`SEOPayload`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `SEOEntity` | Content entity with type, title, excerpt, images, etc. |
| `route_path` | `str` | Route path, e.g. `"/blog/hello"` |
| `config` | `SEOConfig` | Site-wide configuration (host, base URL, defaults) |
| `overrides` | `SEOOverrides \| None` | Per-call overrides (highest precedence) |

**Returns:** `SEOPayload`, a structured payload with all SEO fields.

---

### `SEOConfig`

Site-wide configuration.

| Field | Type | Description |
|-------|------|-------------|
| `canonical_host` | `str` | Canonical hostname, e.g. `"example.com"` (host only, no scheme) |
| `public_base_url` | `str` | Full base URL for path resolution, e.g. `"https://example.com"` |
| `url_policy` | `URLPolicy \| None` | URL normalization policy (HTTPS, lowercase, trailing slash, tracking params) |
| `default_robots` | `Robots \| None` | Default robots directives for regular pages |
| `search_robots` | `Robots \| None` | Robots directives for search-type pages (default: noindex,follow) |
| `default_og_image` | `SEOImage \| None` | Fallback OG image |
| `site_name` | `str \| None` | Site name (og:site_name, title template) |
| `title_template` | `str \| None` | Template for titles, e.g. `"{title} - My Site"` |
| `publisher_name` | `str \| None` | Organization/publisher name for schemas |
| `publisher_logo` | `str \| None` | Publisher logo URL for schemas |
| `locale` | `str \| None` | og:locale, e.g. `"en_US"` |
| `locale_alternate` | `list[str] \| None` | Alternate locales |
| `twitter_site` | `str \| None` | Twitter @handle for twitter:site |
| `auto_generate_schema` | `bool` | Auto-generate JSON-LD from entity type (default: `True`) |
| `emit_warnings` | `bool` | Collect validation warnings (default: `False`) |
| `schema_type_map` | `dict \| None` | Override entity-type to schema.org type mapping |
| `hooks` | `HookRegistry \| None` | Config-scoped post-processing |
| `schema_registry` | `SchemaRegistry \| None` | Config-scoped JSON-LD generators |

---

### `SEOEntity`

Content entity representation.

| Field | Type | Description |
|-------|------|-------------|
| `entity_type` | `str` | One of: `home`, `post`, `page`, `video`, `taxonomy`, `search`, `product`, `organization`, `local_business`, `faq`, `other` |
| `title` | `str \| None` | Page title |
| `excerpt` | `str \| None` | Short description |
| `slug` | `str \| None` | Entity slug |
| `body_html` | `str \| None` | Full content (used to derive a description snippet when no excerpt) |
| `status` | `str \| None` | Publication status (non-published becomes noindex) |
| `featured_image` | `SEOImage \| None` | Primary image |
| `published_at` | `str \| None` | ISO date/datetime |
| `updated_at` | `str \| None` | ISO date/datetime |
| `author_name` | `str \| None` | Author display name |
| `breadcrumbs` | `list[Breadcrumb] \| None` | Breadcrumb trail (appended as BreadcrumbList JSON-LD) |
| `faq_items` | `list[FAQItem] \| None` | FAQ entries (FAQPage schema) |
| `sku` | `str \| None` | Product SKU |
| `price` | `str \| None` | Product price |
| `price_currency` | `str \| None` | ISO currency code, e.g. `"USD"` |
| `availability` | `str \| None` | Product availability |
| `same_as` | `list[str] \| None` | sameAs URLs (Organization schema) |
| `address` | `str \| None` | Address (LocalBusiness schema) |

---

### `SEOPayload`

The output. Structured, hashable, renderable.

| Field | Type | Description |
|-------|------|-------------|
| `title` | `str` | Resolved title |
| `description` | `str` | Resolved description |
| `canonical` | `str` | Normalized canonical URL |
| `robots` | `str` | Resolved robots directives, e.g. `"index,follow"` |
| `og` | `OGPayload` | Open Graph metadata |
| `twitter` | `TwitterPayload` | Twitter Card metadata |
| `schema_jsonld` | `dict \| None` | JSON-LD structured data |

**Methods:**

| Method | Description |
|--------|-------------|
| `render_html()` | Full `<head>` HTML snippet |
| `render_opengraph()` | OG meta tags only |
| `render_twitter()` | Twitter meta tags only |
| `render_jsonld()` | JSON-LD `<script>` tag only (with `<` escaped to be script-safe) |
| `to_dict()` | Payload as a canonical snake_case dictionary |
| `to_json()` | Payload as a canonical JSON string |
| `hash()` | SHA-256 hash of the payload |
| `etag()` | HTTP ETag string (quoted) |

Payloads are dict-compatible and comparable:

```python
payload["title"]
payload.get("title")
"title" in payload
payload == other_payload
payload == payload.to_dict()
```

---

### Custom JSON-LD schemas

Replace the schema for one page with overrides:

```python
from easeo import SEOOverrides, build_seo_payload

payload = build_seo_payload(
    entity,
    "/podcast/ep-1",
    config,
    SEOOverrides(schema_jsonld={
        "@context": "https://schema.org",
        "@type": "Podcast",
        "name": "My Podcast",
    }),
)
```

Register a generator for a whole schema type on the config:

```python
from easeo import SchemaRegistry

registry = SchemaRegistry()

@registry.register("Article")
def podcast_episode(entity, config, canonical, title, description, og_image):
    return {"@context": "https://schema.org", "@type": "Podcast", "name": title}

config = SEOConfig(..., schema_registry=registry)
```

---

### Extension points

**Hooks** post-process the payload. They are config-scoped, so the builder
stays a pure function of its inputs:

```python
from easeo import HookRegistry

hooks = HookRegistry()

@hooks.hook("post_process")
def add_generator(payload, entity, config):
    payload["generator"] = "easeo"
    return payload

config = SEOConfig(..., hooks=hooks)
```

The JavaScript equivalents are `HookRegistry` and `SchemaRegistry` on
`@easeo/core`, attached through `hooks` and `schemaRegistry` on the config.

---

### `SEOEntityBuilder`

Fluent builder for complex entities.

```python
from easeo import SEOEntityBuilder

entity = (
    SEOEntityBuilder("post")
    .title("Hello World")
    .excerpt("An example post.")
    .featured_image("https://example.com/hero.jpg", width=1200, height=630)
    .breadcrumb("Home", "/")
    .breadcrumb("Blog", "/blog")
    .build()
)
```

---

### Factories

Convenience constructors for common content types.

```python
from easeo import from_blog_post, from_product, from_faq

entity = from_blog_post(title="Hello", body_html="<p>Body</p>", author="Jane")
product = from_product(name="Widget", sku="W-1", price=9.99)
faq = from_faq(questions=[{"question": "Q?", "answer": "A."}])
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

### `build_seo_payload_dict(entity, route_path, config, overrides=None)`

Returns the payload as a plain dictionary. Useful for JSON serialization.

```python
data = build_seo_payload_dict(entity, "/blog/hello", config)
```

---

### `build_seo_payload_async(entity, route_path, config, overrides=None)`

Async wrapper that offloads the build to a thread pool. The Rust core releases
the GIL, so this genuinely runs off the event loop.

```python
from easeo import build_seo_payload_async

payload = await build_seo_payload_async(entity, "/blog/hello", config)
```

---

## Fallback resolution

easeo resolves SEO fields through an explicit precedence chain:

```text
SEOOverrides        highest priority
      |
SEOEntity
      |
SEOConfig
      |
hardcoded defaults  lowest priority
```

For example:

```text
config.default_og_image
      |
entity.featured_image
      |
overrides.og_image
```

This avoids both "everything is global" and "everything is magically inferred".

## Errors

All errors inherit from `EaseoError`, which inherits from `ValueError` in
Python and `Error` in JavaScript. Python code that catches `ValueError` keeps
working.

| Type | Raised when |
|------|-------------|
| `InvalidUrlError` | A URL is malformed or a URL policy is invalid |
| `ConfigurationError` | A config value fails validation |
| `EntityError` | An entity or overrides value fails validation |
| `SchemaError` | JSON-LD construction fails |
| `ContractError` | Contract generation fails |

## Documentation

Full documentation lives in `docs/` and builds with Zensical. The docs site
uses the easeo Zensical extension, so easeo must be importable by the
interpreter that runs Zensical. `uv` handles this in one command:

```bash
uv run zensical serve     # live preview on http://localhost:8000
uv run zensical build     # writes site/
```

Or with a manual environment:

```bash
pip install -e ".[zensical]"
zensical serve
```

Topics include a tutorial, concepts, guides, an API reference,
per-framework integrations, and recipes.

To reproduce the exact artifact that CI and Cloudflare Pages deploy (easeo
installed editable, `llms-full.txt` regenerated, then the build):

```bash
bash scripts/build_docs.sh     # output in site/
```

Cloudflare Pages settings: build command `bash scripts/build_docs.sh`, output
directory `site`, environment variable `PYTHON_VERSION=3.12` (a Rust toolchain
is required). See `docs/about/deploying-docs.md` for details.

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
maturin develop -m crates/easeo-python/Cargo.toml

# Node bindings
cd packages/core
napi build --platform --release --manifest-path ../../crates/easeo-node/Cargo.toml
cp ../../crates/easeo-node/*.node .
```

### Test

```bash
# Rust tests
cargo test --workspace

# Python tests
pytest tests/python/

# JavaScript tests
node --test tests/javascript/*.cjs

# Cross-language conformance
python tests/conformance/test_conformance.py
```

## License

MIT
