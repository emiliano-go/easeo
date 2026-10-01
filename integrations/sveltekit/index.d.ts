import type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";
export type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";

/**
 * Builds an SEO payload for use in SvelteKit pages.
 *
 * @param entity - Content entity for the page.
 * @param route - Route path for the page.
 * @param config - Site-wide easeo configuration.
 * @returns The resolved SEO payload.
 */
export declare function buildEaseoPayload(
  entity: SEOEntity,
  route: string,
  config: SEOConfig
): SEOPayload;

/** Props accepted by the `EaseoHead` component. */
export interface EaseoHeadProps {
  /** Payload to render into `<svelte:head>`. */
  seo: SEOPayload;
}

/**
 * Svelte component that renders the payload into `<svelte:head>`.
 *
 * Typed as `any` to stay compatible across Svelte 4 and 5 component typings.
 */
export declare const EaseoHead: any;
