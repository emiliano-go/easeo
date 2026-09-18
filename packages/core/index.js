// @easeo/core - JavaScript bindings for easeo
//
// Loads the napi-rs native module and wraps its result objects so that the
// JavaScript surface follows JS conventions:
//
//   * payload/contract data lives in enumerable camelCase properties, so
//     `Object.keys`, spread, and the TypeScript types all agree;
//   * methods are non-enumerable, so they never leak into serialized data;
//   * `toObject()` / `toJSON()` return a plain camelCase object, so
//     `JSON.stringify(payload)` produces an object (not a double-encoded
//     string) and integrates with `res.json()` and friends;
//   * `toDict()` returns the canonical snake_case wire format shared with the
//     Python/Rust APIs and the published JSON schemas;
//   * `toJSONString()` / `toString()` return that canonical JSON string;
//   * errors are real `EaseoError` subclasses, not generic `Error`s.

"use strict";

// ── Native module loading ─────────────────────────────────────────────
// The published package bundles one `easeo.<platform>-<arch>[-<libc>].node`
// per supported platform; local development builds may use the unsuffixed
// `easeo.node`.

function isMusl() {
  if (!process.report || typeof process.report.getReport !== "function") {
    return false;
  }
  try {
    return !process.report.getReport().header.glibcVersionRuntime;
  } catch {
    return false;
  }
}

function bindingCandidates() {
  const { platform, arch } = process;
  const candidates = [];
  if (platform === "linux") {
    candidates.push(`./easeo.linux-${arch}-${isMusl() ? "musl" : "gnu"}.node`);
  } else if (platform === "darwin") {
    candidates.push(`./easeo.darwin-${arch}.node`);
  } else if (platform === "win32") {
    candidates.push(`./easeo.win32-${arch}-msvc.node`);
  }
  candidates.push("./easeo.node");
  return candidates;
}

function loadNative() {
  const tried = [];
  for (const candidate of bindingCandidates()) {
    tried.push(candidate);
    try {
      return require(candidate);
    } catch {
      // try the next candidate
    }
  }
  throw new Error(
    `@easeo/core: Native bindings not available for ${process.platform}-${process.arch}. ` +
      `Tried: ${tried.join(", ")}. Run \`napi build --release\` first, ` +
      "or reinstall @easeo/core."
  );
}

const native = loadNative();

// ── Error model ───────────────────────────────────────────────────────
// Mirrors the Rust/Python hierarchy so `instanceof` works for consumers.

class EaseoError extends Error {
  constructor(message, options) {
    super(message, options);
    this.name = "EaseoError";
    this.code = "EASEO_ERROR";
  }
}

class InvalidUrlError extends EaseoError {
  constructor(message, options) {
    super(message, options);
    this.name = "InvalidUrlError";
    this.code = "EASEO_INVALID_URL";
  }
}

class ConfigurationError extends EaseoError {
  constructor(message, options) {
    super(message, options);
    this.name = "ConfigurationError";
    this.code = "EASEO_CONFIGURATION";
  }
}

class EntityError extends EaseoError {
  constructor(message, options) {
    super(message, options);
    this.name = "EntityError";
    this.code = "EASEO_ENTITY";
  }
}

class SchemaError extends EaseoError {
  constructor(message, options) {
    super(message, options);
    this.name = "SchemaError";
    this.code = "EASEO_SCHEMA";
  }
}

class ContractError extends EaseoError {
  constructor(message, options) {
    super(message, options);
    this.name = "ContractError";
    this.code = "EASEO_CONTRACT";
  }
}

const ERROR_PREFIXES = [
  ["invalid URL:", InvalidUrlError],
  ["URL policy error:", InvalidUrlError],
  ["invalid configuration:", ConfigurationError],
  ["invalid entity:", EntityError],
  ["invalid schema:", SchemaError],
  ["contract error:", ContractError],
  ["serialization error:", EaseoError],
];

function mapNativeError(err) {
  if (err instanceof EaseoError) return err;
  const message = err && typeof err.message === "string" ? err.message : String(err);
  for (const [prefix, ErrorClass] of ERROR_PREFIXES) {
    if (message.startsWith(prefix)) {
      return new ErrorClass(message.slice(prefix.length).trim(), { cause: err });
    }
  }
  return err;
}

function callNative(fn) {
  try {
    return fn();
  } catch (err) {
    throw mapNativeError(err);
  }
}

// ── Argument guards ───────────────────────────────────────────────────

function describe(value) {
  if (value === null) return "null";
  if (Array.isArray(value)) return "an array";
  return `type ${typeof value}`;
}

function assertString(value, name, fnName) {
  if (typeof value !== "string") {
    throw new TypeError(
      `${fnName}: expected '${name}' to be a string, received ${describe(value)}`
    );
  }
}

function assertObject(value, name, fnName) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `${fnName}: expected '${name}' to be an object, received ${describe(value)}`
    );
  }
}

// ── Result wrappers ───────────────────────────────────────────────────
// `nativeOf` maps a public wrapper back to its native object for the few
// functions that accept a payload (e.g. validatePayload).

const nativeOf = new WeakMap();

function camelPayload(n) {
  const og = n.openGraph || {};
  const tw = n.twitter || {};
  const out = {
    title: n.title,
    description: n.description,
    canonical: n.canonical,
    robots: n.robots,
    openGraph: {
      type: og.type,
      title: og.title,
      description: og.description,
      url: og.url,
      image: og.image,
      imageWidth: og.imageWidth,
      imageHeight: og.imageHeight,
      imageAlt: og.imageAlt,
      siteName: og.siteName,
      locale: og.locale,
      localeAlternate: og.localeAlternate,
      audio: og.audio,
      video: og.video,
    },
    twitter: {
      card: tw.card,
      title: tw.title,
      description: tw.description,
      image: tw.image,
      imageAlt: tw.imageAlt,
      site: tw.site,
      creator: tw.creator,
    },
  };
  const schema = n.schemaJsonLd;
  if (schema !== undefined && schema !== null) {
    out.schemaJsonLd = schema;
  }
  // Hook-added fields. Skipped for the fixed keys so they cannot shadow
  // the structural fields.
  const extra = n.extra;
  if (extra && typeof extra === "object") {
    for (const [key, value] of Object.entries(extra)) {
      if (!(key in out)) out[key] = value;
    }
  }
  return out;
}

function camelContract(n) {
  return {
    contractVersion: n.contractVersion,
    generator: { name: n.generatorName, version: n.generatorVersion },
    site: { canonicalHost: n.site.canonicalHost, scheme: n.site.scheme },
    defaults: n.defaults,
    rules: n.rules,
    exceptions: n.exceptions,
  };
}

function defineData(target, nativeObj, keys) {
  for (const key of keys) {
    Object.defineProperty(target, key, {
      enumerable: true,
      configurable: true,
      get: () => nativeObj[key],
    });
  }
}

function defineMethods(target, methods) {
  for (const [name, fn] of Object.entries(methods)) {
    Object.defineProperty(target, name, {
      enumerable: false,
      configurable: true,
      writable: true,
      value: fn,
    });
  }
}

function wrapPayload(nativePayload) {
  const wrapper = {};
  nativeOf.set(wrapper, nativePayload);

  defineData(wrapper, nativePayload, [
    "title",
    "description",
    "canonical",
    "robots",
    "openGraph",
    "twitter",
  ]);

  // schemaJsonLd is optional: only expose it when present, so
  // `Object.keys`, spread, and the optional TS type all agree.
  if (nativePayload.schemaJsonLd !== undefined && nativePayload.schemaJsonLd !== null) {
    defineData(wrapper, nativePayload, ["schemaJsonLd"]);
  }

  // Hook-added fields are enumerable too, so `{...payload}` includes them.
  const extra = nativePayload.extra;
  if (extra && typeof extra === "object") {
    for (const key of Object.keys(extra)) {
      if (!(key in wrapper)) {
        Object.defineProperty(wrapper, key, {
          enumerable: true,
          configurable: true,
          get: () => nativePayload.extra[key],
        });
      }
    }
  }

  defineMethods(wrapper, {
    renderHtml: () => callNative(() => nativePayload.renderHtml()),
    renderOpengraph: () => callNative(() => nativePayload.renderOpengraph()),
    renderTwitter: () => callNative(() => nativePayload.renderTwitter()),
    renderJsonld: () => callNative(() => nativePayload.renderJsonld()),
    // JS-idiomatic camelCase object; used by JSON.stringify.
    toObject: () => camelPayload(nativePayload),
    toJSON: () => camelPayload(nativePayload),
    // Canonical snake_case wire format (shared with Python/Rust).
    toDict: () => callNative(() => nativePayload.toObject()),
    toJSONString: () => callNative(() => nativePayload.toJSON()),
    toString: () => callNative(() => nativePayload.toJSON()),
    hash: () => callNative(() => nativePayload.hash()),
    etag: () => callNative(() => nativePayload.etag()),
    /** Look up a field with a default, dict-style. */
    get: (key, fallback) => {
      const data = wrapper.toObject();
      return Object.prototype.hasOwnProperty.call(data, key) ? data[key] : fallback;
    },
    /** Whether a field is present, dict-style. */
    has: (key) => Object.prototype.hasOwnProperty.call(wrapper.toObject(), key),
    /** Deep equality against another payload or a plain object. */
    equals: (other) => payloadsEqual(wrapper, other),
  });

  Object.defineProperty(wrapper, Symbol.toStringTag, {
    value: "SEOPayload",
    enumerable: false,
  });

  return wrapper;
}

function wrapContract(nativeContract) {
  const wrapper = {};
  nativeOf.set(wrapper, nativeContract);

  defineData(wrapper, nativeContract, [
    "contractVersion",
    "generatorName",
    "generatorVersion",
    "site",
    "defaults",
    "rules",
    "exceptions",
  ]);

  defineMethods(wrapper, {
    hash: () => callNative(() => nativeContract.hash()),
    toObject: () => camelContract(nativeContract),
    toJSON: () => camelContract(nativeContract),
    toDict: () => callNative(() => nativeContract.toDict()),
    toJSONString: () => callNative(() => nativeContract.toJSON()),
    toString: () => callNative(() => nativeContract.toJSON()),
  });

  Object.defineProperty(wrapper, Symbol.toStringTag, {
    value: "SEOContract",
    enumerable: false,
  });

  return wrapper;
}

// ── Config-scoped extensions (hooks + schema registry) ───────────────

function stableStringify(value) {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(stableStringify).join(",")}]`;
  const keys = Object.keys(value).sort();
  return `{${keys.map((k) => `${JSON.stringify(k)}:${stableStringify(value[k])}`).join(",")}}`;
}

function payloadsEqual(a, b) {
  // Normalize payload instances to their camelCase object form, so a
  // payload compares equal to `payload.toObject()` as well as to another
  // payload. Plain objects are used as-is.
  const normalize = (value) =>
    value && typeof value.toObject === "function" ? value.toObject() : value;
  const left = normalize(a);
  const right = normalize(b);
  if (left === right) return true;
  if (left === undefined || right === undefined) return false;
  return stableStringify(left) === stableStringify(right);
}

/**
 * Apply a config-scoped SchemaRegistry, then hooks, to a freshly built
 * payload. Both are plain JS concepts, so the payload round-trips through
 * its dict form. Determinism holds because both live on the config.
 */
function applyConfigExtensions(payloadDict, entity, config) {
  const registry = config.schemaRegistry;
  const hooks = config.hooks;
  if (!registry && !hooks) return payloadDict;

  let current = payloadDict;

  if (registry && typeof registry.generate === "function") {
    const schema = current.schema_jsonld;
    const schemaType = schema && !Array.isArray(schema) ? schema["@type"] : undefined;
    if (schemaType && typeof registry.has === "function" && registry.has(schemaType)) {
      const og = current.og || {};
      const generated = registry.generate(
        schemaType,
        entity,
        config,
        current.canonical,
        current.title,
        current.description,
        og.image
      );
      if (generated !== undefined && generated !== null) {
        current = { ...current, schema_jsonld: generated };
      }
    }
  }

  if (hooks && typeof hooks.run === "function") {
    const result = hooks.run("post_process", current, entity, config);
    if (result !== undefined && result !== null) {
      current = result;
    }
  }

  return current;
}

function finishPayload(nativePayload, entity, config) {
  const effectiveConfig = config || {};
  const hasExtensions = effectiveConfig.hooks || effectiveConfig.schemaRegistry;
  if (!hasExtensions) {
    return wrapPayload(nativePayload);
  }
  const dict = callNative(() => nativePayload.toObject());
  const extended = applyConfigExtensions(dict, entity, effectiveConfig);
  const rebuilt = callNative(() => native.payloadFromDict(extended));
  return wrapPayload(rebuilt);
}

// ── Public API ────────────────────────────────────────────────────────

/**
 * Build an SEO payload for a given entity and route.
 * @param {import('./index.d').SEOEntity} entity
 * @param {string} route
 * @param {import('./index.d').SEOConfig} config
 * @param {import('./index.d').SEOOverrides} [overrides]
 * @returns {import('./index.d').SEOPayload}
 */
function buildSeoPayload(entity, route, config, overrides) {
  assertObject(entity, "entity", "buildSeoPayload");
  assertString(route, "route", "buildSeoPayload");
  assertObject(config, "config", "buildSeoPayload");
  if (overrides !== undefined) {
    assertObject(overrides, "overrides", "buildSeoPayload");
    return buildSeoPayloadWithOverrides(entity, route, config, overrides);
  }
  const nativePayload = callNative(() =>
    native.buildSeoPayload(entity, route, config)
  );
  return finishPayload(nativePayload, entity, config);
}

/**
 * Build an SEO payload with overrides.
 * @param {import('./index.d').SEOEntity} entity
 * @param {string} route
 * @param {import('./index.d').SEOConfig} config
 * @param {import('./index.d').SEOOverrides} overrides
 * @returns {import('./index.d').SEOPayload}
 */
function buildSeoPayloadWithOverrides(entity, route, config, overrides) {
  assertObject(entity, "entity", "buildSeoPayloadWithOverrides");
  assertString(route, "route", "buildSeoPayloadWithOverrides");
  assertObject(config, "config", "buildSeoPayloadWithOverrides");
  assertObject(overrides, "overrides", "buildSeoPayloadWithOverrides");
  const nativePayload = callNative(() =>
    native.buildSeoPayloadWithOverrides(entity, route, config, overrides)
  );
  return finishPayload(nativePayload, entity, config);
}

/**
 * Build an SEO contract.
 * @param {import('./index.d').SEOContractConfig} config
 * @returns {import('./index.d').SEOContract}
 */
function buildSeoContract(config) {
  assertObject(config, "config", "buildSeoContract");
  const nativeConfig = {
    ...config,
    exceptionsJson: config.exceptions
      ? JSON.stringify(config.exceptions)
      : undefined,
  };
  delete nativeConfig.exceptions;
  return wrapContract(callNative(() => native.buildSeoContract(nativeConfig)));
}

/**
 * Validate an SEO payload against best practices.
 * @param {import('./index.d').SEOPayload} payload
 * @returns {import('./index.d').SEOIssue[]}
 */
function validatePayload(payload) {
  const nativePayload = nativeOf.get(payload) || payload;
  if (!nativePayload || typeof nativePayload !== "object") {
    throw new TypeError(
      `validatePayload: expected an easeo payload, received ${describe(payload)}`
    );
  }
  return callNative(() => native.validatePayload(nativePayload));
}

/**
 * Normalize a URL path.
 * @param {string} path
 * @param {object} [options]
 * @returns {string}
 */
function normalizePath(path, options) {
  assertString(path, "path", "normalizePath");
  if (options !== undefined) {
    assertObject(options, "options", "normalizePath");
  }
  return callNative(() =>
    native.normalizePath(
      path,
      options?.enforceHttps,
      options?.lowercasePaths,
      options?.trailingSlash,
      options?.collapseDuplicateSlashes,
      options?.stripTrackingParams,
      options?.allowedQueryParams
    )
  );
}

/**
 * Normalize a public URL.
 * @param {string} url
 * @param {import('./index.d').SEOConfig} config
 * @returns {string}
 */
function normalizePublicUrl(url, config) {
  assertString(url, "url", "normalizePublicUrl");
  assertObject(config, "config", "normalizePublicUrl");
  return callNative(() => native.normalizePublicUrl(url, config));
}

/**
 * Clean tracking parameters from a URL.
 * @param {string} url
 * @returns {{ url: string, removedParams: Record<string, string>, cleanedParams: Record<string, string> }}
 */
function cleanUrl(url) {
  assertString(url, "url", "cleanUrl");
  return callNative(() => native.cleanUrl(url));
}

/**
 * Clean tracking parameters from a query string.
 * @param {string} query
 * @returns {string}
 */
function cleanQuery(query) {
  assertString(query, "query", "cleanQuery");
  return callNative(() => native.cleanQuery(query));
}

/**
 * Get the Rust-backed schema registry (type-name introspection only).
 * For custom generators from JS, use the {@link SchemaRegistry} class.
 * @returns {object}
 */
function getSchemaRegistry() {
  return native.getSchemaRegistry();
}

// ── Config-scoped extension points ───────────────────────────────────

/**
 * Config-scoped hook registry. Attach to a config via
 * `{ hooks }` and hooks run after the payload is built.
 */
class HookRegistry {
  constructor() {
    this._hooks = new Map();
  }

  /** Register `fn` for the `name` hook point. */
  register(name, fn) {
    if (typeof name !== "string" || !name.trim()) {
      throw new TypeError("HookRegistry.register: name must be a non-empty string");
    }
    if (typeof fn !== "function") {
      throw new TypeError("HookRegistry.register: hook must be a function");
    }
    const list = this._hooks.get(name) || [];
    list.push(fn);
    this._hooks.set(name, list);
    return fn;
  }

  /** Decorator form: `hooks.hook("post_process")(fn)`. */
  hook(name) {
    return (fn) => this.register(name, fn);
  }

  unregister(name, fn) {
    const list = this._hooks.get(name);
    if (!list) return;
    const index = list.indexOf(fn);
    if (index >= 0) list.splice(index, 1);
  }

  run(name, payload, entity, config) {
    let current = payload;
    for (const fn of [...(this._hooks.get(name) || [])]) {
      current = fn(current, entity, config);
    }
    return current;
  }

  clear(name) {
    if (name === undefined) this._hooks.clear();
    else this._hooks.delete(name);
  }

  size() {
    let total = 0;
    for (const list of this._hooks.values()) total += list.length;
    return total;
  }
}

/**
 * Config-scoped registry of custom JSON-LD generators, keyed by schema
 * type. When the resolved schema `@type` matches, the generator's output
 * replaces the built-in schema.
 */
class SchemaRegistry {
  constructor() {
    this._generators = new Map();
  }

  register(schemaType, fn) {
    if (fn === undefined) {
      if (typeof schemaType !== "function") {
        throw new TypeError("SchemaRegistry.register: expected a type and a generator");
      }
      const name = schemaType.name;
      if (!name) {
        throw new TypeError("SchemaRegistry.register: anonymous function needs a type name");
      }
      this._generators.set(name, schemaType);
      return schemaType;
    }
    if (typeof schemaType !== "string" || !schemaType.trim()) {
      throw new TypeError("SchemaRegistry.register: schema type must be a non-empty string");
    }
    if (typeof fn !== "function") {
      throw new TypeError("SchemaRegistry.register: generator must be a function");
    }
    this._generators.set(schemaType.trim(), fn);
    return fn;
  }

  unregister(schemaType) {
    this._generators.delete(schemaType);
  }

  get(schemaType) {
    return this._generators.get(schemaType);
  }

  has(schemaType) {
    return this._generators.has(schemaType);
  }

  listTypes() {
    return [...this._generators.keys()].sort();
  }

  generate(schemaType, entity, config, canonical, title, description, ogImage) {
    const generator = this._generators.get(schemaType);
    if (!generator) return undefined;
    const result = generator(entity, config, canonical, title, description, ogImage);
    if (result === undefined || result === null) return undefined;
    if (typeof result !== "object") {
      throw new TypeError(
        `SchemaRegistry: generator for '${schemaType}' must return an object`
      );
    }
    return result;
  }
}

// ── Entity factories ─────────────────────────────────────────────────

/** Create a published blog post entity. */
function fromBlogPost({ title, bodyHtml, slug, author, excerpt, breadcrumbs } = {}) {
  return {
    entityType: "post",
    title,
    bodyHtml,
    slug,
    authorName: author || undefined,
    excerpt,
    status: "published",
    breadcrumbs,
  };
}

/** Create a published product entity. */
function fromProduct({ name, sku, price, currency, availability, description, breadcrumbs } = {}) {
  return {
    entityType: "product",
    title: name,
    sku,
    price: price === undefined ? undefined : String(price),
    priceCurrency: currency ?? "USD",
    availability: availability ?? "InStock",
    excerpt: description,
    status: "published",
    breadcrumbs,
  };
}

/** Create a published FAQ page entity. */
function fromFaq({ questions = [], title, description, breadcrumbs } = {}) {
  return {
    entityType: "faq",
    title: title ?? "FAQ",
    excerpt: description,
    faqItems: questions,
    status: "published",
    breadcrumbs,
  };
}

module.exports = {
  // Errors
  EaseoError,
  InvalidUrlError,
  ConfigurationError,
  EntityError,
  SchemaError,
  ContractError,
  // Functions
  buildSeoPayload,
  buildSeoPayloadWithOverrides,
  buildSeoContract,
  validatePayload,
  normalizePath,
  normalizePublicUrl,
  cleanUrl,
  cleanQuery,
  getSchemaRegistry,
  // Extension points
  HookRegistry,
  SchemaRegistry,
  // Factories
  fromBlogPost,
  fromProduct,
  fromFaq,
  // Internals useful for wrappers
  applyConfigExtensions,
};
