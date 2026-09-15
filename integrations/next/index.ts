// @easeo/next — Next.js integration for easeo
// Converts easeo payload to Next.js native Metadata object

import { buildSeoPayload, type SEOConfig, type SEOEntity, type SEOPayload } from "@easeo/core";

export interface EaseoMetadataInput {
  entity: SEOEntity;
  route: string;
  config?: SEOConfig;
}

/**
 * Convert easeo payload to Next.js Metadata object.
 * Use with Next.js generateMetadata() API.
 *
 * @example
 * ```ts
 * import { easeoMetadata } from "@easeo/next";
 *
 * export async function generateMetadata({ params }) {
 *   const product = await getProduct(params.slug);
 *   return easeoMetadata({
 *     entity: { entityType: "product", title: product.name, description: product.description },
 *     route: `/products/${product.slug}`,
 *   });
 * }
 * ```
 */
export function easeoMetadata(input: EaseoMetadataInput): Record<string, unknown> {
  const { entity, route, config } = input;

  const payload: SEOPayload = buildSeoPayload(entity, route, config ?? {
    canonicalHost: "",
    publicBaseUrl: "",
  });

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
      images: payload.openGraph.image ? [{
        url: payload.openGraph.image,
        width: payload.openGraph.imageWidth,
        height: payload.openGraph.imageHeight,
        alt: payload.openGraph.imageAlt,
      }] : undefined,
      locale: payload.openGraph.locale,
      type: payload.openGraph.type as "website" | "article",
    },
    twitter: {
      card: payload.twitter.card as "summary" | "summary_large_image",
      title: payload.twitter.title,
      description: payload.twitter.description,
      images: payload.twitter.image ? [payload.twitter.image] : undefined,
      site: payload.twitter.site,
      creator: payload.twitter.creator,
    },
  };
}

export default easeoMetadata;
