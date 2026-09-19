---
title: "React"
description: "Use the easeo React component to keep document.head in sync."
---

# React { #react }

`@easeo/react` provides `<EaseoHead />`, a component that keeps
`document.head` in sync with an easeo payload on the client.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* React 17 or newer.

## Install { #install }

```bash
npm install @easeo/react
```

## Quick start { #usage }

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

## Props { #props }

| Prop | Type | Description |
|---|---|---|
| `entity` | `SEOEntity` | Content entity |
| `route` | `string` | URL path |
| `config` | `SEOConfig` | Site-wide settings |

## Patterns { #patterns }

### Router-driven head { #router }

Mount one `<EaseoHead />` per route and pass the current route, so the head
updates on navigation. See the [React example](../examples/react.md#react-router).

### Server rendering { #ssr }

`<EaseoHead />` is SSR-safe: when `document` is unavailable, it returns `null`
and does nothing. For SSR or SSG, build the payload and put the rendered head
into your HTML template:

```tsx
const payload = buildSeoPayload(entity, route, config);

return (
  <html>
    <head dangerouslySetInnerHTML={{ __html: payload.renderHtml() }} />
    <body>{children}</body>
  </html>
);
```

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| No head updates on SSR | The component is a no-op without `document`; render the head on the server instead |
| Old tags remain after navigation | Mount the component inside each route so it remounts |
| Two `<title>` tags | Another library also manages the document title |

## Notes { #notes }

* The component returns `null`; it is not a visual element.
* `renderHtml()` output is already escaped, so it is safe to inject.
* Both default and named imports are supported.

## Related { #related }

* [React example](../examples/react.md)
* [JavaScript API reference](../reference/javascript-api.md)
