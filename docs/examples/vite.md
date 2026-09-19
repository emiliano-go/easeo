---
title: "Vite"
description: "Vite examples for easeo: single-page injection and multi-page builds with per-page defaults."
---

# Vite { #vite }

`@easeo/vite` injects SEO tags into each built HTML file through
`transformIndexHtml`.

## Simple: a single-page app { #vite-simple }

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

The plugin derives the route from the page path and fills the title from
`siteName`.

## Complex: multiple HTML entry points { #vite-mpa }

For a multi-page build, each HTML file becomes its own route. A trailing
`index.html` is stripped, so `about/index.html` becomes `/about`:

```text
src/
├── index.html          -> /
├── about/index.html    -> /about
└── pricing/index.html  -> /pricing
```

```js
// vite.config.mjs
import { defineConfig } from "vite";
import { resolve } from "node:path";
import easeo from "@easeo/vite";

export default defineConfig({
  plugins: [
    easeo({
      config: {
        canonicalHost: "example.com",
        publicBaseUrl: "https://example.com",
        siteName: "Example",
        defaultOgImage: "https://example.com/assets/og-image.png",
      },
    }),
  ],
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, "src/index.html"),
        about: resolve(__dirname, "src/about/index.html"),
        pricing: resolve(__dirname, "src/pricing/index.html"),
      },
    },
  },
});
```

## Complex: prebuild payloads for per-page metadata { #vite-prebuilt }

The plugin only knows the route, so for per-page titles and descriptions,
generate payloads ahead of time and merge them into the HTML. A small prebuild
script writes one JSON file per route:

```js
// scripts/build-seo.mjs
import { writeFileSync, mkdirSync } from "node:fs";
import { buildSeoPayload } from "@easeo/core";

const config = { canonicalHost: "example.com", publicBaseUrl: "https://example.com" };
const pages = [
  { route: "/", entity: { entityType: "home", title: "Example" } },
  { route: "/about", entity: { entityType: "page", title: "About", description: "About us." } },
  { route: "/pricing", entity: { entityType: "page", title: "Pricing", description: "Plans." } },
];

mkdirSync(".seo", { recursive: true });
for (const { route, entity } of pages) {
  const payload = buildSeoPayload(entity, route, config);
  const name = route === "/" ? "index" : route.replace(/\//g, "_");
  writeFileSync(`.seo/${name}.json`, JSON.stringify(payload.toDict()));
}
```

Run it before `vite build` (`node scripts/build-seo.mjs && vite build`) and
consume the files from your templates, or keep the plugin for the defaults.

## Notes { #vite-notes }

* The JSON-LD `children` is a JSON string with `<` escaped, so it is safe to
  inject into a script tag.
* The plugin runs for dev and preview too, so the head is correct while you
  develop.
* For richer per-page data, a framework integration with a data layer is a
  better fit.
