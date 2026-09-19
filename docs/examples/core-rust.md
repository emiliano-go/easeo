---
title: "Rust"
description: "Rust examples for easeo-core, from a minimal payload to schema registries and hashing."
---

# Rust { #rust }

## Simple: a minimal payload { #rust-simple }

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

## Simple: render and hash { #rust-render }

```rust
println!("{}", payload.render_html()?);

let hash = easeo_core::hashing::hash_payload(&payload)?;
let etag = easeo_core::hashing::etag_payload(&payload)?;
```

## Complex: full configuration { #rust-config }

```rust
use easeo_core::{Robots, SEOConfig, SEOImage, URLPolicy, TrailingSlash};

let config = SEOConfig {
    canonical_host: "shop.example.com".into(),
    public_base_url: "https://shop.example.com".into(),
    url_policy: URLPolicy {
        enforce_https: true,
        lowercase_paths: true,
        trailing_slash: TrailingSlash::Never,
        collapse_duplicate_slashes: true,
        strip_tracking_params: true,
        allowed_query_params: vec!["page".into(), "q".into()],
    },
    site_name: Some("Example Shop".into()),
    title_template: Some("{title} - Example Shop".into()),
    default_robots: Robots { index: true, follow: true, ..Default::default() },
    default_og_image: Some(SEOImage {
        url: "https://shop.example.com/assets/og-image.png".into(),
        width: Some(1200),
        height: Some(630),
        alt: Some("Example Shop".into()),
    }),
    publisher_name: Some("Example Shop".into()),
    locale: Some("en_US".into()),
    search_url_template: Some(
        "https://shop.example.com/search?q={search_term_string}".into(),
    ),
    ..Default::default()
};
```

## Complex: overrides { #rust-overrides }

```rust
use easeo_core::{SEOOverrides, build_seo_payload_with_overrides};

let overrides = SEOOverrides {
    meta_title: Some("A one-off title".into()),
    skip_title_template: true,
    ..Default::default()
};

let payload = build_seo_payload_with_overrides(&entity, "/blog/hello", &config, &overrides)?;
```

## Complex: a custom schema generator { #rust-registry }

```rust
use easeo_core::registry::SchemaRegistry;

let mut registry = SchemaRegistry::new();
registry.register("Podcast", |ctx| {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Podcast",
        "name": ctx.title,
        "url": ctx.canonical,
    })
});
```

The registry is consumed through the lower-level
`easeo_core::payload::build_seo_payload(entity, route, config, overrides, registry)`
function.

## Complex: a data-driven build { #rust-data }

```rust
use easeo_core::{SEOEntity, EntityType, build_seo_payload};

struct Product {
    name: String,
    slug: String,
    description: String,
    price: String,
    sku: String,
}

fn seo_for(product: &Product, config: &SEOConfig) -> Result<SEOPayload, EaseoError> {
    let entity = SEOEntity {
        entity_type: EntityType::Product,
        title: Some(product.name.clone()),
        excerpt: Some(product.description.clone()),
        slug: Some(product.slug.clone()),
        sku: Some(product.sku.clone()),
        price: Some(product.price.clone()),
        price_currency: Some("USD".into()),
        availability: Some("InStock".into()),
        ..Default::default()
    };

    build_seo_payload(&entity, &format!("/products/{}", product.slug), config)
}
```

## Complex: tests { #rust-tests }

```rust
#[test]
fn seo_is_deterministic() {
    let a = build_seo_payload(&entity, "/blog/hello", &config).unwrap();
    let b = build_seo_payload(&entity, "/blog/hello", &config).unwrap();
    assert_eq!(a, b);
    assert_eq!(
        easeo_core::hashing::hash_payload(&a).unwrap(),
        easeo_core::hashing::hash_payload(&b).unwrap()
    );
}
```

## Related { #related }

* [Rust API reference](../reference/rust-api.md)
* [Fallback chains](../concepts/fallback-chains.md)
