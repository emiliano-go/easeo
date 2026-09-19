---
title: "Framework Recipes"
description: "Copy-paste integration patterns for JavaScript and Python frameworks."
---

# Framework Recipes { #framework-recipes }

Short, copy-paste patterns for wiring easeo into a framework. For full details
per integration, see the [Integrations](../integrations/index.md) track.

## Next.js { #next }

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
    config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
  });
}
```

`easeoMetadata` returns a native Next.js `Metadata` object.

## Astro { #astro }

```js
// astro.config.mjs
import easeo from "@easeo/astro";

export default defineConfig({
  integrations: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
      contract: { canonicalHost: "example.com", scheme: "https" },
    }),
  ],
});
```

## Vite { #vite }

```js
// vite.config.mjs
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
    }),
  ],
});
```

## Nuxt { #nuxt }

```ts
import { useEaseoSeo } from "@easeo/nuxt";

useEaseoSeo({
  entity: { entityType: "post", title: article.title, description: article.description },
  route: `/blog/${article.slug}`,
  config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
});
```

## SvelteKit { #sveltekit }

```svelte
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";
  import { page } from "$app/stores";

  const seo = buildEaseoPayload(
    { entityType: "post", title: "Hello", description: "A post" },
    $page.url.pathname,
    { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
  );
</script>

<EaseoHead {seo} />
```

## React { #react }

```tsx
import { EaseoHead } from "@easeo/react";

<EaseoHead
  entity={{ entityType: "product", title: product.name }}
  route={`/products/${product.slug}`}
  config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
/>
```

The component renders nothing; it keeps `document.head` in sync on the client.
For SSR, put `payload.renderHtml()` into your template.

## FastAPI { #fastapi }

```python
from easeo.adapters.fastapi import EaseoSEO

seo = EaseoSEO(config)

@app.get("/products/{slug}")
def product(slug: str):
    return seo.for_entity(product, f"/products/{slug}")
```

## Flask { #flask }

```python
from easeo.adapters.flask import Easeo

easeo = Easeo(app, config)
```

Then in a template: `{% raw %}{{ easeo_head(entity, request.path) }}{% endraw %}`.

## Django { #django }

```python
# settings.py
EASEO = {
    "canonical_host": "example.com",
    "public_base_url": "https://example.com",
    "site_name": "Example",
}
```

```django
{% load easeo_tags %}
<head>
  {% easeo_head entity request.path %}
</head>
```

## Recap { #recap }

* Every JS integration supports default and named imports.
* Python adapters are lazy and installed as extras.
* Route and config are the two inputs every adapter needs.
