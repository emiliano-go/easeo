# @easeo/astro

Astro integration for [`easeo`](https://easeo.emiliano-go.com/). Generates SEO metadata at build time and can emit a machine-readable contract.

## Install

```bash
npm install @easeo/core @easeo/astro
```

## Usage

```javascript
// astro.config.mjs
import easeo from "@easeo/astro";

export default {
  integrations: [
    easeo({
      config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
      // Optional: write .easeo/contract.json to the output directory after build.
      contract: { site: { canonicalHost: "example.com", scheme: "https" } },
    }),
  ],
};
```

## Documentation

https://easeo.emiliano-go.com/integrations/astro/

## License

MIT
