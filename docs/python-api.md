# Python API Reference

## Core Types

- `SEOConfig` — Site-wide configuration
- `SEOEntity` — Content entity input
- `SEOOverrides` — Per-entity overrides
- `SEOPayload` — Generated SEO output
- `URLPolicy` — URL normalization policy
- `Robots` — Robots directive
- `SEOImage` — Structured image
- `Breadcrumb` — Breadcrumb item
- `FAQItem` — FAQ question/answer
- `SEOContract` — SEO contract
- `SEOContractConfig` — Contract configuration
- `SEOIssue` — Validation issue

## Core Functions

- `build_seo_payload(entity, route, config, overrides=None)` — Build SEO payload
- `build_seo_payload_dict(...)` — Build as dict
- `build_seo_contract(config)` — Build contract
- `validate_payload(payload)` — Validate payload
- `normalize_path(path, policy)` — Normalize URL path
- `normalize_public_url(url, config)` — Build canonical URL
- `clean_url(url)` — Remove tracking params from URL
- `clean_query(query)` — Remove tracking params from query
