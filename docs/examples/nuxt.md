---
title: "Nuxt"
description: "Nuxt examples for easeo: the useEaseoSeo() composable, module config, and useHead integration."
---

# Nuxt { #nuxt }

`@easeo/nuxt` provides a module that stores the site config and a
`useEaseoSeo()` composable for pages.

## Simple: configure once, use everywhere { #nuxt-simple }

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

```ts
// pages/about.vue
<script setup lang="ts">
import { useEaseoSeo } from "@easeo/nuxt";

useEaseoSeo({
  entity: { entityType: "page", title: "About", description: "About us." },
  route: "/about",
});
</script>
```

## Complex: a blog post page { #nuxt-complex }

For a dynamic route, build the entity from the route data. Passing `config` per
call overrides the module-level config for that page.

```vue
<!-- pages/blog/[slug].vue -->
<script setup lang="ts">
import { useEaseoSeo } from "@easeo/nuxt";

const route = useRoute();
const { data: post } = await useFetch(`/api/posts/${route.params.slug}`);

useEaseoSeo({
  entity: {
    entityType: "post",
    title: post.value.title,
    description: post.value.excerpt,
    image: post.value.cover,
    authorName: post.value.author,
    publishedAt: post.value.publishedAt,
    breadcrumbs: [
      { name: "Home", url: "/" },
      { name: "Blog", url: "/blog" },
    ],
  },
  route: `/blog/${route.params.slug}`,
  config: {
    canonicalHost: "blog.example.com",
    publicBaseUrl: "https://blog.example.com",
    siteName: "Example Blog",
    titleTemplate: "{title} - Example Blog",
  },
});
</script>
```

## Complex: reuse the payload { #nuxt-payload }

`useEaseoSeo` returns the payload, so you can use it for `useHead`, JSON-LD, or
caching:

```vue
<script setup lang="ts">
import { useEaseoSeo } from "@easeo/nuxt";

const payload = useEaseoSeo({
  entity: { entityType: "product", title: "Widget", description: "A widget." },
  route: "/products/widget",
});

// Escape "<" so schema data cannot close the script tag early.
const jsonLd = JSON.stringify(payload.schemaJsonLd).replace(/</g, "\\u003c");

useHead({
  script: [{ type: "application/ld+json", innerHTML: jsonLd }],
});
</script>
```

## Notes { #nuxt-notes }

* `useEaseoSeo()` always returns the payload. When Nuxt's `useHead()`
  auto-import is available in a setup context, it also pushes the basic tags.
* A per-call `config` wins over the module-level config.
* If neither is present, `useEaseoSeo()` throws with a clear message.
