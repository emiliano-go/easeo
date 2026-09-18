// @easeo/vite — Vite plugin for easeo
// Injects SEO meta tags via the transformIndexHtml hook.

const { buildSeoPayload } = require("@easeo/core");

/**
 * @typedef {Object} EaseoViteConfig
 * @property {import("./index.d").SEOConfig} config - site-wide easeo config
 */

/**
 * Vite plugin for easeo SEO generation.
 *
 * @param {EaseoViteConfig} options
 * @returns {{ name: string, transformIndexHtml: Function }}
 *
 * @example
 * // vite.config.ts
 * import easeo from "@easeo/vite";
 *
 * export default defineConfig({
 *   plugins: [
 *     easeo({
 *       config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
 *     }),
 *   ],
 * });
 */
function easeo(options) {
  const { config } = options;

  return {
    name: "@easeo/vite",

    transformIndexHtml(html, ctx) {
      // Build a default entity from the page path. Only strip a trailing
      // "index" when it is its own segment (e.g. /blog/index.html), so
      // routes like /reindex are left intact.
      const route = ctx.path.replace(/\.html$/, "").replace(/(^|\/)index$/, "") || "/";
      const entity = {
        entityType: "page",
        title: config.siteName ?? "Page",
      };

      const payload = buildSeoPayload(entity, route, config);

      const tags = [
        { tag: "title", children: payload.title },
        { tag: "meta", attrs: { name: "description", content: payload.description } },
        { tag: "link", attrs: { rel: "canonical", href: payload.canonical } },
        { tag: "meta", attrs: { name: "robots", content: payload.robots } },
      ];

      // Open Graph tags
      if (payload.openGraph.title) {
        tags.push({ tag: "meta", attrs: { property: "og:title", content: payload.openGraph.title } });
      }
      if (payload.openGraph.description) {
        tags.push({ tag: "meta", attrs: { property: "og:description", content: payload.openGraph.description } });
      }
      if (payload.openGraph.url) {
        tags.push({ tag: "meta", attrs: { property: "og:url", content: payload.openGraph.url } });
      }
      if (payload.openGraph.image) {
        tags.push({ tag: "meta", attrs: { property: "og:image", content: payload.openGraph.image } });
      }
      if (payload.openGraph.siteName) {
        tags.push({ tag: "meta", attrs: { property: "og:site_name", content: payload.openGraph.siteName } });
      }

      // Twitter tags
      if (payload.twitter.card) {
        tags.push({ tag: "meta", attrs: { name: "twitter:card", content: payload.twitter.card } });
      }
      if (payload.twitter.title) {
        tags.push({ tag: "meta", attrs: { name: "twitter:title", content: payload.twitter.title } });
      }
      if (payload.twitter.description) {
        tags.push({ tag: "meta", attrs: { name: "twitter:description", content: payload.twitter.description } });
      }

      // JSON-LD. Escape "<" so a closing script tag in the data cannot break out.
      if (payload.schemaJsonLd) {
        tags.push({
          tag: "script",
          attrs: { type: "application/ld+json" },
          children: JSON.stringify(payload.schemaJsonLd).replace(/</g, "\\u003c"),
        });
      }

      return tags;
    },
  };
}

module.exports = easeo;
module.exports.default = easeo;
