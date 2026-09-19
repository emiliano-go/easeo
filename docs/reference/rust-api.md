---
title: "Rust API"
description: "The easeo-core Rust crate: functions, types, payload methods, and registry."
---

# Rust API { #rust-api }

The Rust crate is `easeo-core`. It contains all the logic; the Python and
JavaScript packages are thin bindings over it. Use it directly when you are
building a Rust service or another language binding.

## Cargo { #cargo }

```toml
[dependencies]
easeo-core = { path = "../easeo/crates/easeo-core" }
```

The crate has no I/O dependencies. Its dependency set is `serde`,
`serde_json`, `url`, `sha2`, and `thiserror`.

## The primary function { #primary }

```rust
pub fn build_seo_payload(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
) -> Result<SEOPayload, EaseoError>
```

With overrides:

```rust
pub fn build_seo_payload_with_overrides(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: &SEOOverrides,
) -> Result<SEOPayload, EaseoError>
```

## Example { #example }

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

let payload = build_seo_payload(&entity, "/blog/hello", &config)?;
assert_eq!(payload.canonical, "https://example.com/blog/hello");
```

## Types { #types }

| Type | Purpose |
|---|---|
| `SEOConfig` | Site-wide configuration |
| `SEOEntity` | Content entity, with `EntityType` |
| `SEOOverrides` | Per-call overrides |
| `SEOPayload` | Output, with `to_dict`, `to_json`, render methods |
| `OGPayload`, `TwitterPayload` | Nested payload groups |
| `SEOImage`, `Robots`, `Breadcrumb`, `FAQItem` | Value types |
| `SEOContract`, `SEOContractConfig` | Contract model |
| `EaseoError` | Error enum |

## Payload methods { #payload-methods }

```rust
payload.to_dict()?;        // serde_json::Value
payload.to_json()?;        // compact string
payload.to_json_pretty()?; // pretty string
payload.render_html()?;    // full <head> block
payload.render_opengraph();
payload.render_twitter();
payload.render_jsonld()?;
```

Hashing is free-standing:

```rust
use easeo_core::{hashing, SEOPayload};

let hash = hashing::hash_payload(&payload)?;
let etag = hashing::etag_payload(&payload)?;
```

## Custom schemas { #schemas }

The Rust registry accepts closures:

```rust
use easeo_core::registry::SchemaRegistry;

let mut registry = SchemaRegistry::new();
registry.register("Podcast", |ctx| {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Podcast",
        "name": ctx.title,
    })
});
```

Pass the registry through the lower-level `payload::build_seo_payload`
function when you need it during a build.

## Errors { #errors }

`EaseoError` is a `thiserror` enum with variants `InvalidUrl`,
`InvalidConfiguration`, `InvalidEntity`, `InvalidSchema`, `SerializationError`,
`ContractError`, and `URLPolicyError`. See [Errors](errors.md#errors).

## Recap { #recap }

* `easeo-core` holds all logic and has no I/O.
* The Python and JavaScript packages call into this crate.
* Rust closures can be registered as schema generators.
