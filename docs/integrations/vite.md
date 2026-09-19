---
title: "Vite"
description: "Inject SEO tags into built HTML with the easeo Vite plugin."
---

# Vite { #vite }

`@easeo/vite` injects SEO tags into the built `index.html` through Vite's
`transformIndexHtml` hook.

## Install { #install }

```bash
npm install @easeo/vite
```

## Usage { #usage }

```js
// vite.config.mjs
import { defineConfig } from "vite";
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [
    easeo({
      config: {
        canonicalHost: "example.com",
        publicBaseUrl: "https://example.com",
        siteName: "Example",
      },
    }),
  ],
});
```

## What it injects { #injects }

For each built HTML page, the plugin adds:

* `<title>`
* `<meta name="description">`
* `<link rel="canonical">`
* `<meta name="robots">`
* Open Graph tags
* Twitter Card tags
* a JSON-LD `<script>` when a schema exists

The route is derived from the page path. A trailing `index.html` is stripped,
so `/blog/index.html` becomes `/blog`, while a path like `/reindex` is left
untouched.

## Notes { #notes }

* The JSON-LD `children` is a JSON string with `<` escaped, so it is safe to
  inject into a script tag.
* For per-page metadata beyond the path-derived defaults, use a framework with
  a data layer, or prebuild payloads and inject them yourself.
