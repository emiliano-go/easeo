---
title: "Vite"
description: "Inject SEO tags into built HTML with the easeo Vite plugin."
---

# Vite { #vite }

`@easeo/vite` injects SEO tags into the built `index.html` through Vite's
`transformIndexHtml` hook.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* Vite 4 or newer.

## Install { #install }

```bash
npm install @easeo/vite
```

## Quick start { #usage }

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

## Options { #options }

| Option | Type | Description |
|---|---|---|
| `config` | `SEOConfig` | Required. Site-wide settings |

## What it injects { #injects }

For each built HTML page, the plugin adds:

* `<title>` (from `config.siteName`)
* `<meta name="description">`
* `<link rel="canonical">`
* `<meta name="robots">`
* Open Graph tags
* Twitter Card tags
* a JSON-LD `<script>` when a schema exists

The route is derived from the page path. A trailing `index.html` is stripped,
so `/blog/index.html` becomes `/blog`, while a path like `/reindex` is left
untouched.

## Patterns { #patterns }

### Multi-page builds { #mpa }

List every HTML entry point in `build.rollupOptions.input`; each becomes its own
route. See the [Vite example](../examples/vite.md#vite-mpa).

### Per-page metadata { #per-page }

The plugin only knows the route. For per-page titles and descriptions, prebuild
payloads with `@easeo/core` and merge them into the templates. See the
[prebuild pattern](../examples/vite.md#vite-prebuilt).

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| Tags appear twice | The plugin and another plugin both inject a `<title>` |
| Wrong canonical for a page | The page is not listed in `build.rollupOptions.input`, so its path is unexpected |
| JSON-LD looks escaped | This is intentional: `<` is escaped so the value cannot close the script tag |

## Notes { #notes }

* The JSON-LD `children` is a JSON string with `<` escaped, so it is safe to
  inject into a script tag.
* For per-page metadata beyond path-derived defaults, use a framework with a
  data layer, or prebuild payloads and inject them yourself.

## Related { #related }

* [Vite example](../examples/vite.md)
* [JavaScript API reference](../reference/javascript-api.md)
