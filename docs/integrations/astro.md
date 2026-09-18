---
title: "Astro"
---

# Astro { #astro }

`@easeo/astro` is a build-time integration. It injects the site config into the
Vite define map and, optionally, emits an SEO contract after the build.

## Install { #install }

```bash
npm install @easeo/astro
```

## Usage { #usage }

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
      },
      // Optional: write <outDir>/.easeo/contract.json after the build.
      contract: { canonicalHost: "example.com", scheme: "https" },
    }),
  ],
});
```

## Hooks it registers { #hooks }

| Hook | Effect |
|---|---|
| `astro:config:setup` | Defines `__EASEO_CONFIG__` for use in components |
| `astro:build:done` | Writes `.easeo/contract.json` when `contract` is set |

## Emitting the contract { #contract }

The contract is written to the build output directory:

```text
dist/
└── .easeo/
    └── contract.json
```

Commit it or upload it as a build artifact, then gate deployments on a diff.
See [Contracts in CI](../guides/contracts-in-ci.md).

## Notes { #notes }

* The output directory is resolved with `fileURLToPath`, so paths with spaces
  work correctly.
* The contract file uses the canonical snake_case format that matches the
  published JSON schema.
