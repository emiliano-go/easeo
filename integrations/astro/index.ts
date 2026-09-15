// @easeo/astro — Astro integration for easeo
import { buildSeoContract, type SEOConfig, type SEOContractConfig } from "@easeo/core";
import { writeFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";

export interface EaseoAstroConfig {
  config: SEOConfig;
  contract?: SEOContractConfig;
}

export default function easeo(options: EaseoAstroConfig) {
  const { config: siteConfig, contract: contractConfig } = options;

  return {
    name: "@easeo/astro",
    hooks: {
      "astro:build:done"({ dir }: { dir: URL }) {
        const outDir = dir.pathname;
        // Emit contract if configured
        if (contractConfig) {
          const contract = buildSeoContract(contractConfig);
          const contractDir = join(outDir, ".easeo");
          if (!existsSync(contractDir)) {
            mkdirSync(contractDir, { recursive: true });
          }
          writeFileSync(join(contractDir, "contract.json"), contract.toJson());
        }
      },
      "astro:config:setup"({ updateConfig }: { updateConfig: (config: Record<string, unknown>) => void }) {
        // Inject SEO config into Astro's global scope
        updateConfig({
          vite: {
            define: {
              "__EASEO_CONFIG__": JSON.stringify(siteConfig),
            },
          },
        });
      },
    },
  };
}
