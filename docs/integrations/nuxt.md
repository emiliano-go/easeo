---
title: "Nuxt"
description: "Use the easeo Nuxt module and useEaseoSeo() composable."
---

# Nuxt { #nuxt }

`@easeo/nuxt` provides a lightweight module that stores the site config and a
`useEaseoSeo()` composable for pages.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* Nuxt 3 or newer.

## Install { #install }

```bash
npm install @easeo/nuxt
```

## Quick start { #usage }

```ts
import { useEaseoSeo } from "@easeo/nuxt";

useEaseoSeo({
  entity: {
    entityType: "post",
    title: article.title,
    description: article.description,
  },
  route: `/blog/${article.slug}`,
  config: {
    canonicalHost: "example.com",
    publicBaseUrl: "https://example.com",
  },
});
```

`useEaseoSeo()` always returns the built payload. When Nuxt's `useHead()`
auto-import is available in a page or component setup context, the composable
also pushes the tags into the page head.

## Module config { #module }

Register the module and store the config once:

```ts
// nuxt.config.ts
import easeoModule from "@easeo/nuxt";

export default defineNuxtConfig({
  modules: [
    easeoModule({
      config: {
        canonicalHost: "example.com",
        publicBaseUrl: "https://example.com",
        siteName: "Example",
      },
    }),
  ],
});
```

After that, `useEaseoSeo()` calls do not need to pass `config`.

## Function signature { #signature }

```ts
useEaseoSeo(input: { entity: SEOEntity; route: string; config?: SEOConfig }): SEOPayload
```

## Patterns { #patterns }

### Use the returned payload { #payload }

The composable returns the payload, so you can add JSON-LD or read fields:

```ts
const payload = useEaseoSeo({ entity, route });
const jsonLd = JSON.stringify(payload.schemaJsonLd).replace(/</g, "\\u003c");
useHead({ script: [{ type: "application/ld+json", innerHTML: jsonLd }] });
```

### Per-page config override { #override }

A per-call `config` wins over the module-level config, which is useful for a
sub-site or a locale:

```ts
useEaseoSeo({
  entity,
  route,
  config: { canonicalHost: "blog.example.com", publicBaseUrl: "https://blog.example.com" },
});
```

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| `No config provided` error | Neither the module nor the call passed a config |
| Head tags not applied | `useHead()` was not available in that setup context; use the returned payload |
| One page keeps another page's tags | The composable ran outside a page or component setup context |

## Notes { #notes }

* A per-call `config` always wins over the module-level config.
* Both default and named imports are supported.

## Related { #related }

* [Nuxt example](../examples/nuxt.md)
* [JavaScript API reference](../reference/javascript-api.md)
