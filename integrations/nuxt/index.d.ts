import type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";
export type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";

/** Options accepted by the easeo Nuxt module. */
export interface EaseoNuxtConfig {
  /** Site-wide easeo configuration stored for `useEaseoSeo()`. */
  config: SEOConfig;
}

/** Lightweight Nuxt module: stores site-wide config for `useEaseoSeo()`. */
export declare function easeoModule(options: EaseoNuxtConfig): { name: string };

/**
 * Sets SEO metadata for the current page.
 *
 * Pushes tags via Nuxt's `useHead()` when available; always returns the
 * built payload.
 *
 * @param input - Entity, route, and optional per-call config.
 * @returns The resolved SEO payload.
 */
export declare function useEaseoSeo(input: {
  /** Content entity for the page. */
  entity: SEOEntity;
  /** Route path for the page. */
  route: string;
  /** Overrides the module config for this call. */
  config?: SEOConfig;
}): SEOPayload;

export default easeoModule;
