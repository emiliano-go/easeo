---
title: "JavaScript API"
description: "The @easeo/core JavaScript and TypeScript API, serialization, and errors."
---

# JavaScript/TypeScript API Reference { #javascripttypescript-api-reference }

## Core Types

- `SEOConfig`: Site-wide configuration (`canonicalHost` required)
- `SEOEntity`: Content entity input
- `SEOOverrides`: Per-entity overrides (highest precedence)
- `SEOPayload`: Generated SEO output
- `OpenGraphPayload`: Open Graph data
- `TwitterPayload`: Twitter Card data
- `SEOContract`: SEO contract
- `SEOContractConfig`: Contract configuration
- `SEOIssue`: Validation issue

## Core Functions

- `buildSeoPayload(entity, route, config, overrides?)`: Build SEO payload
- `buildSeoPayloadWithOverrides(entity, route, config, overrides)`: Explicit alias
- `fromBlogPost`, `fromProduct`, `fromFaq`: entity factories
- `buildSeoContract(config)`: Build contract
- `validatePayload(payload)`: Validate payload, returns `SEOIssue[]`
- `normalizePath(path, options?)`: Normalize URL path
- `normalizePublicUrl(url, config)`: Build canonical URL
- `cleanUrl(url)`: Remove tracking params; returns `{ url, removedParams, cleanedParams }` (plain objects)
- `cleanQuery(query)`: Remove tracking params from a query string
- `getSchemaRegistry()`: Schema registry introspection (`has`, `listTypes`)

## Payload methods

`renderHtml()`, `renderOpengraph()`, `renderTwitter()`, `renderJsonld()`,
`toObject()`, `toJSON()`, `toDict()`, `toJSONString()`, `hash()`, `etag()`.

## Serialization

Payload data lives in enumerable camelCase properties, so `Object.keys`,
spread, and the TypeScript types all agree. Methods are non-enumerable.

```typescript
const payload = buildSeoPayload(entity, route, config);

Object.keys(payload);
// ["title", "description", "canonical", "robots", "openGraph", "twitter", "schemaJsonLd"]

{ ...payload };              // plain camelCase data
JSON.stringify(payload);     // → an object (not a double-encoded string)
payload.toObject();          // plain camelCase object
payload.toDict();            // canonical snake_case object (matches Python/Rust)
payload.toJSONString();      // canonical pretty-printed JSON string
payload.toString();          // same as toJSONString()
```

`toObject()` / `toJSON()` are what `JSON.stringify` uses, so `res.json(payload)`
works as expected. `toDict()` and `toJSONString()` return the canonical
snake_case wire format shared with the Python and Rust APIs and the published
JSON schemas.

## Errors

The JS error hierarchy mirrors Rust and Python, and maps core errors to typed
classes:

```typescript
import { EaseoError, ConfigurationError, EntityError } from "@easeo/core";

try {
  buildSeoPayload(entity, route, config);
} catch (err) {
  if (err instanceof ConfigurationError) {
    // err.code === "EASEO_CONFIGURATION"
  } else if (err instanceof EaseoError) {
    // base class
  }
}
```

| Class | `code` |
|-------|--------|
| `EaseoError` | `EASEO_ERROR` |
| `InvalidUrlError` | `EASEO_INVALID_URL` |
| `ConfigurationError` | `EASEO_CONFIGURATION` |
| `EntityError` | `EASEO_ENTITY` |
| `SchemaError` | `EASEO_SCHEMA` |
| `ContractError` | `EASEO_CONTRACT` |

Missing or wrong-typed arguments throw `TypeError` with a message naming the
function and parameter.

## Payload lookup and equality

```typescript
payload.get("title");               // field, or undefined
payload.get("title", "fallback");   // with default
payload.has("title");               // boolean
payload.equals(otherPayload);       // deep equality
```

## Factories

```typescript
import { fromBlogPost, fromProduct, fromFaq } from "@easeo/core";

fromBlogPost({ title, bodyHtml, slug, author, excerpt, breadcrumbs });
fromProduct({ name, sku, price, currency, availability, description });
fromFaq({ questions, title, description });
```

## Extension points

```typescript
import { HookRegistry, SchemaRegistry, buildSeoPayload } from "@easeo/core";

const hooks = new HookRegistry();
hooks.register("post_process", (payload, entity, config) => {
  payload.generator = "easeo";
  return payload;
});

const registry = new SchemaRegistry();
registry.register("Article", (entity, config, canonical, title) => ({
  "@context": "https://schema.org",
  "@type": "PodcastEpisode",
  name: title,
}));

const config = { canonicalHost: "example.com", publicBaseUrl: "https://example.com", hooks, schemaRegistry: registry };
const payload = buildSeoPayload(entity, "/x", config);
payload.get("generator");   // "easeo"
```

Both live on the config, so the build stays deterministic and scoped.

## Custom JSON-LD schemas

`SchemaRegistry.register()` is Rust-only and throws from JavaScript by design. Pass custom JSON-LD per page instead:

```typescript
import { buildSeoPayloadWithOverrides } from "@easeo/core";

const payload = buildSeoPayloadWithOverrides(entity, "/podcast/ep-1", config, {
  schemaJsonLd: { "@context": "https://schema.org", "@type": "Podcast", name: "My Podcast" },
});
```
