---
title: "JavaScript"
description: "JavaScript and TypeScript examples for easeo, from a minimal payload to hooks, registries, and type-safe usage."
---

# JavaScript { #javascript }

## Simple: a minimal payload { #js-simple }

```js
const { buildSeoPayload } = require("@easeo/core");

const config = {
  canonicalHost: "example.com",
  publicBaseUrl: "https://example.com",
};

const payload = buildSeoPayload(
  { entityType: "post", title: "Hello World", description: "An example post." },
  "/blog/hello",
  config
);

console.log(payload.renderHtml());
```

## Simple: read and serialize { #js-serialize }

```js
payload.title;             // resolved title
payload.openGraph.type;    // "article"
payload.schemaJsonLd;      // JSON-LD object

payload.toObject();        // camelCase object (used by JSON.stringify)
payload.toDict();          // canonical snake_case object
payload.toJSONString();    // canonical pretty JSON string

JSON.stringify(payload);   // an object, not a double-encoded string
```

## Simple: overrides { #js-overrides }

```js
const payload = buildSeoPayload(
  entity,
  "/blog/hello",
  config,
  { metaTitle: "A one-off title", skipTitleTemplate: true, twitterCreator: "@easeo" }
);
```

## Simple: factories { #js-factories }

```js
const { fromBlogPost, fromProduct, fromFaq } = require("@easeo/core");

const post = fromBlogPost({ title: "Hello", bodyHtml: "<p>Body</p>", author: "Jane" });
const product = fromProduct({ name: "Widget", sku: "W-1", price: 9.99 });
const faq = fromFaq({ questions: [{ question: "Q?", answer: "A." }] });
```

## Complex: a full config module { #js-config }

```js
// seo.config.mjs
export const config = {
  canonicalHost: "shop.example.com",
  publicBaseUrl: "https://shop.example.com",
  siteName: "Example Shop",
  titleTemplate: "{title} - Example Shop",
  trailingSlash: "never",
  allowedQueryParams: ["page", "q"],
  defaultOgImage: "https://shop.example.com/assets/og-image.png",
  publisherName: "Example Shop",
  locale: "en_US",
  localeAlternate: ["es_UY", "pt_BR"],
  twitterSite: "@exampleshop",
  searchUrlTemplate: "https://shop.example.com/search?q={search_term_string}",
};
```

## Complex: caching headers from the payload { #js-caching }

```js
const payload = buildSeoPayload(entity, route, config);

return new Response(html, {
  headers: {
    ETag: payload.etag(),
    "Cache-Control": "public, max-age=300",
  },
});
```

## Complex: hooks for a site-wide schema { #js-hooks }

```js
const { HookRegistry, buildSeoPayload } = require("@easeo/core");

const hooks = new HookRegistry();

hooks.register("post_process", (payload, entity, config) => {
  const org = {
    "@context": "https://schema.org",
    "@type": "Organization",
    name: config.publisherName ?? "Example",
    url: config.publicBaseUrl,
  };
  const existing = payload.schema_jsonld;
  if (Array.isArray(existing)) {
    payload.schema_jsonld = [org, ...existing];
  } else if (existing) {
    payload.schema_jsonld = [org, existing];
  } else {
    payload.schema_jsonld = org;
  }
  return payload;
});

const configWithHooks = { ...config, hooks };
```

## Complex: a registered schema generator { #js-registry }

```js
const { SchemaRegistry } = require("@easeo/core");

const registry = new SchemaRegistry();

registry.register("Article", (entity, config, canonical, title, description, ogImage) => ({
  "@context": "https://schema.org",
  "@type": "PodcastEpisode",
  name: title,
  url: canonical,
  description,
}));

const configWithRegistry = { ...config, schemaRegistry: registry };
```

## Complex: TypeScript { #js-typescript }

```ts
import {
  buildSeoPayload,
  HookRegistry,
  SchemaRegistry,
  fromProduct,
  type SEOPayload,
  type SEOConfig,
} from "@easeo/core";

const config: SEOConfig = {
  canonicalHost: "shop.example.com",
  publicBaseUrl: "https://shop.example.com",
  siteName: "Example Shop",
};

const payload: SEOPayload = buildSeoPayload(
  fromProduct({ name: "Widget", sku: "W-1", price: 9.99 }),
  "/products/widget",
  config
);

const data = payload.toObject();
console.log(data.openGraph.title, payload.hash());
```

## Complex: validation warnings { #js-warnings }

Set `emitWarnings` to have the build log issues through `console.warn`:

```js
const payload = buildSeoPayload(entity, route, {
  ...config,
  emitWarnings: true,
});
// [EASEO108] OG image is missing; set default_og_image or an entity image (https://...)
```

## Complex: determinism check in tests { #js-tests }

```js
const assert = require("node:assert");

const a = buildSeoPayload(entity, "/blog/hello", config);
const b = buildSeoPayload(entity, "/blog/hello", config);

assert(a.equals(b));
assert.equal(a.hash(), b.hash());
```

## Related { #related }

* [JavaScript API reference](../reference/javascript-api.md)
* [Fallback chains](../concepts/fallback-chains.md)
* [Recipes](../recipes/index.md)
