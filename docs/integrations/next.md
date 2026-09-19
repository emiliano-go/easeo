---
title: "Next.js"
description: "Turn an easeo payload into a native Next.js Metadata object."
---

# Next.js { #nextjs }

`@easeo/next` converts an easeo payload into a native Next.js `Metadata`
object. Use it inside `generateMetadata`; there is no HTML manipulation.

## Install { #install }

```bash
npm install @easeo/next
```

## Usage { #usage }

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

## What it returns { #returns }

| Next.js key | Source |
|---|---|
| `title` | `payload.title` |
| `description` | `payload.description` |
| `alternates.canonical` | `payload.canonical` |
| `robots` | `payload.robots` |
| `openGraph` | title, description, url, siteName, images, locale, type |
| `twitter` | card, title, description, images, site, creator |

`config` is required: the core rejects an empty `canonicalHost`.

## Notes { #notes }

* Images are only set when the payload has one; otherwise the `images` key is
  `undefined` and Next.js omits it.
* Both `import easeoMetadata from ...` and
  `import { easeoMetadata } from ...` work.
