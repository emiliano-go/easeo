// @easeo/sveltekit — SvelteKit integration for easeo
// Provides EaseoHead component for SEO metadata in <svelte:head>

import { buildSeoPayload, type SEOConfig, type SEOEntity, type SEOPayload } from "@easeo/core";

export interface EaseoHeadProps {
  entity: SEOEntity;
  route: string;
  config?: SEOConfig;
}

/**
 * Build SEO payload for use in SvelteKit pages.
 *
 * @example
 * ```svelte
 * <script>
 *   import { buildEaseoPayload } from "@easeo/sveltekit";
 *   import { page } from "$app/stores";
 *
 *   const seo = buildEaseoPayload(
 *     { entityType: "post", title: "Hello", description: "A post" },
 *     $page.url.pathname,
 *     { publicBaseUrl: "https://example.com", canonicalHost: "example.com" }
 *   );
 * </script>
 *
 * <svelte:head>
 *   <title>{seo.title}</title>
 *   <meta name="description" content={seo.description} />
 *   <link rel="canonical" href={seo.canonical} />
 *   <meta property="og:title" content={seo.openGraph.title} />
 *   <meta name="twitter:card" content={seo.twitter.card} />
 * </svelte:head>
 * ```
 */
export function buildEaseoPayload(
  entity: SEOEntity,
  route: string,
  config: SEOConfig
): SEOPayload {
  return buildSeoPayload(entity, route, config);
}

/**
 * Svelte component for injecting SEO head tags.
 * Use with SvelteKit's <svelte:head>.
 *
 * @example
 * ```svelte
 * <script>
 *   import { buildEaseoPayload } from "@easeo/sveltekit";
 *   import EaseoHead from "@easeo/sveltekit/EaseoHead.svelte";
 *
 *   const seo = buildEaseoPayload(
 *     { entityType: "post", title: "Hello", description: "A post" },
 *     $page.url.pathname,
 *     { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
 *   );
 * </script>
 *
 * <EaseoHead {seo} />
 * ```
 */
