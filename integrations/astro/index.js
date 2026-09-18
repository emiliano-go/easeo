// @easeo/astro — Astro integration for easeo
// Build-time SEO contract emission + config injection.

const { buildSeoContract } = require("@easeo/core");
const { writeFileSync, mkdirSync, existsSync } = require("fs");
const { join } = require("path");
const { fileURLToPath } = require("url");

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
        const outDir = fileURLToPath(dir);
        const contract = buildSeoContract(contractConfig);
        const contractDir = join(outDir, ".easeo");
        if (!existsSync(contractDir)) {
          mkdirSync(contractDir, { recursive: true });
        }
        // Emit the canonical snake_case wire format so the artifact matches
        // the published seo-contract JSON schema.
        writeFileSync(
          join(contractDir, "contract.json"),
          JSON.stringify(contract.toDict(), null, 2)
        );
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
