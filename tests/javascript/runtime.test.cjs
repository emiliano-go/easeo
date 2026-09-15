// Runtime tests for @easeo/core against the built native module.
// Skips cleanly on a bare checkout (native module not built yet);
// runs in CI after `napi build` places easeo.node in packages/core/.

const { describe, it } = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");

const pkgDir = path.join(__dirname, "..", "..", "packages", "core");
const nativePath = path.join(pkgDir, "easeo.node");
const hasNative = fs.existsSync(nativePath);
const skipOpts = hasNative ? {} : { skip: "native module not built (packages/core/easeo.node missing)" };

const config = { canonicalHost: "example.com", publicBaseUrl: "https://example.com" };

function loadCore() {
  return require(pkgDir);
}

describe("@easeo/core runtime", skipOpts, () => {
  it("payload exposes data properties and methods", () => {
    const { buildSeoPayload } = loadCore();
    const payload = buildSeoPayload(
      { entityType: "post", title: "Hello World", description: "An example post." },
      "/blog/hello",
      config
    );
    assert.equal(payload.canonical, "https://example.com/blog/hello");
    assert.equal(payload.title, "Hello World");
    assert.equal(payload.openGraph.type, "article");
    assert.equal(payload.twitter.card, "summary_large_image");
    for (const m of ["renderHtml", "renderOpengraph", "renderTwitter", "renderJsonld", "toObject", "toJSON", "hash", "etag"]) {
      assert.equal(typeof payload[m], "function", `${m} must be a function`);
    }
    assert.match(payload.hash(), /^[0-9a-f]{64}$/);
    assert.match(payload.etag(), /^"[0-9a-f]{64}"$/);
    assert(payload.renderHtml().includes('rel="canonical"'));
  });

  it("is deterministic", () => {
    const { buildSeoPayload } = loadCore();
    const entity = { entityType: "page", title: "Stable" };
    assert.equal(
      buildSeoPayload(entity, "/x", config).hash(),
      buildSeoPayload(entity, "/x", config).hash()
    );
  });

  it("contract exceptions survive the wrapper translation", () => {
    const { buildSeoContract } = loadCore();
    const contract = buildSeoContract({
      canonicalHost: "example.com",
      exceptions: { "/tmp/*": { indexable: false } },
    });
    const parsed = JSON.parse(contract.toJSON());
    assert.equal(Object.keys(parsed.exceptions).length, 1);
    assert.equal(typeof contract.toJSON, "function");
    assert.equal(contract.toJson, undefined);
  });

  it("cleanUrl returns plain objects, not JSON strings", () => {
    const { cleanUrl } = loadCore();
    const result = cleanUrl("https://example.com/p?utm_source=x&keep=2");
    assert.equal(result.url, "https://example.com/p?keep=2");
    assert.deepEqual(result.removedParams, { utm_source: "x" });
    assert.deepEqual(result.cleanedParams, { keep: "2" });
  });

  it("rejects invalid config with a useful error", () => {
    const { buildSeoPayload } = loadCore();
    assert.throws(
      () => buildSeoPayload({ entityType: "page", title: "T" }, "/x", { canonicalHost: "", publicBaseUrl: "" }),
      /canonical_host must be a non-empty string/
    );
  });

  it("SchemaRegistry.register throws with guidance (Rust-only)", () => {
    const { getSchemaRegistry } = loadCore();
    assert.throws(() => getSchemaRegistry().register("Podcast"), /Rust-only.*schemaJsonLd/s);
    assert.equal(getSchemaRegistry().has("Podcast"), false);
  });

  it("validatePayload returns SEOIssue objects", () => {
    const { buildSeoPayload, validatePayload } = loadCore();
    const payload = buildSeoPayload({ entityType: "page", title: "A".repeat(70) }, "/x", config);
    const issues = validatePayload(payload);
    assert(Array.isArray(issues));
    if (issues.length > 0) {
      assert.equal(typeof issues[0].ruleId, "string");
      assert.equal(typeof issues[0].details, "object");
    }
  });
});
