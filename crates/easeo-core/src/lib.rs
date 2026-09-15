pub mod config;
pub mod entity;
pub mod payload;
pub mod url;
pub mod canonical;
pub mod opengraph;
pub mod twitter;
pub mod robots;
pub mod breadcrumbs;
pub mod jsonld;
pub mod contract;
pub mod validation;
pub mod hashing;
pub mod detrack;
pub mod text;
pub mod error;

pub use config::{SEOConfig, URLPolicy, TrailingSlash};
pub use entity::{SEOEntity, EntityType, SEOAuthor, SEOImage, Breadcrumb, FAQItem, Robots, SEOOverrides};
pub use payload::{SEOPayload, OGPayload, TwitterPayload};
pub use contract::{SEOContract, SEOContractConfig, SEOContractRule, SEOExpectation, ContractSeverity, SchemaExpectation, FieldExpectation, ContractGenerator, ContractSite};
pub use jsonld::registry;
pub use error::EaseoError;

use entity as entity_mod;

pub fn build_seo_payload(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
) -> Result<SEOPayload, EaseoError> {
    payload::build_seo_payload(entity, route, config, None, None)
}

pub fn build_seo_payload_with_overrides(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: &entity_mod::SEOOverrides,
) -> Result<SEOPayload, EaseoError> {
    payload::build_seo_payload(entity, route, config, Some(overrides), None)
}

pub fn build_seo_contract(
    config: &contract::SEOContractConfig,
) -> Result<contract::SEOContract, EaseoError> {
    contract::build_contract(config)
}

pub fn validate_payload(payload: &SEOPayload) -> Vec<validation::SEOIssue> {
    validation::validate(payload)
}

pub fn hash_payload(payload: &SEOPayload) -> String {
    hashing::hash_payload(payload)
}

pub fn etag_payload(payload: &SEOPayload) -> String {
    hashing::etag_payload(payload)
}

pub use url::{normalize_path, normalize_public_url};
pub use detrack::{clean_url, clean_query, CleanResult};

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
        assert_eq!(hash_payload(&payload1), hash_payload(&payload2));
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
        let result = detrack::clean_url("https://example.com/page?utm_source=twitter&q=hello&fbclid=123");
        assert_eq!(result.url, "https://example.com/page?q=hello");
        assert!(result.removed_params.contains_key("utm_source"));
        assert!(result.removed_params.contains_key("fbclid"));
    }

    #[test]
    fn test_robots_serialization() {
        let robots = Robots { index: true, follow: false, ..Default::default() };
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
        let html = payload.render_html();
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
        let json = serde_json::to_string(&payload.to_dict()).unwrap();
        let hash = format!("{:x}", sha2::Sha256::digest(json.as_bytes()));
        println!("CONFORMANCE_HASH:{}", hash);
    }
}
