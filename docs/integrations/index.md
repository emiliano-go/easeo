---
title: "Integrations"
description: "Adapters that wire easeo into JavaScript and Python frameworks."
---

# Integrations { #integrations }

easeo ships adapters for the most common frameworks. They are thin: each one
calls `build_seo_payload` and hands the result to the framework's native
metadata mechanism, so the behavior is identical everywhere.

## JavaScript / TypeScript { #javascript }

| Framework | Package | Entry point | Best for |
|---|---|---|---|
| [Next.js](next.md#nextjs) | `@easeo/next` | `easeoMetadata()` | App Router pages and route handlers |
| [Astro](astro.md#astro) | `@easeo/astro` | build integration, contract emission | Static and SSR content sites |
| [Vite](vite.md#vite) | `@easeo/vite` | `transformIndexHtml` plugin | SPAs and multi-page builds |
| [Nuxt](nuxt.md#nuxt) | `@easeo/nuxt` | `useEaseoSeo()` composable | Nuxt 3 pages and layouts |
| [SvelteKit](sveltekit.md#sveltekit) | `@easeo/sveltekit` | `buildEaseoPayload()` and `<EaseoHead />` | SvelteKit routes |
| [React](react.md#react) | `@easeo/react` | `<EaseoHead />` component | Client-rendered and SSR React apps |

All JavaScript integrations support both default and named imports:

```ts
import easeoMetadata from "@easeo/next";      // default
import { easeoMetadata } from "@easeo/next";  // named
```

## Python { #python }

| Framework | Import | Entry point |
|---|---|---|
| [FastAPI](fastapi.md#fastapi) | `from easeo.adapters.fastapi import EaseoSEO` | `for_entity()` |
| [Django](django.md#django) | `from easeo.adapters.django import seo_head` | settings plus template tags |
| [Flask](flask.md#flask) | `from easeo.adapters.flask import Easeo` | context processor and `for_entity()` |
| [Zensical](zensical.md#zensical) | `easeo.contrib.zensical` | markdown extension |

Python adapters are lazy and installed as extras:

```bash
pip install "easeo[fastapi]"   # or [django], [flask], [zensical], [all]
```

## How adapters work { #model }

Every adapter does the same three things:

1. Accept a site-wide `SEOConfig` once.
2. Turn a framework-specific object and route into an `SEOEntity`.
3. Return the payload, a dict, or rendered head HTML, depending on the
   framework's idiom.

Because the build runs in the shared Rust core, a Next.js page and a FastAPI
endpoint with the same inputs produce byte-identical metadata.

## Choosing an approach { #choosing }

* If your framework has a native metadata API, use the adapter that targets it.
  Next.js is the clearest example.
* If it does not, render `payload.render_html()` into your template, or build
  the payload yourself with the core API.
* For static sites, generate payloads at build time and commit the contract.

## Prerequisites { #prerequisites }

* JavaScript integrations: Node 20 or newer (see
  [Installation](../tutorial/installation.md)).
* Python adapters: Python 3.10 or newer. The framework itself is pulled in by
  the matching extra.

## Next steps { #next }

* [Installation](../tutorial/installation.md)
* [Examples](../examples/index.md): a simple and a complex example for every
  integration.
* [Framework Recipes](../guides/framework-recipes.md): short copy-paste
  patterns.
