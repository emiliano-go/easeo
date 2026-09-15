// Conformance test: reads fixture and verifies output
#[cfg(test)]
mod conformance_tests {
    use easeo_core::*;

    #[test]
    fn test_article_basic_conformance() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Post,
            title: Some("Hello World".to_string()),
            excerpt: Some("An example blog post.".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/blog/hello-world", &config).unwrap();

        // Verify against expected values from fixture
        assert_eq!(payload.title, "Hello World");
        assert_eq!(payload.description, "An example blog post.");
        assert_eq!(payload.canonical, "https://example.com/blog/hello-world");
        assert_eq!(payload.robots, "index,follow");
        assert_eq!(payload.og.og_type, "article");
        assert_eq!(payload.og.title, Some("Hello World".to_string()));
        assert_eq!(payload.og.description, Some("An example blog post.".to_string()));
        assert_eq!(payload.og.url, Some("https://example.com/blog/hello-world".to_string()));
        assert_eq!(payload.twitter.card, "summary_large_image");
        assert_eq!(payload.twitter.title, Some("Hello World".to_string()));
        assert_eq!(payload.twitter.description, Some("An example blog post.".to_string()));
    }

    #[test]
    fn test_product_basic_conformance() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Product,
            title: Some("Widget".to_string()),
            excerpt: Some("A useful widget.".to_string()),
            sku: Some("WIDGET-001".to_string()),
            price: Some("29.99".to_string()),
            price_currency: Some("USD".to_string()),
            availability: Some("InStock".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/products/widget", &config).unwrap();

        assert_eq!(payload.title, "Widget");
        assert_eq!(payload.og.og_type, "website");
        assert!(payload.schema_jsonld.is_some());
        let schema = payload.schema_jsonld.unwrap();
        assert_eq!(schema["@type"], "Product");
        assert_eq!(schema["sku"], "WIDGET-001");
    }

    #[test]
    fn test_hash_determinism_across_calls() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("Deterministic".to_string()),
            ..Default::default()
        };

        let hash1 = hash_payload(&build_seo_payload(&entity, "/test", &config).unwrap());
        let hash2 = hash_payload(&build_seo_payload(&entity, "/test", &config).unwrap());
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_contract_conformance() {
        let config = SEOContractConfig {
            canonical_host: "example.com".to_string(),
            scheme: "https".to_string(),
            ..Default::default()
        };

        let contract = build_seo_contract(&config).unwrap();
        assert_eq!(contract.contract_version, "1");
        assert_eq!(contract.site.canonical_host, "example.com");
        assert_eq!(contract.site.scheme, "https");
        assert_eq!(contract.generator.name, "easeo");
    }

    #[test]
    fn test_render_html_conformance() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("Test Page".to_string()),
            excerpt: Some("A test page description.".to_string()),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/test", &config).unwrap();
        let html = payload.render_html();

        assert!(html.contains("<title>Test Page</title>"));
        assert!(html.contains("<meta name=\"description\" content=\"A test page description.\">"));
        assert!(html.contains("<link rel=\"canonical\" href=\"https://example.com/test\">"));
        assert!(html.contains("<meta name=\"robots\" content=\"index,follow\">"));
        assert!(html.contains("og:title"));
        assert!(html.contains("twitter:card"));
    }
}
