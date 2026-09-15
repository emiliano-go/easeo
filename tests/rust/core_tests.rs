// easeo-core Rust tests
#[cfg(test)]
mod tests {
    use easeo_core::*;

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
        let result = normalize_path("//blog///hello", &policy).unwrap();
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
}
