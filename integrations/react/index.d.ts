import type { SEOConfig, SEOEntity } from "@easeo/core";
export type { SEOConfig, SEOEntity } from "@easeo/core";

export interface EaseoHeadProps {
  entity: SEOEntity;
  route: string;
  config: SEOConfig;
}

/**
 * React component for injecting SEO meta tags into <head>.
 * SSR-safe: returns null when `document` is unavailable.
 */
export declare function EaseoHead(props: EaseoHeadProps): null;

export default EaseoHead;
