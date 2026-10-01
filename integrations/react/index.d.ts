import type { SEOConfig, SEOEntity } from "@easeo/core";
export type { SEOConfig, SEOEntity } from "@easeo/core";

/** Props accepted by the `EaseoHead` component. */
export interface EaseoHeadProps {
  /** Content entity for the page. */
  entity: SEOEntity;
  /** Route path for the page. */
  route: string;
  /** Site-wide easeo configuration. */
  config: SEOConfig;
}

/**
 * React component for injecting SEO meta tags into `<head>`.
 *
 * SSR-safe: returns null when `document` is unavailable.
 *
 * @param props - Entity, route, and config.
 * @returns Always null; the component only updates `document.head`.
 */
export declare function EaseoHead(props: EaseoHeadProps): null;

export default EaseoHead;
