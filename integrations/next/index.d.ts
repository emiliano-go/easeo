import type { SEOConfig, SEOEntity } from "@easeo/core";
export type { SEOConfig, SEOEntity } from "@easeo/core";

export interface EaseoMetadataInput {
  entity: SEOEntity;
  route: string;
  /** Required: the core rejects an empty canonicalHost at build time. */
  config: SEOConfig;
}

/** Convert an easeo payload to a Next.js Metadata object. */
export declare function easeoMetadata(input: EaseoMetadataInput): Record<string, unknown>;

export default easeoMetadata;
