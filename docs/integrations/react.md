---
title: "React"
---

# React { #react }

`@easeo/react` provides `<EaseoHead />`, a component that keeps
`document.head` in sync with an easeo payload on the client.

## Install { #install }

```bash
npm install @easeo/react
```

## Usage { #usage }

```tsx
import { EaseoHead } from "@easeo/react";

<EaseoHead
  entity={{ entityType: "product", title: product.name, description: product.description }}
  route={`/products/${product.slug}`}
  config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
/>
```

The component renders nothing. It builds the payload on render and updates the
document head.

## Server rendering { #ssr }

`<EaseoHead />` is SSR-safe: when `document` is unavailable, it returns `null`
and does nothing. For SSR or SSG, put the rendered head into your HTML
template:

```tsx
const payload = buildSeoPayload(entity, route, config);

return (
  <html>
    <head dangerouslySetInnerHTML={{ __html: payload.renderHtml() }} />
    <body>{children}</body>
  </html>
);
```

## Notes { #notes }

* The component returns `null`; it is not a visual element.
* `renderHtml()` output is already escaped, so it is safe to inject.
* Both default and named imports are supported.
