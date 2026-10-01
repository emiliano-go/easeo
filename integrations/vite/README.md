# @easeo/vite

Vite integration for [`easeo`](https://easeo.emiliano-go.com/). Injects SEO meta tags during `transformIndexHtml`.

## Install

```bash
npm install @easeo/core @easeo/vite
```

## Usage

```typescript
import { defineConfig } from "vite";
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
    }),
  ],
});
```

## Documentation

https://easeo.emiliano-go.com/integrations/vite/

## License

MIT
