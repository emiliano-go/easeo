---
title: "Examples"
description: "Runnable examples for every easeo integration, from a minimal payload to full production setups."
---

# Examples { #examples }

Every page in this section is a working example. Each one has a **simple**
version that fits in a few lines and a **complex** version that shows the shape
of a real application: dynamic routes, overrides, caching, contracts, and
validation.

Start with the core language examples if you are new to easeo, then jump to the
framework you use.

## Core library { #core }

| Page | What it covers |
|---|---|
| [Python](core-python.md#python) | Config, entities, overrides, factories, async, hooks, registries, contracts |
| [JavaScript](core-javascript.md#javascript) | Payloads, serialization, hooks, registries, factories, TypeScript |
| [Rust](core-rust.md#rust) | `easeo-core`, schema registry, hashing, rendered HTML |

## JavaScript frameworks { #javascript-frameworks }

| Page | Package |
|---|---|
| [Next.js](next.md#nextjs) | `@easeo/next` |
| [Astro](astro.md#astro) | `@easeo/astro` |
| [Vite](vite.md#vite) | `@easeo/vite` |
| [Nuxt](nuxt.md#nuxt) | `@easeo/nuxt` |
| [SvelteKit](sveltekit.md#sveltekit) | `@easeo/sveltekit` |
| [React](react.md#react) | `@easeo/react` |

## Python frameworks { #python-frameworks }

| Page | Import |
|---|---|
| [FastAPI](fastapi.md#fastapi) | `easeo.adapters.fastapi` |
| [Django](django.md#django) | `easeo.adapters.django` |
| [Flask](flask.md#flask) | `easeo.adapters.flask` |
| [Zensical](zensical.md#zensical) | `easeo.contrib.zensical` |

## Runnable files { #files }

A few examples are also committed as runnable files under `examples/`:

```text
examples/
├── python_basic.py
└── node_basic.mjs
```

```bash
python examples/python_basic.py
node examples/node_basic.mjs
```

The framework examples assume you already have a project scaffolded with that
framework. For setup steps, see the matching [integration page](../integrations/index.md).
