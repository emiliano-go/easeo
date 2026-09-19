---
title: "SvelteKit"
description: "SvelteKit examples for easeo: page head, load functions, and layout defaults."
---

# SvelteKit { #sveltekit }

`@easeo/sveltekit` provides `buildEaseoPayload()` and the `<EaseoHead />`
component, which renders into `<svelte:head>`.

## Simple: a page head { #svelte-simple }

```svelte
<!-- src/routes/about/+page.svelte -->
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";

  const seo = buildEaseoPayload(
    { entityType: "page", title: "About", description: "About us." },
    "/about",
    { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
  );
</script>

<EaseoHead {seo} />
<h1>About</h1>
```

## Complex: a dynamic blog route { #svelte-complex }

Build the entity from data loaded in a `+page.server.ts` function, and pass the
result to the component.

```ts
// src/routes/blog/[slug]/+page.server.ts
import type { PageServerLoad } from "./$types";
import { getPost } from "$lib/posts";

export const load: PageServerLoad = async ({ params }) => {
  const post = await getPost(params.slug);
  return { post };
};
```

```svelte
<!-- src/routes/blog/[slug]/+page.svelte -->
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";
  export let data;

  const seo = buildEaseoPayload(
    {
      entityType: "post",
      title: data.post.title,
      description: data.post.excerpt,
      image: data.post.cover,
      authorName: data.post.author,
      publishedAt: data.post.publishedAt,
      breadcrumbs: [
        { name: "Home", url: "/" },
        { name: "Blog", url: "/blog" }
      ]
    },
    `/blog/${data.post.slug}`,
    {
      canonicalHost: "blog.example.com",
      publicBaseUrl: "https://blog.example.com",
      siteName: "Example Blog",
      titleTemplate: "{title} - Example Blog"
    }
  );
</script>

<EaseoHead {seo} />
<article>{@html data.post.body}</article>
```

## Complex: shared config in a layout { #svelte-layout }

Keep the config in one module and one `<EaseoHead />` per page.

```js
// src/lib/seo.js
export const config = {
  canonicalHost: "blog.example.com",
  publicBaseUrl: "https://blog.example.com",
  siteName: "Example Blog",
  titleTemplate: "{title} - Example Blog",
  defaultOgImage: "https://blog.example.com/assets/og-image.png",
};
```

```svelte
<!-- src/routes/blog/[slug]/+page.svelte -->
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";
  import { config } from "$lib/seo";
  export let data;

  const seo = buildEaseoPayload(
    { entityType: "post", title: data.post.title, description: data.post.excerpt },
    `/blog/${data.post.slug}`,
    config
  );
</script>

<EaseoHead {seo} />
```

## Notes { #svelte-notes }

* The JSON-LD block is serialized with `<` escaped, so schema data cannot close
  the script tag early.
* Use one `<EaseoHead />` per page; it merges into SvelteKit's `<svelte:head>`.
* The package is ESM only and exports `EaseoHead.svelte` explicitly.
