---
title: "React"
description: "React examples for easeo: client-side head sync and server rendering."
---

# React { #react }

`@easeo/react` provides `<EaseoHead />`, which keeps `document.head` in sync on
the client.

## Simple: a component { #react-simple }

```tsx
import { EaseoHead } from "@easeo/react";

export function About() {
  return (
    <>
      <EaseoHead
        entity={{ entityType: "page", title: "About", description: "About us." }}
        route="/about"
        config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
      />
      <h1>About</h1>
    </>
  );
}
```

## Complex: a router-driven app { #react-router }

`<EaseoHead />` updates whenever its props change, so mount it once near the
router outlet and pass the current route.

```tsx
import { BrowserRouter, Routes, Route, useLocation } from "react-router-dom";
import { EaseoHead } from "@easeo/react";
import { config } from "./seo";

function ProductPage({ product }: { product: Product }) {
  const route = `/products/${product.slug}`;

  return (
    <>
      <EaseoHead
        entity={{
          entityType: "product",
          title: product.name,
          description: product.shortDescription,
          image: product.imageUrl,
          sku: product.sku,
          price: product.price,
          priceCurrency: product.currency,
          availability: product.inStock ? "InStock" : "OutOfStock",
        }}
        route={route}
        config={config}
      />
      <ProductDetail product={product} />
    </>
  );
}
```

```ts
// src/seo.ts
import type { SEOConfig } from "@easeo/core";

export const config: SEOConfig = {
  canonicalHost: "shop.example.com",
  publicBaseUrl: "https://shop.example.com",
  siteName: "Example Shop",
  titleTemplate: "{title} - Example Shop",
  defaultOgImage: "https://shop.example.com/assets/og-image.png",
};
```

## Complex: server rendering and static generation { #react-ssr }

`<EaseoHead />` is SSR-safe: it renders nothing when `document` is absent. For
SSR or SSG, build the payload and inject the head into your HTML template:

```tsx
import { buildSeoPayload } from "@easeo/core";
import { config } from "./seo";
import { renderToString } from "react-dom/server";

export function renderPage(product: Product) {
  const payload = buildSeoPayload(
    { entityType: "product", title: product.name, description: product.shortDescription },
    `/products/${product.slug}`,
    config
  );

  const body = renderToString(<ProductDetail product={product} />);

  return `<!doctype html>
<html>
  <head>${payload.renderHtml()}</head>
  <body>${body}</body>
</html>`;
}
```

## Notes { #react-notes }

* The component renders `null`; it is not a visual element.
* `renderHtml()` output is already escaped, so it is safe to inject.
* Mount one `<EaseoHead />` per route so the head always matches the page.
