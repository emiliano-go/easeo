// easeo conformance tests - loads fixtures and verifies output
#[cfg(test)]
mod fixture_tests {
    use std::fs;
    use easeo_core::*;

    #[test]
    fn test_article_basic_from_fixture() {
        let fixture = fs::read_to_string("fixtures/payloads/article-basic.json")
            .expect("Failed to read fixture");
        let fixture: serde_json::Value = serde_json::from_str(&fixture).unwrap();

        let input = &fixture["input"];
        let expected = &fixture["expected"];

        // Build config from fixture
        let config = SEOConfig {
            canonical_host: input["config"]["canonical_host"].as_str().unwrap().to_string(),
            public_base_url: input["config"]["public_base_url"].as_str().unwrap().to_string(),
            ..Default::default()
        };

        // Build entity from fixture
        let entity = SEOEntity {
            entity_type: EntityType::from_str(input["entity"]["entity_type"].as_str().unwrap()).unwrap(),
            title: input["entity"]["title"].as_str().map(|s| s.to_string()),
            excerpt: input["entity"]["excerpt"].as_str().map(|s| s.to_string()),
            ..Default::default()
        };

        let route = input["route"].as_str().unwrap();
        let payload = build_seo_payload(&entity, route, &config).unwrap();

        // Verify against expected
        assert_eq!(payload.title, expected["title"].as_str().unwrap());
        assert_eq!(payload.description, expected["description"].as_str().unwrap());
        assert_eq!(payload.canonical, expected["canonical"].as_str().unwrap());
        assert_eq!(payload.robots, expected["robots"].as_str().unwrap());
    }

    #[test]
    fn test_contract_from_fixture() {
        let config = SEOContractConfig {
            canonical_host: "example.com".to_string(),
            scheme: "https".to_string(),
            ..Default::default()
        };

        let contract = build_seo_contract(&config).unwrap();
        assert_eq!(contract.contract_version, "1");
        assert_eq!(contract.site.canonical_host, "example.com");
        assert_eq!(contract.site.scheme, "https");
    }

    #[test]
    fn test_render_html_produces_valid_output() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Post,
            title: Some("Test".to_string()),
            excerpt: Some("Description.".to_string()),
            breadcrumbs: Some(vec![
                Breadcrumb { name: "Home".to_string(), url: "/".to_string() },
                Breadcrumb { name: "Blog".to_string(), url: "/blog".to_string() },
            ]),
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/blog/test", &config).unwrap();
        let html = payload.render_html();

        // Verify essential elements
        assert!(html.contains("<title>Test</title>"));
        assert!(html.contains("og:title"));
        assert!(html.contains("twitter:card"));
        assert!(html.contains("BreadcrumbList"));
    }

    #[test]
    fn test_hash_is_deterministic() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("Stable".to_string()),
            ..Default::default()
        };

        let h1 = hash_payload(&build_seo_payload(&entity, "/test", &config).unwrap());
        let h2 = hash_payload(&build_seo_payload(&entity, "/test", &config).unwrap());
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // SHA-256 hex is 64 chars
    }

    #[test]
    fn test_etag_format() {
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
        let etag = etag_payload(&payload);
        assert!(etag.starts_with('"'));
        assert!(etag.ends_with('"'));
    }

    #[test]
    fn test_validation_produces_issues() {
        let config = SEOConfig {
            canonical_host: "example.com".to_string(),
            public_base_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let entity = SEOEntity {
            entity_type: EntityType::Page,
            title: Some("A".repeat(70)), // Too long
            ..Default::default()
        };

        let payload = build_seo_payload(&entity, "/test", &config).unwrap();
        let issues = validate_payload(&payload);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| i.rule_id == "EASEO102"));
    }

    #[test]
    fn test_detrack_removes_utm() {
        let result = clean_url("https://example.com/page?utm_source=twitter&q=hello&fbclid=123");
        assert_eq!(result.url, "https://example.com/page?q=hello");
        assert!(result.removed_params.contains_key("utm_source"));
        assert!(result.removed_params.contains_key("fbclid"));
        assert!(!result.cleaned_params.contains_key("utm_source"));
    }
}
