# Framework Integrations

## JavaScript/TypeScript

### Next.js

```bash
npm install @easeo/next
```

```typescript
import { easeoMetadata } from "@easeo/next";

export async function generateMetadata({ params }) {
  return easeoMetadata({
    entity: await getProduct(params.slug),
    route: `/products/${params.slug}`,
    config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
  });
}
```

Returns a native Next.js `Metadata` object. `config` is required (the core rejects an empty `canonicalHost`).

### Vite

```bash
npm install @easeo/vite
```

```typescript
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
    }),
  ],
});
```

Uses Vite's `transformIndexHtml` hook.

### Astro

```bash
npm install @easeo/astro
```

```typescript
import easeo from "@easeo/astro";

export default defineConfig({
  integrations: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
      contract: { canonicalHost: "example.com" }, // optional: emit .easeo/contract.json
    }),
  ],
});
```

### Nuxt

```bash
npm install @easeo/nuxt
```

```typescript
import { useEaseoSeo } from "@easeo/nuxt";

useEaseoSeo({
  entity: { entityType: "post", title: article.title, description: article.description },
  route: `/blog/${article.slug}`,
  config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
});
```

Pushes tags via Nuxt's `useHead()` when available and always returns the built payload.

### SvelteKit

```bash
npm install @easeo/sveltekit
```

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

### React

```bash
npm install @easeo/react
```

```tsx
import { EaseoHead } from "@easeo/react";

<EaseoHead
  entity={{ entityType: "product", title: product.name, description: product.description }}
  route={`/products/${product.slug}`}
  config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
/>
```

SSR-safe (renders nothing server-side; use `payload.renderHtml()` in your HTML template for SSR/SSG).

## Python

### FastAPI

```bash
pip install easeo[fastapi]
```

```python
from easeo.adapters.fastapi import EaseoSEO

seo = EaseoSEO(config)

@app.get("/products/{slug}")
def product(slug: str):
    return templates.TemplateResponse("product.html", {"seo": seo.for_entity(product, f"/products/{slug}")})
```

### Flask

```bash
pip install easeo[flask]
```

```python
from easeo.adapters.flask import Easeo

easeo = Easeo(app, config)
```

### Django

```bash
pip install easeo[django]
```

```python
from easeo.adapters.django import seo_head
```

### Zensical

```bash
pip install easeo[zensical]
```

```toml
[project.markdown_extensions]
"easeo.contrib.zensical" = {
  canonical_host = "example.com",
  public_base_url = "https://example.com/",
  site_name = "Example",
  title_template = "{title} - Example",
  auto_generate_schema = true
}
```
