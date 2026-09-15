// Cross-language conformance tests
// These tests verify that fixture inputs produce expected outputs

#[cfg(test)]
mod conformance {
    use std::fs;
    use easeo_core::*;

    #[test]
    fn article_basic_fixture() {
        let fixture = fs::read_to_string("fixtures/payloads/article-basic.json")
            .expect("fixture not found");
        let v: serde_json::Value = serde_json::from_str(&fixture).unwrap();

        let config = SEOConfig {
            canonical_host: v["input"]["config"]["canonical_host"].as_str().unwrap().to_string(),
            public_base_url: v["input"]["config"]["public_base_url"].as_str().unwrap().to_string(),
            ..Default::default()
        };
        let entity = SEOEntity {
            entity_type: EntityType::from_str(v["input"]["entity"]["entity_type"].as_str().unwrap()).unwrap(),
            title: v["input"]["entity"]["title"].as_str().map(|s| s.to_string()),
            excerpt: v["input"]["entity"]["excerpt"].as_str().map(|s| s.to_string()),
            ..Default::default()
        };
        let payload = build_seo_payload(&entity, v["input"]["route"].as_str().unwrap(), &config).unwrap();

        assert_eq!(payload.title, v["expected"]["title"].as_str().unwrap());
        assert_eq!(payload.description, v["expected"]["description"].as_str().unwrap());
        assert_eq!(payload.canonical, v["expected"]["canonical"].as_str().unwrap());
        assert_eq!(payload.robots, v["expected"]["robots"].as_str().unwrap());
    }

    #[test]
    fn contract_fixture() {
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
}
