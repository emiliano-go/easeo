---
title: "Next.js"
description: "Next.js examples for easeo: static metadata, dynamic routes, and a shared config module."
---

# Next.js { #nextjs }

`@easeo/next` returns a native Next.js `Metadata` object from `generateMetadata`.

## Simple: a static page { #next-simple }

```tsx
// app/about/page.tsx
import { easeoMetadata } from "@easeo/next";
import type { Metadata } from "next";

export function generateMetadata(): Metadata {
  return easeoMetadata({
    entity: { entityType: "page", title: "About", description: "About us." },
    route: "/about",
    config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
  });
}

export default function About() {
  return <h1>About</h1>;
}
```

## Complex: a dynamic product route { #next-complex }

A shared config module keeps the site settings in one place. The page maps a
product record to an entity, then lets overrides win for the social card.

```ts
// lib/seo.ts
import type { SEOConfig } from "@easeo/core";

export const config: SEOConfig = {
  canonicalHost: "shop.example.com",
  publicBaseUrl: "https://shop.example.com",
  siteName: "Example Shop",
  titleTemplate: "{title} - Example Shop",
  defaultOgImage: "https://shop.example.com/assets/og-image.png",
  publisherName: "Example Shop",
  locale: "en_US",
  twitterSite: "@exampleshop",
  searchUrlTemplate: "https://shop.example.com/search?q={search_term_string}",
};
```

```tsx
// app/products/[slug]/page.tsx
import type { Metadata } from "next";
import { easeoMetadata } from "@easeo/next";
import { config } from "@/lib/seo";
import { getProduct } from "@/lib/products";

export async function generateMetadata(
  { params }: { params: { slug: string } }
): Promise<Metadata> {
  const product = await getProduct(params.slug);

  return easeoMetadata({
    entity: {
      entityType: "product",
      title: product.name,
      description: product.shortDescription,
      image: product.imageUrl,
      imageWidth: 1200,
      imageHeight: 630,
      imageAlt: product.name,
      sku: product.sku,
      price: product.price,
      priceCurrency: product.currency,
      availability: product.inStock ? "InStock" : "OutOfStock",
      updatedAt: product.updatedAt,
    },
    route: `/products/${product.slug}`,
    config,
  });
}
```

## Complex: reuse the payload for a route handler { #next-payload }

`generateMetadata` covers the head, but sometimes you also need the JSON-LD or
the hash for caching. Build the payload directly in a route handler:

```ts
// app/api/seo/products/[slug]/route.ts
import { NextResponse } from "next/server";
import { buildSeoPayload } from "@easeo/core";
import { config } from "@/lib/seo";
import { getProduct } from "@/lib/products";

export async function GET(_request: Request, { params }: { params: { slug: string } }) {
  const product = await getProduct(params.slug);

  const payload = buildSeoPayload(
    { entityType: "product", title: product.name, description: product.shortDescription },
    `/products/${product.slug}`,
    config
  );

  return NextResponse.json(
    { html: payload.renderHtml(), jsonLd: payload.schemaJsonLd },
    { headers: { ETag: payload.etag(), "Cache-Control": "public, max-age=300" } }
  );
}
```

## Notes { #next-notes }

* `config` is required; the core rejects an empty `canonicalHost`.
* Images are only set when the payload has one; otherwise Next.js omits the key.
* The same `SEOConfig` object works for both `easeoMetadata` and
  `buildSeoPayload`.
* See the [Next.js integration page](../integrations/next.md) for the returned
  `Metadata` mapping.
