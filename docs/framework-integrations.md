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
  });
}
```

### Vite

```bash
npm install @easeo/vite
```

```typescript
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [easeo({ config: { publicBaseUrl: "https://example.com" } })],
});
```

### Astro

```bash
npm install @easeo/astro
```

```typescript
import easeo from "@easeo/astro";

export default defineConfig({
  integrations: [easeo({ config: { siteName: "My Site", publicBaseUrl: "https://example.com" } })],
});
```

## Python

### FastAPI

```python
from easeo.adapters.fastapi import EaseoSEO

seo = EaseoSEO(config)

@app.get("/products/{slug}")
def product(slug: str):
    return templates.TemplateResponse("product.html", {"seo": seo.for_entity(product, f"/products/{slug}")})
```

### Flask

```python
from easeo.adapters.flask import Easeo

easeo = Easeo(app, config)
```

### Django

```python
from easeo.adapters.django import seo_head
```
