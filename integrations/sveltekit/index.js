// @easeo/sveltekit — SvelteKit integration for easeo
// buildEaseoPayload() for <svelte:head> blocks, plus the <EaseoHead /> component.

import { buildSeoPayload } from "@easeo/core";

/**
 * Build an SEO payload for use in SvelteKit pages.
 *
 * @param {import("./index.d").SEOEntity} entity
 * @param {string} route
 * @param {import("./index.d").SEOConfig} config
 * @returns {import("./index.d").SEOPayload}
 *
 * @example
 * <script>
 *   import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";
 *   import { page } from "$app/stores";
 *
 *   const seo = buildEaseoPayload(
 *     { entityType: "post", title: "Hello", description: "A post" },
 *     $page.url.pathname,
 *     { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
 *   );
 * </script>
 *
 * <EaseoHead {seo} />
 */
export function buildEaseoPayload(entity, route, config) {
  return buildSeoPayload(entity, route, config);
}

export { default as EaseoHead } from "./EaseoHead.svelte";
