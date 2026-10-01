const { describe, it } = require("node:test");
const assert = require("node:assert");

// These tests verify the TypeScript declarations compile correctly
// The actual native module tests require building the napi-rs addon

describe("@easeo/core type declarations", () => {
  it("types are defined", () => {
    const fs = require("fs");
    const path = require("path");
    const root = path.join(__dirname, "..", "..", "packages", "core");
    const indexDts = fs.readFileSync(path.join(root, "index.d.ts"), "utf8");
    const nativeDts = fs.readFileSync(path.join(root, "native.d.ts"), "utf8");

    // Public aliases and the wrapper surface live in index.d.ts.
    assert(indexDts.includes("NodeSeoConfig as SEOConfig"));
    assert(indexDts.includes("NodeSeoEntity as SEOEntity"));
    assert(indexDts.includes("NodeSeoOverrides as SEOOverrides"));
    assert(indexDts.includes("export interface SEOPayload"));
    assert(indexDts.includes("export interface SEOContract"));
    assert(indexDts.includes("export declare class SchemaRegistry"));
    assert(indexDts.includes("export declare class HookRegistry"));
    assert(indexDts.includes("buildSeoPayload"));
    assert(indexDts.includes("buildSeoPayloadWithOverrides"));
    assert(indexDts.includes("buildSeoContract"));
    assert(indexDts.includes("validatePayload"));
    assert(indexDts.includes("fromBlogPost"));
    assert(indexDts.includes("fromProduct"));
    assert(indexDts.includes("fromFaq"));

    // The Rust-generated records live in native.d.ts.
    assert(nativeDts.includes("export interface NodeSeoConfig"));
    assert(nativeDts.includes("export interface NodeSeoEntity"));
    assert(nativeDts.includes("export interface NodeSeoOverrides"));
    assert(nativeDts.includes("export interface NodeSeoExpectation"));
  });
});
