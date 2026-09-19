---
title: "Tutorial"
description: "What easeo is, what it is not, and how the tutorial is organized."
---

# Tutorial { #tutorial }

This tutorial teaches you how to use **easeo** to turn a content entity into a
deterministic SEO payload, step by step. Each page builds on the previous one,
but every page is self-contained: you can jump straight to the topic you need
and copy-paste the examples.

## What is easeo? { #what-is-easeo }

**easeo** is a deterministic SEO metadata generator. It takes a content entity,
a route path, and a site configuration, and produces one structured payload:

- canonical URL
- title and description
- robots directives
- Open Graph tags
- Twitter Card tags
- JSON-LD structured data

The core is written in Rust and shipped to Python and JavaScript/TypeScript, so
the same inputs produce byte-for-byte identical output in every language.

```text
content entity + route + config
    |
    v
easeo (library)
    |
    v
SEOPayload
    |
    +--> framework adapter --> <head>
    |
    +--> SEO contract --> cheseo validation
```

## What easeo is not { #what-easeo-is-not }

- **Not an SEO crawler.** That is a separate tool.
- **Not a score generator.** It does not grade your content.
- **Not a keyword research tool.** It formats the data you give it.
- **Not a browser automation framework.** It performs no I/O and no network
  calls.
- **Not an analytics platform.** It stores no state.

## How the tutorial works { #how-the-tutorial-works }

Each page covers one topic:

1. [Installation](installation.md#installation): install the Python or
   JavaScript package.
2. [First Payload](first-payload.md#first-payload): build your first payload.
3. [Fallback and Overrides](fallback-and-overrides.md#fallback-and-overrides):
   control which value wins.
4. [Rendering HTML](rendering.md#rendering-html): emit a ready-to-use
   `<head>` block.
5. [Contracts](contracts.md#contracts): turn SEO intent into a testable
   artifact.
6. [Configuration](configuration.md#configuration): every config field.

!!! tip "Two audiences, one API"

    The Python and JavaScript APIs are intentionally mirrored. Every concept
    on these pages exists in both, with `snake_case` in Python and `camelCase`
    in JavaScript. Pick your language and follow along; the other binding
    works the same way.

## Recap { #recap }

* **easeo** generates deterministic SEO payloads from content entities.
* One primary function: `build_seo_payload(entity, route, config)`.
* The Rust core guarantees identical output across Python and JavaScript.

**Next:** [Installation](installation.md#installation), install easeo and
verify the version.
