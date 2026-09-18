// Regression tests for the framework integrations.
// These exercise the plain-JS integration packages against a built core.
// Skips cleanly when the native module is not built.

const { describe, it } = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const os = require("os");
const path = require("path");

const root = path.join(__dirname, "..", "..");
const coreDir = path.join(root, "packages", "core");
const hasNative = fs.existsSync(coreDir) && fs.readdirSync(coreDir).some((f) => f.endsWith(".node"));
const skipOpts = hasNative ? {} : { skip: "native module not built (no *.node in packages/core/)" };

const config = { canonicalHost: "example.com", publicBaseUrl: "https://example.com" };

// Integrations `require("@easeo/core")`, so build a throwaway node_modules
// layout that mirrors a real install. Node resolves symlinks to their real
// path, so copy the packages rather than linking them.
const linkDir = fs.mkdtempSync(path.join(os.tmpdir(), "easeo-links-"));
const scopeDir = path.join(linkDir, "node_modules", "@easeo");
fs.mkdirSync(scopeDir, { recursive: true });
fs.cpSync(coreDir, path.join(scopeDir, "core"), { recursive: true });
for (const name of ["astro", "next", "nuxt", "react", "sveltekit", "vite"]) {
  fs.cpSync(path.join(root, "integrations", name), path.join(scopeDir, name), {
    recursive: true,
  });
}

function integration(name) {
  return require(path.join(scopeDir, name));
}

describe("framework integrations", skipOpts, () => {
  it("vite does not strip a non-index 'index' suffix", () => {
    const easeo = integration("vite");
    const plugin = easeo({ config });
    const canon = plugin
      .transformIndexHtml("", { path: "/reindex" })
      .find((t) => t.tag === "link");
    assert.equal(canon.attrs.href, "https://example.com/reindex");
  });

  it("vite escapes </script> in JSON-LD", () => {
    const easeo = integration("vite");
    const plugin = easeo({ config: { ...config, siteName: "</script><img src=x>" } });
    const script = plugin
      .transformIndexHtml("", { path: "/x" })
      .find((t) => t.tag === "script");
    assert(script, "expected a JSON-LD script tag");
    assert(!script.children.includes("</script>"), "raw closing script tag leaked");
  });

  it("astro writes the contract to a decoded output path", () => {
    const easeo = integration("astro");
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "easeo space "));
    const instance = easeo({
      config,
      contract: { canonicalHost: "example.com", scheme: "https" },
    });
    instance.hooks["astro:build:done"]({ dir: new URL(`file://${dir}/`) });
    assert(fs.existsSync(path.join(dir, ".easeo", "contract.json")));
  });

  it("astro registers config:setup inside hooks", () => {
    const easeo = integration("astro");
    const instance = easeo({ config });
    assert.equal(typeof instance.hooks["astro:config:setup"], "function");
  });

  it("next supports default and named ESM-style exports", () => {
    const mod = integration("next");
    assert.equal(typeof mod, "function");
    assert.equal(typeof mod.easeoMetadata, "function");
    assert.equal(typeof mod.default, "function");
  });

  it("react supports default and named ESM-style exports", () => {
    const mod = integration("react");
    assert.equal(typeof mod, "function");
    assert.equal(typeof mod.EaseoHead, "function");
    assert.equal(typeof mod.default, "function");
  });

  it("nuxt supports default and named ESM-style exports", () => {
    const mod = integration("nuxt");
    assert.equal(typeof mod, "function");
    assert.equal(typeof mod.useEaseoSeo, "function");
    assert.equal(typeof mod.default, "function");
  });

  it("sveltekit escapes < in the JSON-LD block before {@html}", () => {
    const source = fs.readFileSync(
      path.join(root, "integrations", "sveltekit", "EaseoHead.svelte"),
      "utf8"
    );
    assert(source.includes("u003c"), "JSON-LD must be escaped before {@html}");
  });
});
