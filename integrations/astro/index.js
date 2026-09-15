// @easeo/astro — Astro integration for easeo
// Build-time SEO contract emission + config injection.

const { buildSeoContract } = require("@easeo/core");
const { writeFileSync, mkdirSync, existsSync } = require("fs");
const { join } = require("path");

/**
 * @typedef {Object} EaseoAstroConfig
 * @property {import("./index.d").SEOConfig} config - site-wide easeo config
 * @property {import("./index.d").SEOContractConfig} [contract] - emit .easeo/contract.json on build done
 */

/**
 * Astro integration for easeo.
 *
 * @param {EaseoAstroConfig} options
 * @returns {{ name: string, hooks: Record<string, Function> }}
 *
 * @example
 * // astro.config.mjs
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
function easeo(options) {
  const { config: siteConfig, contract: contractConfig } = options;

  return {
    name: "@easeo/astro",
    hooks: {
      "astro:build:done"({ dir }) {
        if (!contractConfig) return;
        const outDir = dir.pathname;
        const contract = buildSeoContract(contractConfig);
        const contractDir = join(outDir, ".easeo");
        if (!existsSync(contractDir)) {
          mkdirSync(contractDir, { recursive: true });
        }
        writeFileSync(join(contractDir, "contract.json"), contract.toJSON());
      },
      "astro:config:setup"({ updateConfig }) {
        updateConfig({
          vite: {
            define: {
              __EASEO_CONFIG__: JSON.stringify(siteConfig),
            },
          },
        });
      },
    },
  };
}

module.exports = easeo;
module.exports.default = easeo;
