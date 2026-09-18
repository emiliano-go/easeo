// Runtime tests for @easeo/core against the built native module.
// Skips cleanly on a bare checkout (native module not built yet);
// runs in CI after `napi build` places easeo.node in packages/core/.

const { describe, it } = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");

const pkgDir = path.join(__dirname, "..", "..", "packages", "core");
const hasNative = fs.existsSync(pkgDir) && fs.readdirSync(pkgDir).some((f) => f.endsWith(".node"));
const skipOpts = hasNative ? {} : { skip: "native module not built (no *.node in packages/core/)" };

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
    for (const m of ["renderHtml", "renderOpengraph", "renderTwitter", "renderJsonld", "toObject", "toJSON", "toDict", "toJSONString", "hash", "etag"]) {
      assert.equal(typeof payload[m], "function", `${m} must be a function`);
    }
    assert.match(payload.hash(), /^[0-9a-f]{64}$/);
    assert.match(payload.etag(), /^"[0-9a-f]{64}"$/);
    assert(payload.renderHtml().includes('rel="canonical"'));
  });

  it("exposes enumerable camelCase data and hides methods", () => {
    const { buildSeoPayload } = loadCore();
    const payload = buildSeoPayload(
      { entityType: "post", title: "Hello", description: "D" },
      "/x",
      config
    );
    assert.deepEqual(Object.keys(payload).sort(), [
      "canonical",
      "description",
      "openGraph",
      "robots",
      "schemaJsonLd",
      "title",
      "twitter",
    ]);
    assert(!Object.keys(payload).includes("renderHtml"), "methods must not be enumerable");
    assert.deepEqual({ ...payload }.openGraph, payload.openGraph);
  });

  it("JSON.stringify(payload) produces an object, not a double-encoded string", () => {
    const { buildSeoPayload } = loadCore();
    const payload = buildSeoPayload(
      { entityType: "post", title: "Hello", description: "D" },
      "/x",
      config
    );
    const parsed = JSON.parse(JSON.stringify(payload));
    assert.equal(typeof parsed, "object");
    assert.equal(parsed.openGraph.type, "article");
    assert.equal(parsed.schemaJsonLd["@type"], "Article");
  });

  it("toObject returns camelCase and toDict returns the snake_case wire format", () => {
    const { buildSeoPayload } = loadCore();
    const payload = buildSeoPayload(
      { entityType: "post", title: "Hello", description: "D" },
      "/x",
      config
    );
    assert(payload.toObject().openGraph, "toObject must use camelCase openGraph");
    assert(payload.toDict().og, "toDict must use the canonical snake_case og");
    assert.equal(typeof payload.toJSONString(), "string");
    assert.deepEqual(JSON.parse(payload.toJSONString()), payload.toDict());
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
    assert.equal(Object.keys(contract.toObject().exceptions).length, 1);
    assert.equal(typeof contract.toJSON, "function");
    assert.equal(contract.toJson, undefined);
    const roundTripped = JSON.parse(JSON.stringify(contract));
    assert.equal(roundTripped.generator.name, "easeo");
    assert.equal(Object.keys(roundTripped.exceptions).length, 1);
    assert.equal(contract.toDict().generator.name, "easeo");
  });

  it("cleanUrl returns plain objects, not JSON strings", () => {
    const { cleanUrl } = loadCore();
    const result = cleanUrl("https://example.com/p?utm_source=x&keep=2");
    assert.equal(result.url, "https://example.com/p?keep=2");
    assert.deepEqual(result.removedParams, { utm_source: "x" });
    assert.deepEqual(result.cleanedParams, { keep: "2" });
  });

  it("rejects invalid config with a typed error", () => {
    const { buildSeoPayload, ConfigurationError, EaseoError } = loadCore();
    assert.throws(
      () => buildSeoPayload({ entityType: "page", title: "T" }, "/x", { canonicalHost: "", publicBaseUrl: "" }),
      (err) => {
        assert(err instanceof ConfigurationError, "expected ConfigurationError");
        assert(err instanceof EaseoError, "expected EaseoError base");
        assert.equal(err.code, "EASEO_CONFIGURATION");
        assert.match(err.message, /canonical_host must be a non-empty string/);
        return true;
      }
    );
  });

  it("maps invalid entity types to EntityError", () => {
    const { buildSeoPayload, EntityError } = loadCore();
    assert.throws(
      () => buildSeoPayload({ entityType: "bogus", title: "T" }, "/x", config),
      (err) => err instanceof EntityError
    );
  });

  it("guards missing arguments with TypeError", () => {
    const { normalizePath, cleanQuery, buildSeoPayload } = loadCore();
    assert.throws(() => normalizePath(undefined), TypeError);
    assert.throws(() => cleanQuery(undefined), TypeError);
    assert.throws(() => buildSeoPayload(null, "/x", config), TypeError);
    assert.throws(() => buildSeoPayload({ entityType: "page" }, 123, config), TypeError);
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
