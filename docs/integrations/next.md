---
title: "Next.js"
description: "Turn an easeo payload into a native Next.js Metadata object."
---

# Next.js { #nextjs }

`@easeo/next` converts an easeo payload into a native Next.js `Metadata`
object. Use it inside `generateMetadata`; there is no HTML manipulation.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* Next.js 13 or newer (App Router).
* A site-wide [`SEOConfig`](../tutorial/configuration.md).

## Install { #install }

```bash
npm install @easeo/next
```

## Quick start { #usage }

```tsx
// app/products/[slug]/page.tsx
import { easeoMetadata } from "@easeo/next";

export async function generateMetadata({ params }) {
  const product = await getProduct(params.slug);

  return easeoMetadata({
    entity: {
      entityType: "product",
      title: product.name,
      description: product.description,
    },
    route: `/products/${product.slug}`,
    config: {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
    },
  });
}
```

## Function signature { #signature }

```ts
easeoMetadata(input: { entity: SEOEntity; route: string; config: SEOConfig }): Metadata
```

`config` is required; the core rejects an empty `canonicalHost`.

## What it returns { #returns }

| Next.js key | Source |
|---|---|
| `title` | `payload.title` |
| `description` | `payload.description` |
| `alternates.canonical` | `payload.canonical` |
| `robots` | `payload.robots` |
| `openGraph` | title, description, url, siteName, images, locale, type |
| `twitter` | card, title, description, images, site, creator |

Images are only set when the payload has one; otherwise the `images` key is
`undefined` and Next.js omits it.

## Patterns { #patterns }

### Share the config { #share-config }

Keep the config in one module so every route uses the same settings:

```ts
// lib/seo.ts
import type { SEOConfig } from "@easeo/core";

export const config: SEOConfig = {
  canonicalHost: "shop.example.com",
  publicBaseUrl: "https://shop.example.com",
  siteName: "Example Shop",
  titleTemplate: "{title} - Example Shop",
  defaultOgImage: "https://shop.example.com/assets/og-image.png",
};
```

### Use the payload beyond the head { #payload }

`generateMetadata` covers the head. When you also need the JSON-LD or the hash
for caching, build the payload directly:

```ts
import { buildSeoPayload } from "@easeo/core";

const payload = buildSeoPayload(entity, route, config);
payload.renderHtml();
payload.schemaJsonLd;
payload.etag();
```

### Static metadata { #static }

For pages that never change, return the object synchronously:

```tsx
export function generateMetadata() {
  return easeoMetadata({ entity: { entityType: "page", title: "About" }, route: "/about", config });
}
```

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `canonicalHost` required error | `config` is missing or `canonicalHost` is empty |
| Social preview has no image | No `defaultOgImage` and the entity has no image; see [EASEO108](../concepts/validation.md) |
| Title has the site suffix twice | `titleTemplate` applied in the app and in easeo |

## Related { #related }

* [Next.js example](../examples/next.md)
* [JavaScript API reference](../reference/javascript-api.md)
* [Framework Recipes](../guides/framework-recipes.md)
