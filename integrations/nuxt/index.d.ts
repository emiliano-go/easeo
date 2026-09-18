import type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";
export type { SEOConfig, SEOEntity, SEOPayload } from "@easeo/core";

export interface EaseoNuxtConfig {
  config: SEOConfig;
}

/** Lightweight Nuxt module: stores site-wide config for useEaseoSeo(). */
export declare function easeoModule(options: EaseoNuxtConfig): { name: string };

/**
 * Set SEO metadata for the current page.
 * Pushes tags via Nuxt's useHead() when available; always returns the payload.
 */
export declare function useEaseoSeo(input: {
  entity: SEOEntity;
  route: string;
  config?: SEOConfig;
}): SEOPayload;

export default easeoModule;
