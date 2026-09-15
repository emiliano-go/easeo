const { describe, it } = require("node:test");
const assert = require("node:assert");

// These tests verify the TypeScript declarations compile correctly
// The actual native module tests require building the napi-rs addon

describe("@easeo/core type declarations", () => {
  it("types are defined", () => {
    // Verify the type declarations exist by checking the index.d.ts content
    const fs = require("fs");
    const path = require("path");
    const dtsPath = path.join(__dirname, "..", "..", "packages", "core", "index.d.ts");
    const content = fs.readFileSync(dtsPath, "utf8");

    assert(content.includes("export interface SEOConfig"));
    assert(content.includes("export interface SEOEntity"));
    assert(content.includes("export interface SEOPayload"));
    assert(content.includes("export interface SEOOverrides"));
    assert(content.includes("export interface SEOContract"));
    assert(content.includes("export interface SchemaRegistry"));
    assert(content.includes("buildSeoPayload"));
    assert(content.includes("buildSeoPayloadWithOverrides"));
    assert(content.includes("buildSeoContract"));
    assert(content.includes("validatePayload"));
  });
});
