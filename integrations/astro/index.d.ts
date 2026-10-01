import type { SEOConfig, SEOContractConfig } from "@easeo/core";
export type { SEOConfig, SEOContractConfig } from "@easeo/core";

/** Options accepted by the easeo Astro integration. */
export interface EaseoAstroConfig {
  /** Site-wide easeo configuration. */
  config: SEOConfig;
  /** When set, a machine-readable contract is written to `<outDir>/.easeo/contract.json` after build. */
  contract?: SEOContractConfig;
}

/** Astro integration shape returned by the easeo integration factory. */
export interface EaseoAstroIntegration {
  /** Integration name. */
  name: string;
  /** Astro lifecycle hooks. */
  hooks: Record<string, (...args: any[]) => void>;
}

/**
 * Astro integration that injects the easeo config and can emit an SEO
 * contract at build time.
 *
 * @example
 * import easeo from "@easeo/astro";
 *
 * export default defineConfig({
 *   integrations: [
 *     easeo({
 *       config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
 *     }),
 *   ],
 * });
 */
declare function easeo(options: EaseoAstroConfig): EaseoAstroIntegration;

export default easeo;
