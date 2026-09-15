import type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";
export type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";

/** Build an SEO payload for use in SvelteKit pages. */
export declare function buildEaseoPayload(
  entity: SEOEntity,
  route: string,
  config: SEOConfig
): SEOPayload;

export interface EaseoHeadProps {
  seo: SEOPayload;
}

/**
 * Svelte component that renders the payload into <svelte:head>.
 * Typed as `any` to stay compatible across Svelte 4/5 component typings.
 */
export declare const EaseoHead: any;
