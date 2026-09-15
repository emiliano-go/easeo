// @easeo/nuxt — Nuxt integration for easeo
// Provides the useEaseoSeo() composable for SEO metadata.
//
// useEaseoSeo() always returns the built payload. When Nuxt's useHead()
// auto-import is available (any Nuxt 3+ page/component setup context),
// the tags are also pushed into the page head automatically.

const { buildSeoPayload } = require("@easeo/core");

let globalConfig = null;

/**
 * @typedef {Object} EaseoNuxtConfig
 * @property {import("./index.d").SEOConfig} config
 */

/**
 * Lightweight Nuxt module: stores the site-wide config so
 * useEaseoSeo() calls don't need to repeat it.
 *
 * @param {EaseoNuxtConfig} options
 */
function easeoModule(options) {
  if (options && options.config) {
    globalConfig = options.config;
  }
  return { name: "@easeo/nuxt" };
}

/**
 * Composable for setting SEO metadata in Nuxt pages.
 *
 * @param {{ entity: import("./index.d").SEOEntity, route: string, config?: import("./index.d").SEOConfig }} input
 * @returns {import("./index.d").SEOPayload}
 *
 * @example
 * import { useEaseoSeo } from "@easeo/nuxt";
 *
 * useEaseoSeo({
 *   entity: { entityType: "post", title: article.title, description: article.description },
 *   route: `/blog/${article.slug}`,
 *   config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
 * });
 */
function useEaseoSeo(input) {
  const config = input.config ?? globalConfig;
  if (!config) {
    throw new Error(
      "@easeo/nuxt: No config provided. Pass config to useEaseoSeo() or configure the module."
    );
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
      link: [{ rel: "canonical", href: payload.canonical }],
    });
  }

  return payload;
}

module.exports = { useEaseoSeo, default: easeoModule };
