import type { SEOConfig, SEOContractConfig } from "@easeo/core";
export type { SEOConfig, SEOContractConfig } from "@easeo/core";

export interface EaseoAstroConfig {
  config: SEOConfig;
  /** When set, a machine-readable contract is written to <outDir>/.easeo/contract.json after build. */
  contract?: SEOContractConfig;
}

export interface EaseoAstroIntegration {
  name: string;
  hooks: Record<string, (...args: any[]) => void>;
}

declare function easeo(options: EaseoAstroConfig): EaseoAstroIntegration;

export default easeo;
