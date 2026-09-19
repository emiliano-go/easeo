---
title: "Astro"
description: "Use the easeo Astro integration for config injection and contract emission."
---

# Astro { #astro }

`@easeo/astro` is a build-time integration. It injects the site config into the
Vite define map and, optionally, emits an SEO contract after the build. Page
metadata itself is generated with `buildSeoPayload` from `@easeo/core`.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* Astro 3 or newer.
* A site-wide [`SEOConfig`](../tutorial/configuration.md).

## Install { #install }

```bash
npm install @easeo/astro
```

## Quick start { #usage }

```js
// astro.config.mjs
import { defineConfig } from "astro/config";
import easeo from "@easeo/astro";

export default defineConfig({
  integrations: [
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
| `contract` | `SEOContractConfig` | Optional. Emits `.easeo/contract.json` after the build |

## Hooks it registers { #hooks }

| Hook | Effect |
|---|---|
| `astro:config:setup` | Defines `__EASEO_CONFIG__` in the Vite define map |
| `astro:build:done` | Writes `.easeo/contract.json` when `contract` is set |

## Patterns { #patterns }

### Render a head per page { #head }

```astro
---
import { buildSeoPayload } from "@easeo/core";
const payload = buildSeoPayload(
  { entityType: "page", title: "About", description: "About us." },
  "/about",
  { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
);
---
<html>
  <head><Fragment set:html={payload.renderHtml()} /></head>
  <body><h1>About</h1></body>
</html>
```

### Emit a contract { #contract }

```js
easeo({
  config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
  contract: { canonicalHost: "example.com", scheme: "https" },
});
```

The contract is written to the build output directory:

```text
dist/
└── .easeo/
    └── contract.json
```

Commit it or upload it as a build artifact, then gate deployments on a diff.
See [Contracts in CI](../guides/contracts-in-ci.md).

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `.easeo/contract.json` missing | `contract` option not set, or the build failed before `astro:build:done` |
| Head tags missing | `payload.renderHtml()` not injected into the layout |

## Notes { #notes }

* The output directory is resolved with `fileURLToPath`, so paths with spaces
  work correctly.
* The contract file uses the canonical snake_case format that matches the
  published JSON schema.

## Related { #related }

* [Astro example](../examples/astro.md)
* [JavaScript API reference](../reference/javascript-api.md)
* [Contracts in CI](../guides/contracts-in-ci.md)
