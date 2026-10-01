# @easeo/nuxt

Nuxt module for [`easeo`](https://easeo.emiliano-go.com/). Stores site-wide config and exposes the `useEaseoSeo()` composable.

## Install

```bash
npm install @easeo/core @easeo/nuxt
```

## Usage

```javascript
// nuxt.config.ts
import { easeoModule } from "@easeo/nuxt";

export default defineNuxtConfig({
  modules: [
    easeoModule({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
    }),
  ],
});
```

```javascript
// pages/blog/[slug].vue
const seo = useEaseoSeo({
  entity: { entityType: "post", title: "Hello World", description: "An example post." },
  route: "/blog/hello",
});
```

## Documentation

https://easeo.emiliano-go.com/integrations/nuxt/

## License

MIT
