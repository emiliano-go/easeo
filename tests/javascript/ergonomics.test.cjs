// Tests for the JS ergonomics layer: payload equality/lookup, hooks,
// schema registry, factories, and the optional-overrides signature.

const { describe, it } = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");

const pkgDir = path.join(__dirname, "..", "..", "packages", "core");
const hasNative = fs.existsSync(pkgDir) && fs.readdirSync(pkgDir).some((f) => f.endsWith(".node"));
const skipOpts = hasNative ? {} : { skip: "native module not built (no *.node in packages/core/)" };

const config = { canonicalHost: "example.com", publicBaseUrl: "https://example.com" };
const entity = { entityType: "post", title: "Hello", description: "World" };

function core() {
  return require(pkgDir);
}

describe("@easeo/core ergonomics", skipOpts, () => {
  it("payload.equals compares payloads and plain objects", () => {
    const { buildSeoPayload } = core();
    const a = buildSeoPayload(entity, "/x", config);
    const b = buildSeoPayload(entity, "/x", config);
    assert.equal(a.equals(b), true);
    assert.equal(a.equals(a.toObject()), true);
    assert.equal(a.equals({ nope: true }), false);
  });

  it("payload.get and has", () => {
    const { buildSeoPayload } = core();
    const p = buildSeoPayload(entity, "/x", config);
    assert.equal(p.get("title"), "Hello");
    assert.equal(p.get("nope", "fallback"), "fallback");
    assert.equal(p.get("nope"), undefined);
    assert.equal(p.has("title"), true);
    assert.equal(p.has("nope"), false);
  });

  it("buildSeoPayload accepts optional overrides", () => {
    const { buildSeoPayload } = core();
    const p = buildSeoPayload(entity, "/x", config, { metaTitle: "Overridden" });
    assert.equal(p.title, "Overridden");
  });

  it("hooks mutate the payload", () => {
    const { buildSeoPayload, HookRegistry } = core();
    const hooks = new HookRegistry();
    hooks.register("post_process", (payload) => {
      payload.generator = "easeo";
      return payload;
    });
    const p = buildSeoPayload(entity, "/x", { ...config, hooks });
    assert.equal(p.get("generator"), "easeo");
    assert.equal(p.toDict().generator, "easeo");
    assert.equal({ ...p }.generator, "easeo");
  });

  it("hooks are scoped to their config", () => {
    const { buildSeoPayload, HookRegistry } = core();
    const hooks = new HookRegistry();
    hooks.register("post_process", (p) => ({ ...p, site: "A" }));
    assert.equal(buildSeoPayload(entity, "/x", { ...config, hooks }).get("site"), "A");
    assert.equal(buildSeoPayload(entity, "/x", config).get("site"), undefined);
  });

  it("hooks are deterministic", () => {
    const { buildSeoPayload, HookRegistry } = core();
    const hooks = new HookRegistry();
    hooks.register("post_process", (p) => ({ ...p, tag: "x" }));
    const cfg = { ...config, hooks };
    assert.equal(
      buildSeoPayload(entity, "/x", cfg).hash(),
      buildSeoPayload(entity, "/x", cfg).hash()
    );
  });

  it("HookRegistry supports decorator form and clear", () => {
    const { HookRegistry } = core();
    const hooks = new HookRegistry();
    const fn = hooks.hook("post_process")((p) => p);
    assert.equal(hooks.size(), 1);
    hooks.unregister("post_process", fn);
    assert.equal(hooks.size(), 0);
    hooks.register("post_process", (p) => p);
    hooks.clear("post_process");
    assert.equal(hooks.size(), 0);
  });

  it("schema registry replaces the built-in schema", () => {
    const { buildSeoPayload, SchemaRegistry } = core();
    const registry = new SchemaRegistry();
    registry.register("Article", (e, c, canonical, title) => ({
      "@context": "https://schema.org",
      "@type": "PodcastEpisode",
      name: title,
    }));
    const p = buildSeoPayload(entity, "/x", { ...config, schemaRegistry: registry });
    assert.equal(p.toDict().schema_jsonld["@type"], "PodcastEpisode");
    // Without the registry the built-in schema is used.
    assert.equal(buildSeoPayload(entity, "/x", config).toDict().schema_jsonld["@type"], "Article");
  });

  it("schema registry unregister restores built-in", () => {
    const { buildSeoPayload, SchemaRegistry } = core();
    const registry = new SchemaRegistry();
    registry.register("Article", () => ({ "@type": "Custom" }));
    registry.unregister("Article");
    const p = buildSeoPayload(entity, "/x", { ...config, schemaRegistry: registry });
    assert.equal(p.toDict().schema_jsonld["@type"], "Article");
  });

  it("factories build valid entities", () => {
    const { buildSeoPayload, fromBlogPost, fromProduct, fromFaq } = core();
    const entities = [
      fromBlogPost({ title: "T", bodyHtml: "<p>B</p>" }),
      fromProduct({ name: "W", sku: "W-1", price: 9.99 }),
      fromFaq({ questions: [{ question: "Q", answer: "A" }] }),
    ];
    for (const e of entities) {
      assert.equal(buildSeoPayload(e, "/x", config).canonical, "https://example.com/x");
    }
  });

  it("errors are typed", () => {
    const { buildSeoPayload, ConfigurationError, EaseoError } = core();
    assert.throws(
      () => buildSeoPayload({ entityType: "page" }, "/x", { canonicalHost: "", publicBaseUrl: "" }),
      (err) => err instanceof ConfigurationError && err instanceof EaseoError
    );
  });
});
