import type { SEOConfig, SEOEntity } from "@easeo/core";
export type { SEOConfig, SEOEntity } from "@easeo/core";

/** Input accepted by {@link easeoMetadata}. */
export interface EaseoMetadataInput {
  /** Content entity for the page. */
  entity: SEOEntity;
  /** Route path for the page. */
  route: string;
  /** Required: the core rejects an empty canonicalHost at build time. */
  config: SEOConfig;
}

/**
 * Converts an easeo payload to a Next.js Metadata object.
 *
 * @param input - Entity, route, and config.
 * @returns A Next.js Metadata object for `generateMetadata()`.
 */
export declare function easeoMetadata(input: EaseoMetadataInput): Record<string, unknown>;

export default easeoMetadata;
