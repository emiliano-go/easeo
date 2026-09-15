import type { SEOConfig } from "@easeo/core";
export type { SEOConfig } from "@easeo/core";

export interface EaseoViteConfig {
  config: SEOConfig;
}

export interface EaseoVitePlugin {
  name: string;
  transformIndexHtml(
    html: string,
    ctx: { path: string }
  ): Array<{ tag: string; attrs?: Record<string, string>; children?: string }>;
}

declare function easeo(options: EaseoViteConfig): EaseoVitePlugin;

export default easeo;
