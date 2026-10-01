import type { SEOConfig } from "@easeo/core";
export type { SEOConfig } from "@easeo/core";

/** Options accepted by the easeo Vite plugin. */
export interface EaseoViteConfig {
  /** Site-wide easeo configuration. */
  config: SEOConfig;
}

/** Vite plugin shape returned by the easeo plugin factory. */
export interface EaseoVitePlugin {
  /** Plugin name. */
  name: string;
  /** Injects SEO tags into the transformed HTML. */
  transformIndexHtml(
    html: string,
    ctx: { path: string }
  ): Array<{ tag: string; attrs?: Record<string, string>; children?: string }>;
}

/**
 * Vite plugin that injects SEO meta tags through `transformIndexHtml`.
 *
 * @example
 * import easeo from "@easeo/vite";
 *
 * export default defineConfig({
 *   plugins: [
 *     easeo({
 *       config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
 *     }),
 *   ],
 * });
 */
declare function easeo(options: EaseoViteConfig): EaseoVitePlugin;

export default easeo;
