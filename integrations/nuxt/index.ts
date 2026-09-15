// @easeo/nuxt — Nuxt integration for easeo
// Provides useEaseoSeo() composable for SEO metadata

import { buildSeoPayload, type SEOConfig, type SEOEntity } from "@easeo/core";

let globalConfig: SEOConfig | null = null;

export interface EaseoNuxtConfig {
  config: SEOConfig;
}

/**
 * Nuxt module for easeo SEO generation.
 *
 * @example
 * ```ts
 * // nuxt.config.ts
 * export default defineNuxtConfig({
 *   modules: ["@easeo/nuxt"],
 *   easeo: {
 *     config: {
 *       siteName: "My Site",
 *       publicBaseUrl: "https://example.com",
 *       canonicalHost: "example.com",
 *       publicBaseUrl: "https://example.com",
 *     },
 *   },
 * });
 * ```
 */
export default function easeoModule(options: EaseoNuxtConfig) {
  globalConfig = options.config;

  return {
    name: "@easeo/nuxt",
    setup(_nuxtOptions: unknown, resolve: (key: string) => string) {
      // Add composable to Nuxt
      const composablesDir = resolve("composables");
      // The composable will be auto-imported by Nuxt
    },
  };
}

/**
 * Composable for setting SEO metadata in Nuxt pages.
 *
 * @example
 * ```vue
 * <script setup>
 * import { useEaseoSeo } from "@easeo/nuxt";
 *
 * const article = await useFetch("/api/article");
 *
 * useEaseoSeo({
 *   entity: { entityType: "post", title: article.value.title, description: article.value.description },
 *   route: `/blog/${article.value.slug}`,
 * });
 * </script>
 * ```
 */
export function useEaseoSeo(input: { entity: SEOEntity; route: string; config?: SEOConfig }) {
  const config = input.config ?? globalConfig;
  if (!config) {
    throw new Error("@easeo/nuxt: No config provided. Pass config to useEaseoSeo() or configure the module.");
  }

  const payload = buildSeoPayload(input.entity, input.route, config);

  // Use Nuxt's useHead() if available (Nuxt 3+)
  if (typeof useHead !== "undefined") {
    useHead({
      title: payload.title,
      meta: [
        { name: "description", content: payload.description },
        { property: "og:title", content: payload.openGraph.title },
        { property: "og:description", content: payload.openGraph.description },
        { property: "og:url", content: payload.openGraph.url },
        { property: "og:image", content: payload.openGraph.image },
        { name: "twitter:card", content: payload.twitter.card },
        { name: "twitter:title", content: payload.twitter.title },
        { name: "twitter:description", content: payload.twitter.description },
      ],
      link: [
        { rel: "canonical", href: payload.canonical },
      ],
    });
  }

  return payload;
}
