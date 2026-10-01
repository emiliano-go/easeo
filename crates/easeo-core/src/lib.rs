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
