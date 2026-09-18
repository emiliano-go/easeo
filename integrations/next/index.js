// @easeo/next — Next.js integration for easeo
// Converts easeo payloads to Next.js native Metadata objects.
// Use with the Next.js generateMetadata() API — no HTML manipulation.

const { buildSeoPayload } = require("@easeo/core");

/**
 * @typedef {Object} EaseoMetadataInput
 * @property {import("./index.d").SEOEntity} entity
 * @property {string} route
 * @property {import("./index.d").SEOConfig} config - required: core rejects empty canonicalHost
 */

/**
 * Convert an easeo payload to a Next.js Metadata object.
 *
 * @param {EaseoMetadataInput} input
 * @returns {Record<string, unknown>} Next.js Metadata
 *
 * @example
 * const { easeoMetadata } = require("@easeo/next");
 *
 * export async function generateMetadata({ params }) {
 *   const product = await getProduct(params.slug);
 *   return easeoMetadata({
 *     entity: { entityType: "product", title: product.name, description: product.description },
 *     route: `/products/${product.slug}`,
 *     config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
 *   });
 * }
 */
function easeoMetadata(input) {
  const { entity, route, config } = input;
  if (!config || !config.canonicalHost) {
    throw new Error(
      "@easeo/next: 'config' with a non-empty 'canonicalHost' is required."
    );
  }

  const payload = buildSeoPayload(entity, route, config);

  return {
    title: payload.title,
    description: payload.description,
    alternates: {
      canonical: payload.canonical,
    },
    robots: payload.robots,
    openGraph: {
      title: payload.openGraph.title,
      description: payload.openGraph.description,
      url: payload.openGraph.url,
      siteName: payload.openGraph.siteName,
      images: payload.openGraph.image
        ? [
            {
              url: payload.openGraph.image,
              width: payload.openGraph.imageWidth,
              height: payload.openGraph.imageHeight,
              alt: payload.openGraph.imageAlt,
            },
          ]
        : undefined,
      locale: payload.openGraph.locale,
      type: payload.openGraph.type,
    },
    twitter: {
      card: payload.twitter.card,
      title: payload.twitter.title,
      description: payload.twitter.description,
      images: payload.twitter.image ? [payload.twitter.image] : undefined,
      site: payload.twitter.site,
      creator: payload.twitter.creator,
    },
  };
}

module.exports = easeoMetadata;
module.exports.easeoMetadata = easeoMetadata;
module.exports.default = easeoMetadata;
