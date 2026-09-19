---
title: "Nuxt"
description: "Use the easeo Nuxt module and useEaseoSeo() composable."
---

# Nuxt { #nuxt }

`@easeo/nuxt` provides a lightweight module that stores the site config and a
`useEaseoSeo()` composable for pages.

## Install { #install }

```bash
npm install @easeo/nuxt
```

## Usage { #usage }

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

You can register the module and store the config once:

```ts
// nuxt.config.ts
import easeoModule from "@easeo/nuxt";

export default defineNuxtConfig({
  modules: [easeoModule({ config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" } })],
});
```

After that, `useEaseoSeo()` calls do not need to pass `config`.

## Notes { #notes }

* Passing `config` per call always wins over the module-level config.
* If neither is present, `useEaseoSeo()` throws with a clear message.
* Both default and named imports are supported.
