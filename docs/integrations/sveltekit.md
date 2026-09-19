---
title: "SvelteKit"
description: "Use buildEaseoPayload() and the EaseoHead component in SvelteKit."
---

# SvelteKit { #sveltekit }

`@easeo/sveltekit` gives you `buildEaseoPayload()` and a `<EaseoHead />`
component that renders into `<svelte:head>`.

## Prerequisites { #prerequisites }

* Node 20 or newer.
* Svelte 4 or newer, SvelteKit 1 or newer.

## Install { #install }

```bash
npm install @easeo/sveltekit
```

## Quick start { #usage }

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

## API { #api }

| Export | Description |
|---|---|
| `buildEaseoPayload(entity, route, config)` | Straight pass-through to `buildSeoPayload` |
| `EaseoHead` | Component that renders the payload into `<svelte:head>` |

## What the component renders { #component }

`<EaseoHead />` emits the title, description, canonical link, robots, Open
Graph, Twitter, and JSON-LD tags.

The JSON-LD payload is serialized with `<` escaped before it is injected, so a
value containing a closing script tag cannot break out of the block.

## Patterns { #patterns }

### Build from loaded data { #load }

Load the content in a `+page.server.ts` function and build the entity in the
page. See the [SvelteKit example](../examples/sveltekit.md#svelte-complex).

### Shared config { #shared-config }

Put the config in `src/lib/seo.js` and import it wherever you call
`buildEaseoPayload`. See the
[layout pattern](../examples/sveltekit.md#svelte-layout).

## One page, one head { #one-head }

Use `<EaseoHead />` once per page. It writes into SvelteKit's `<svelte:head>`,
which merges cleanly with other head content.

## Troubleshooting { #troubleshooting }

| Symptom | Cause |
|---|---|
| Two `<title>` tags | `<EaseoHead />` plus a manual title in the page |
| `.svelte` import error in plain Node | The package must be processed by the Svelte compiler |
| Stale tags after navigation | Mount `<EaseoHead />` in the page, not once globally |

## Notes { #notes }

* The package is ESM only and exports `EaseoHead.svelte` explicitly.
* `buildEaseoPayload` and `EaseoHead` can also be imported from the package
  root.

## Related { #related }

* [SvelteKit example](../examples/sveltekit.md)
* [JavaScript API reference](../reference/javascript-api.md)
