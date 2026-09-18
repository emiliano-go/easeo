---
title: "Integrations"
---

# Integrations { #integrations }

easeo ships adapters for the most common frameworks. They are thin: each one
calls `build_seo_payload` and hands the result to the framework's native
metadata mechanism.

## JavaScript / TypeScript { #javascript }

| Framework | Package | Entry point |
|---|---|---|
| [Next.js](next.md#nextjs) | `@easeo/next` | `easeoMetadata()` |
| [Astro](astro.md#astro) | `@easeo/astro` | build integration plus contract emission |
| [Vite](vite.md#vite) | `@easeo/vite` | `transformIndexHtml` plugin |
| [Nuxt](nuxt.md#nuxt) | `@easeo/nuxt` | `useEaseoSeo()` composable |
| [SvelteKit](sveltekit.md#sveltekit) | `@easeo/sveltekit` | `buildEaseoPayload()` plus `<EaseoHead />` |
| [React](react.md#react) | `@easeo/react` | `<EaseoHead />` component |

All JavaScript integrations support both default and named imports:

```ts
import easeoMetadata from "@easeo/next";      // default
import { easeoMetadata } from "@easeo/next";  // named
```

## Python { #python }

| Framework | Import |
|---|---|
| [FastAPI](fastapi.md#fastapi) | `from easeo.adapters.fastapi import EaseoSEO` |
| [Django](django.md#django) | `from easeo.adapters.django import seo_head` |
| [Flask](flask.md#flask) | `from easeo.adapters.flask import Easeo` |
| [Zensical](zensical.md#zensical) | `easeo.contrib.zensical` markdown extension |

Python adapters are lazy and installed as extras: `pip install easeo[fastapi]`,
`[django]`, `[flask]`, `[zensical]`, or `[all]`.

## Choosing an approach { #choosing }

* If your framework has a native metadata API, use the adapter that targets it.
  Next.js is the clearest example.
* If it does not, use the render helpers and inject `payload.render_html()`
  into your template.
* For static sites, generate payloads at build time and commit the contract.
