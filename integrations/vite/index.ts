// @easeo/vite — Vite plugin for easeo
// Uses transformIndexHtml hook to inject SEO meta tags

import { buildSeoPayload, type SEOConfig, type SEOEntity } from "@easeo/core";

export interface EaseoViteConfig {
  config: SEOConfig;
  entities?: Array<{
    entity: SEOEntity;
    route: string;
  }>;
}

/**
 * Vite plugin for easeo SEO generation.
 *
 * @example
 * ```ts
 * import easeo from "@easeo/vite";
 *
 * export default defineConfig({
 *   plugins: [
 *     easeo({
 *       config: { publicBaseUrl: "https://example.com", canonicalHost: "example.com" },
 *     }),
 *   ],
 * });
 * ```
 */
export default function easeo(options: EaseoViteConfig) {
  const { config } = options;

  return {
    name: "@easeo/vite",

    transformIndexHtml(html: string, ctx: { path: string }) {
      // Build a default entity from the page path
      const route = ctx.path.replace(/\.html$/, "").replace(/index$/, "") || "/";
      const entity: SEOEntity = {
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

      // JSON-LD
      if (payload.schemaJsonLd) {
        tags.push({
          tag: "script",
          attrs: { type: "application/ld+json" },
          children: JSON.stringify(payload.schemaJsonLd),
        });
      }

      return tags;
    },
  };
}
