# JavaScript/TypeScript API Reference

## Core Types

- `SEOConfig` — Site-wide configuration
- `SEOEntity` — Content entity input
- `SEOPayload` — Generated SEO output
- `OpenGraphPayload` — Open Graph data
- `TwitterPayload` — Twitter Card data
- `SEOContract` — SEO contract
- `SEOContractConfig` — Contract configuration
- `SEOIssue` — Validation issue

## Core Functions

- `buildSeoPayload(entity, route, config)` — Build SEO payload
- `buildSeoContract(config)` — Build contract
- `validatePayload(payload)` — Validate payload
- `normalizePath(path, options?)` — Normalize URL path
- `normalizePublicUrl(url, config)` — Build canonical URL
- `cleanUrl(url)` — Remove tracking params from URL
- `cleanQuery(query)` — Remove tracking params from query
