---
title: "SvelteKit"
---

# SvelteKit { #sveltekit }

`@easeo/sveltekit` gives you `buildEaseoPayload()` and a `<EaseoHead />`
component that renders into `<svelte:head>`.

## Install { #install }

```bash
npm install @easeo/sveltekit
```

## Usage { #usage }

```svelte
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";
  import { page } from "$app/stores";

  const seo = buildEaseoPayload(
    { entityType: "post", title: "Hello", description: "A post" },
    $page.url.pathname,
    { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
  );
</script>

<EaseoHead {seo} />
```

`buildEaseoPayload(entity, route, config)` is a straight pass-through to
`buildSeoPayload`; the component then renders the payload's fields.

## What the component renders { #component }

`<EaseoHead />` emits the title, description, canonical link, robots, Open
Graph, Twitter, and JSON-LD tags.

The JSON-LD payload is serialized with `<` escaped before it is injected, so a
value containing a closing script tag cannot break out of the block.

## One page, one head { #one-head }

Use `<EaseoHead />` once per page. It writes into SvelteKit's `<svelte:head>`,
which merges cleanly with other head content.

## Notes { #notes }

* The package is ESM only and exports `EaseoHead.svelte` explicitly.
* `buildEaseoPayload` and `EaseoHead` can also be imported from the package
  root.
