/* Type declarations for @easeo/core.
 *
 * The Rust-generated record types live in `native.d.ts`. This file declares
 * the JavaScript wrapper surface: the payload and contract wrappers, the
 * extension points, the factories, and the error classes. It also augments
 * the native config with the JavaScript-only extension fields.
 *
 * The native declarations use the Rust struct names (`NodeSeoConfig` and
 * friends); this file exports them under the public names.
 */

export type {
  NodeBreadcrumb as Breadcrumb,
  NodeCleanResult as CleanResult,
  NodeContractSite as ContractSite,
  NodeFaqItem as FAQItem,
  NodeOpenGraph as OGPayload,
  NodeTwitter as TwitterPayload,
  NodeSeoConfig as SEOConfig,
  NodeSeoContractRule as SEOContractRule,
  NodeSeoEntity as SEOEntity,
  NodeSeoExpectation as SEOExpectation,
  NodeSeoIssue as SEOIssue,
  NodeSeoOverrides as SEOOverrides,
} from "./native";

import type {
  NodeBreadcrumb,
  NodeCleanResult,
  NodeContractSite,
  NodeFaqItem,
  NodeOpenGraph,
  NodeSeoConfig,
  NodeSeoContractRule,
  NodeSeoEntity,
  NodeSeoExpectation,
  NodeSeoIssue,
  NodeSeoOverrides,
  NodeTwitter,
} from "./native";

/** Config-scoped hooks are a JavaScript-only extension of the native config. */
declare module "./native" {
  interface NodeSeoConfig {
    /** Config-scoped hooks, run after the payload is built. */
    hooks?: HookRegistry;
    /** Config-scoped custom JSON-LD generators. */
    schemaRegistry?: SchemaRegistry;
  }
}

/** A custom JSON-LD generator registered on a {@link SchemaRegistry}. */
export type SchemaGenerator = (
  entity: NodeSeoEntity,
  config: NodeSeoConfig,
  canonical: string,
  title: string,
  description: string | null,
  ogImage: string | null
) => object | null | undefined;

/** A hook function registered on a {@link HookRegistry}. */
export type HookFunc = (
  payload: Record<string, unknown>,
  entity: NodeSeoEntity,
  config: NodeSeoConfig
) => Record<string, unknown>;

/** Config-scoped registry of custom JSON-LD generators. */
export declare class SchemaRegistry {
  /** Register a generator for a schema type. */
  register(schemaType: string, generator: SchemaGenerator): SchemaGenerator;
  /** Register a generator under its function name. */
  register(generator: SchemaGenerator): SchemaGenerator;
  unregister(schemaType: string): void;
  get(schemaType: string): SchemaGenerator | undefined;
  has(schemaType: string): boolean;
  listTypes(): string[];
}

/** Config-scoped hook registry. */
export declare class HookRegistry {
  /** Register a function for a hook point. */
  register(name: string, fn: HookFunc): HookFunc;
  /** Decorator form: `hooks.hook("post_process")(fn)`. */
  hook(name: string): (fn: HookFunc) => HookFunc;
  unregister(name: string, fn: HookFunc): void;
  run(
    name: string,
    payload: Record<string, unknown>,
    entity: NodeSeoEntity,
    config: NodeSeoConfig
  ): Record<string, unknown>;
  clear(name?: string): void;
  size(): number;
}

/** The resolved, deterministic SEO payload returned by the wrappers. */
export interface SEOPayload {
  /** Resolved title. */
  title: string;
  /** Resolved description. */
  description: string;
  /** Normalized canonical URL. */
  canonical: string;
  /** Serialized robots directives, for example `"index,follow"`. */
  robots: string;
  /** Open Graph metadata. */
  openGraph: NodeOpenGraph;
  /** Twitter Card metadata. */
  twitter: NodeTwitter;
  /** Generated JSON-LD, when schema generation is enabled. */
  schemaJsonLd?: unknown;
  /** Fields added by config-scoped hooks. */
  [key: string]: unknown;
  /** Renders the full head snippet. */
  renderHtml(): string;
  /** Renders only the Open Graph meta tags. */
  renderOpengraph(): string;
  /** Renders only the Twitter Card meta tags. */
  renderTwitter(): string;
  /** Renders only the JSON-LD script tag. */
  renderJsonld(): string;
  /** Returns the payload as a plain camelCase object. */
  toObject(): Record<string, unknown>;
  /** Alias of `toObject()` for JSON serialization. */
  toJSON(): Record<string, unknown>;
  /** Returns the canonical snake_case wire format. */
  toDict(): Record<string, unknown>;
  /** Returns the canonical wire format as a JSON string. */
  toJSONString(): string;
  /** Returns the canonical wire format as a JSON string. */
  toString(): string;
  /** Returns the SHA-256 hash of the payload. */
  hash(): string;
  /** Returns the payload hash as a quoted HTTP ETag. */
  etag(): string;
  /** Looks up a camelCase field with a default. */
  get(key: string, fallback?: unknown): unknown;
  /** Whether a camelCase field is present. */
  has(key: string): boolean;
  /** Deep equality against another payload or a plain object. */
  equals(other: SEOPayload | Record<string, unknown>): boolean;
}

/** A generated, machine-readable SEO contract. */
export interface SEOContract {
  contractVersion: string;
  generatorName: string;
  generatorVersion: string;
  site: NodeContractSite;
  defaults: NodeSeoExpectation;
  rules: NodeSeoContractRule[];
  exceptions: Record<string, NodeSeoExpectation>;
  hash(): string;
  toObject(): Record<string, unknown>;
  toJSON(): Record<string, unknown>;
  toDict(): Record<string, unknown>;
  toJSONString(): string;
  toString(): string;
}

/** Input configuration for {@link buildSeoContract}. */
export interface SEOContractConfig {
  canonicalHost: string;
  scheme?: string;
  defaults?: NodeSeoExpectation;
  rules?: NodeSeoContractRule[];
  exceptions?: Record<string, NodeSeoExpectation>;
}

/** Options accepted by {@link normalizePath}. */
export interface NormalizePathOptions {
  enforceHttps?: boolean;
  /** Defaults to `false`: path case is preserved. */
  lowercasePaths?: boolean;
  trailingSlash?: "always" | "never" | "preserve";
  collapseDuplicateSlashes?: boolean;
  stripTrackingParams?: boolean;
  /** Parameters kept even when they match a tracking pattern. */
  allowedQueryParams?: string[];
  /** Extra parameter names to strip, on top of the built-in list. */
  extraTrackingParams?: string[];
}

/** Rust-backed type-name introspection registry. */
export interface RustSchemaRegistry {
  register(typeName: string): void;
  has(typeName: string): boolean;
  listTypes(): string[];
}

/**
 * Builds an SEO payload for an entity at a route.
 *
 * @param entity - Content entity.
 * @param route - Route path, for example `"/blog/hello"`.
 * @param config - Site-wide configuration.
 * @param overrides - Optional per-call overrides.
 * @returns The resolved SEO payload.
 */
export declare function buildSeoPayload(
  entity: NodeSeoEntity,
  route: string,
  config: NodeSeoConfig,
  overrides?: NodeSeoOverrides
): SEOPayload;

/** Builds a payload with explicit overrides. */
export declare function buildSeoPayloadWithOverrides(
  entity: NodeSeoEntity,
  route: string,
  config: NodeSeoConfig,
  overrides: NodeSeoOverrides
): SEOPayload;

/** Builds a machine-readable SEO contract. */
export declare function buildSeoContract(config: SEOContractConfig): SEOContract;

/** Runs the built-in validation checks against a payload. */
export declare function validatePayload(payload: SEOPayload): NodeSeoIssue[];

/** Normalizes a route path according to the given options. */
export declare function normalizePath(path: string, options?: NormalizePathOptions): string;

/** Resolves a path or URL against the configured public base URL. */
export declare function normalizePublicUrl(url: string, config: NodeSeoConfig): string;

/** Removes tracking parameters from a URL. */
export declare function cleanUrl(url: string): NodeCleanResult;

/** Removes tracking parameters from a query string. */
export declare function cleanQuery(query: string): string;

/** Returns the Rust-backed schema registry introspection handle. */
export declare function getSchemaRegistry(): RustSchemaRegistry;

/** Creates a published blog post entity. */
export declare function fromBlogPost(input: {
  title: string;
  bodyHtml: string;
  slug?: string;
  author?: string;
  excerpt?: string;
  breadcrumbs?: NodeBreadcrumb[];
}): NodeSeoEntity;

/** Creates a published product entity. */
export declare function fromProduct(input: {
  name: string;
  sku: string;
  price: string | number;
  currency?: string;
  availability?: string;
  description?: string;
  breadcrumbs?: NodeBreadcrumb[];
}): NodeSeoEntity;

/** Creates a published FAQ page entity. */
export declare function fromFaq(input: {
  questions: NodeFaqItem[];
  title?: string;
  description?: string;
  breadcrumbs?: NodeBreadcrumb[];
}): NodeSeoEntity;

/** Base class for all easeo errors. */
export declare class EaseoError extends Error {
  code: string;
}
/** A URL is malformed or a URL policy is invalid. */
export declare class InvalidUrlError extends EaseoError {}
/** A config value failed validation. */
export declare class ConfigurationError extends EaseoError {}
/** An entity or overrides value failed validation. */
export declare class EntityError extends EaseoError {}
/** JSON-LD construction failed. */
export declare class SchemaError extends EaseoError {}
/** Contract generation failed. */
export declare class ContractError extends EaseoError {}
