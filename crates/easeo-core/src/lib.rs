//! Deterministic SEO metadata generation for content platforms.
//!
//! `easeo-core` is a pure library with no I/O dependencies. Given the same
//! entity, route, and configuration it always produces the same payload, which
//! makes the output safe to snapshot, hash, cache, and diff.
//!
//! # Example
//!
//! ```
//! use easeo_core::{build_seo_payload, EntityType, SEOConfig, SEOEntity};
//!
//! let config = SEOConfig {
//!     canonical_host: "example.com".into(),
//!     public_base_url: "https://example.com".into(),
//!     ..Default::default()
//! };
//!
//! let entity = SEOEntity {
//!     entity_type: EntityType::Post,
//!     title: Some("Hello World".into()),
//!     ..Default::default()
//! };
//!
//! let payload = build_seo_payload(&entity, "/blog/hello", &config).unwrap();
//! assert_eq!(payload.canonical, "https://example.com/blog/hello");
//! ```

#![warn(missing_docs)]

/// Breadcrumb trail building.
pub mod breadcrumbs;
/// Canonical URL resolution.
pub mod canonical;
/// Site-wide configuration types.
pub mod config;
/// SEO contract definition and generation.
pub mod contract;
/// Tracking parameter removal.
pub mod detrack;
/// Content entity model.
pub mod entity;
/// Error types.
pub mod error;
/// Payload hashing and ETags.
pub mod hashing;
/// JSON-LD schema generation.
pub mod jsonld;
/// Open Graph payload types.
pub mod opengraph;
/// The SEO payload builder and output model.
pub mod payload;
/// Robots directive types.
pub mod robots;
/// HTML text extraction helpers.
pub mod text;
/// Twitter Card payload types.
pub mod twitter;
/// URL normalization helpers.
pub mod url;
/// Payload validation.
pub mod validation;

pub use config::{SEOConfig, TrailingSlash, URLPolicy};
pub use contract::{
    ContractGenerator, ContractSeverity, ContractSite, FieldExpectation, SEOContract,
    SEOContractConfig, SEOContractRule, SEOExpectation, SchemaExpectation,
};
pub use entity::{
    Breadcrumb, EntityType, FAQItem, Robots, SEOAuthor, SEOEntity, SEOImage, SEOOverrides,
};
pub use error::EaseoError;
pub use jsonld::registry;
pub use payload::{OGPayload, SEOPayload, TwitterPayload};

use entity as entity_mod;

/// Builds a deterministic SEO payload for an entity at a route.
///
/// # Errors
///
/// Returns [`EaseoError`] when the entity, route, or configuration is invalid.
pub fn build_seo_payload(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
) -> Result<SEOPayload, EaseoError> {
    payload::build_seo_payload(entity, route, config, None, None)
}

/// Builds a payload with per-call overrides, which take the highest precedence.
///
/// # Errors
///
/// Returns [`EaseoError`] when the entity, route, overrides, or configuration
/// is invalid.
pub fn build_seo_payload_with_overrides(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: &entity_mod::SEOOverrides,
) -> Result<SEOPayload, EaseoError> {
    payload::build_seo_payload(entity, route, config, Some(overrides), None)
}

/// Builds a machine-readable SEO contract from the given configuration.
///
/// # Errors
///
/// Returns [`EaseoError`] when the contract configuration is invalid.
pub fn build_seo_contract(
    config: &contract::SEOContractConfig,
) -> Result<contract::SEOContract, EaseoError> {
    contract::build_contract(config)
}

/// Runs the built-in validation checks against a payload.
pub fn validate_payload(payload: &SEOPayload) -> Vec<validation::SEOIssue> {
    validation::validate(payload)
}

/// Returns the SHA-256 hash of the payload.
///
/// # Errors
///
/// Returns [`EaseoError`] when the payload cannot be serialized.
pub fn hash_payload(payload: &SEOPayload) -> Result<String, error::EaseoError> {
    hashing::hash_payload(payload)
}

/// Returns a quoted HTTP ETag for the payload.
///
/// # Errors
///
/// Returns [`EaseoError`] when the payload cannot be serialized.
pub fn etag_payload(payload: &SEOPayload) -> Result<String, error::EaseoError> {
    hashing::etag_payload(payload)
}

pub use detrack::{clean_query, clean_url, CleanResult};
pub use url::{normalize_path, normalize_public_url};

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest;

    #[test]
    fn test_basic_payload() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Post,
            title: Some("Hello World".to_string()),
            excerpt: Some("A test post.".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/blog/hello", &config).unwrap();
        assert_eq!(payload.title, "Hello World");
        assert_eq!(payload.canonical, "https://example.com/blog/hello");
        assert_eq!(payload.og.og_type, "article");
    }

    #[test]
    fn test_url_normalization() {
        let policy = URLPolicy::default();
        let result = url::normalize_path("//blog///hello", &policy).unwrap();
        assert_eq!(result, "/blog/hello");
    }

    fn example_config() -> SEOConfig {
        SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_lowercase_paths_preserved_by_default() {
        let payload = build_seo_payload(
            &SEOEntity {
                entity_type: EntityType::Product,
                title: Some("Phone".to_string()),
                ..Default::default()
            },
            "/Products/iPhone",
            &example_config(),
        )
        .unwrap();
        assert_eq!(payload.canonical, "https://example.com/Products/iPhone");
    }

    #[test]
    fn test_lowercase_paths_opt_in() {
        let config = SEOConfig {
            url_policy: URLPolicy {
                lowercase_paths: true,
                ..Default::default()
            },
            ..example_config()
        };
        let payload = build_seo_payload(
            &SEOEntity {
                entity_type: EntityType::Product,
                title: Some("Phone".to_string()),
                ..Default::default()
            },
            "/Products/iPhone",
            &config,
        )
        .unwrap();
        assert_eq!(payload.canonical, "https://example.com/products/iphone");
    }

    #[test]
    fn test_query_bytes_preserved_and_fragment_dropped() {
        let result = normalize_public_url(
            "/page?foo&a=1&a=2&q=hello%20world&utm_source=x#frag",
            &example_config(),
        )
        .unwrap();
        assert_eq!(
            result,
            "https://example.com/page?foo&a=1&a=2&q=hello%20world"
        );
    }

    #[test]
    fn test_percent_encoded_tracking_param_stripped() {
        let result = normalize_public_url("/page?%75tm_source=x&q=1", &example_config()).unwrap();
        assert_eq!(result, "https://example.com/page?q=1");
    }

    #[test]
    fn test_extra_tracking_params() {
        let config = SEOConfig {
            url_policy: URLPolicy {
                extra_tracking_params: vec!["tag".to_string()],
                ..Default::default()
            },
            ..example_config()
        };
        let result = normalize_public_url("/products?tag=shoes&q=1", &config).unwrap();
        assert_eq!(result, "https://example.com/products?q=1");
    }

    #[test]
    fn test_app_params_survive_default_policy() {
        let result = normalize_public_url(
            "/search?keyword=rust&tag=shoes&ref=related&source=nav",
            &example_config(),
        )
        .unwrap();
        assert!(result.contains("keyword=rust"));
        assert!(result.contains("tag=shoes"));
        assert!(result.contains("ref=related"));
        assert!(result.contains("source=nav"));
    }

    #[test]
    fn test_allowlist_keeps_tracking_param() {
        let config = SEOConfig {
            url_policy: URLPolicy {
                allowed_query_params: vec!["utm_source".to_string()],
                ..Default::default()
            },
            ..example_config()
        };
        let result = normalize_public_url("/page?utm_source=x&q=1", &config).unwrap();
        assert_eq!(result, "https://example.com/page?utm_source=x");
    }

    #[test]
    fn test_non_published_status_noindex() {
        let draft = build_seo_payload(
            &SEOEntity {
                entity_type: EntityType::Page,
                title: Some("Draft".to_string()),
                status: Some("draft".to_string()),
                ..Default::default()
            },
            "/draft",
            &example_config(),
        )
        .unwrap();
        assert!(draft.robots.contains("noindex"));

        let unset = build_seo_payload(
            &SEOEntity {
                entity_type: EntityType::Page,
                title: Some("Page".to_string()),
                ..Default::default()
            },
            "/page",
            &example_config(),
        )
        .unwrap();
        assert_eq!(unset.robots, "index,follow");
    }

    #[test]
    fn test_canonical_path_override_is_normalized() {
        let overrides = SEOOverrides {
            canonical_path: Some("/Promo/".to_string()),
            ..Default::default()
        };
        let payload = build_seo_payload_with_overrides(
            &SEOEntity {
                entity_type: EntityType::Page,
                title: Some("Promo".to_string()),
                ..Default::default()
            },
            "/ignored",
            &example_config(),
            &overrides,
        )
        .unwrap();
        assert_eq!(payload.canonical, "https://example.com/Promo");
    }

    #[test]
    fn test_relative_canonical_url_rejected() {
        let overrides = SEOOverrides {
            canonical_url: Some("/relative".to_string()),
            ..Default::default()
        };
        assert!(build_seo_payload_with_overrides(
            &SEOEntity {
                entity_type: EntityType::Page,
                title: Some("Page".to_string()),
                ..Default::default()
            },
            "/page",
            &example_config(),
            &overrides,
        )
        .is_err());
    }

    #[test]
    fn test_video_schema_has_thumbnail_and_upload_date() {
        let entity = SEOEntity {
            entity_type: EntityType::Video,
            title: Some("Episode 1".to_string()),
            featured_image: Some(SEOImage {
                url: "https://example.com/thumb.jpg".to_string(),
                width: None,
                height: None,
                alt: None,
            }),
            published_at: Some("2026-01-15".to_string()),
            ..Default::default()
        };
        let payload = build_seo_payload(&entity, "/video/ep-1", &example_config()).unwrap();
        let schema = payload.schema_jsonld.unwrap();
        assert_eq!(schema["@type"], "VideoObject");
        assert_eq!(schema["thumbnailUrl"], "https://example.com/thumb.jpg");
        assert_eq!(schema["uploadDate"], "2026-01-15");
    }

    #[test]
    fn test_hash_deterministic() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("Test".to_string()),
            ..Default::default()
        };

        let payload1 = build_seo_payload(&entity, "/test", &config).unwrap();
        let payload2 = build_seo_payload(&entity, "/test", &config).unwrap();
        assert_eq!(
            hash_payload(&payload1).unwrap(),
            hash_payload(&payload2).unwrap()
        );
    }

    #[test]
    fn test_product_schema() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Product,
            title: Some("Widget".to_string()),
            sku: Some("W-001".to_string()),
            price: Some("29.99".to_string()),
            price_currency: Some("USD".to_string()),
            availability: Some("InStock".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/products/widget", &config).unwrap();
        assert_eq!(payload.og.og_type, "website");
        let schema = payload.schema_jsonld.unwrap();
        assert_eq!(schema["@type"], "Product");
        assert_eq!(schema["sku"], "W-001");
    }

    #[test]
    fn test_contract_generation() {
        let config = SEOContractConfig {
            canonical_host: "example.com".to_string(),
            scheme: "https".to_string(),
            ..Default::default()
        };

        let contract = build_seo_contract(&config).unwrap();
        assert_eq!(contract.contract_version, "1");
        assert_eq!(contract.site.canonical_host, "example.com");
    }

    #[test]
    fn test_detrack() {
        let result =
            detrack::clean_url("https://example.com/page?utm_source=twitter&q=hello&fbclid=123");
        assert_eq!(result.url, "https://example.com/page?q=hello");
        assert!(result.removed_params.contains_key("utm_source"));
        assert!(result.removed_params.contains_key("fbclid"));
    }

    #[test]
    fn test_robots_serialization() {
        let robots = Robots {
            index: true,
            follow: false,
            ..Default::default()
        };
        assert_eq!(robots.serialize(), "index,nofollow");
    }

    #[test]
    fn test_render_html() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("Test".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/test", &config).unwrap();
        let html = payload.render_html().unwrap();
        assert!(html.contains("<title>Test</title>"));
        assert!(html.contains("og:title"));
    }

    #[test]
    fn conformance_hash() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Post,
            title: Some("Hello World".to_string()),
            excerpt: Some("An example blog post.".to_string()),
            slug: Some("hello-world".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/blog/test-article", &config).unwrap();
        let json = serde_json::to_string(&payload.to_dict().unwrap()).unwrap();
        let hash = format!("{:x}", sha2::Sha256::digest(json.as_bytes()));
        println!("CONFORMANCE_HASH:{}", hash);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Text module tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_html_to_text_strips_script() {
        let result = crate::text::html_to_text("<p>Hello</p><script>alert(1)</script><p>World</p>");
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
        assert!(!result.contains("alert"));
    }

    #[test]
    fn test_html_to_text_strips_style() {
        let result =
            crate::text::html_to_text("<p>Hello</p><style>.red{color:red}</style><p>World</p>");
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
        assert!(!result.contains("color"));
    }

    #[test]
    fn test_html_to_text_strips_noscript() {
        let result =
            crate::text::html_to_text("<p>Hello</p><noscript>JS disabled</noscript><p>World</p>");
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
        assert!(!result.contains("JS disabled"));
    }

    #[test]
    fn test_html_to_text_collapses_whitespace() {
        let result = crate::text::html_to_text("Hello   \t\n  World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_description_snippet_none_input() {
        assert_eq!(crate::text::build_description_snippet(None, 160), None);
    }

    #[test]
    fn test_description_snippet_truncation() {
        let text = "A".repeat(200);
        let result = crate::text::build_description_snippet(Some(&text), 160).unwrap();
        assert!(result.ends_with("..."));
        assert!(result.len() <= 163);
    }
}
