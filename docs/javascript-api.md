# JavaScript/TypeScript API Reference

## Core Types

- `SEOConfig` — Site-wide configuration (`canonicalHost` required)
- `SEOEntity` — Content entity input
- `SEOOverrides` — Per-entity overrides (highest precedence)
- `SEOPayload` — Generated SEO output
- `OpenGraphPayload` — Open Graph data
- `TwitterPayload` — Twitter Card data
- `SEOContract` — SEO contract
- `SEOContractConfig` — Contract configuration
- `SEOIssue` — Validation issue

## Core Functions

- `buildSeoPayload(entity, route, config)` — Build SEO payload
- `buildSeoPayloadWithOverrides(entity, route, config, overrides)` — Build with per-page overrides
- `buildSeoContract(config)` — Build contract
- `validatePayload(payload)` — Validate payload, returns `SEOIssue[]`
- `normalizePath(path, options?)` — Normalize URL path
- `normalizePublicUrl(url, config)` — Build canonical URL
- `cleanUrl(url)` — Remove tracking params; returns `{ url, removedParams, cleanedParams }` (plain objects)
- `cleanQuery(query)` — Remove tracking params from a query string
- `getSchemaRegistry()` — Schema registry introspection (`has`, `listTypes`)

## Payload methods

`renderHtml()`, `renderOpengraph()`, `renderTwitter()`, `renderJsonld()`, `toObject()`, `toJSON()`, `hash()`, `etag()`.

## Custom JSON-LD schemas

`SchemaRegistry.register()` is Rust-only and throws from JavaScript by design. Pass custom JSON-LD per page instead:

```typescript
import { buildSeoPayloadWithOverrides } from "@easeo/core";

const payload = buildSeoPayloadWithOverrides(entity, "/podcast/ep-1", config, {
  schemaJsonLd: { "@context": "https://schema.org", "@type": "Podcast", name: "My Podcast" },
});
```
