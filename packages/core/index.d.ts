export interface SEOConfig {
  canonicalHost: string;
  publicBaseUrl: string;
  siteName?: string;
  titleTemplate?: string;
  defaultOgImage?: string;
  enforceHttps?: boolean;
  lowercasePaths?: boolean;
  trailingSlash?: "always" | "never" | "preserve";
  collapseDuplicateSlashes?: boolean;
  stripTrackingParams?: boolean;
  allowedQueryParams?: string[];
  locale?: string;
  localeAlternate?: string[];
  twitterSite?: string;
  publisherName?: string;
  publisherLogo?: string;
  autoGenerateSchema?: boolean;
  /** Emit `console.warn` for validation issues during the build. */
  emitWarnings?: boolean;
  defaultRobotsIndex?: boolean;
  defaultRobotsFollow?: boolean;
  searchRobotsIndex?: boolean;
  searchRobotsFollow?: boolean;
  schemaTypeMapJson?: string;
  /**
   * Search URL template for the homepage `WebSite` schema. Setting it adds a
   * `SearchAction`; use `{search_term_string}` as the placeholder.
   */
  searchUrlTemplate?: string;
  /** Config-scoped hooks, run after the payload is built. */
  hooks?: HookRegistry;
  /** Config-scoped custom JSON-LD generators. */
  schemaRegistry?: SchemaRegistry;
}

export interface SEOImage {
  url: string;
  width?: number;
  height?: number;
  alt?: string;
}

export interface Robots {
  index: boolean;
  follow: boolean;
  maxSnippet?: number;
  maxImagePreview?: string;
  maxVideoPreview?: number;
}

export interface SEOEntity {
  entityType: string;
  title?: string;
  description?: string;
  slug?: string;
  bodyHtml?: string;
  status?: string;
  image?: string;
  imageWidth?: number;
  imageHeight?: number;
  imageAlt?: string;
  publishedAt?: string;
  updatedAt?: string;
  authorName?: string;
  sku?: string;
  price?: string;
  priceCurrency?: string;
  availability?: string;
  sameAs?: string[];
  address?: string;
  breadcrumbs?: Breadcrumb[];
  faqItems?: FAQItem[];
}

export interface Breadcrumb {
  name: string;
  url: string;
}

export interface FAQItem {
  question: string;
  answer: string;
}

export interface SEOOverrides {
  metaTitle?: string;
  metaDescription?: string;
  canonicalUrl?: string;
  robotsIndex?: boolean;
  robotsFollow?: boolean;
  robotsMaxSnippet?: number;
  robotsMaxImagePreview?: string;
  robotsMaxVideoPreview?: number;
  ogTitle?: string;
  ogDescription?: string;
  ogImageUrl?: string;
  ogImageWidth?: number;
  ogImageHeight?: number;
  ogImageAlt?: string;
  twitterCard?: string;
  twitterTitle?: string;
  twitterDescription?: string;
  twitterImageUrl?: string;
  schemaJsonLd?: object | object[];
  omitSchema?: boolean;
  skipTitleTemplate?: boolean;
  twitterCreator?: string;
  ogAudio?: string;
  ogVideo?: string;
}

export interface OpenGraphPayload {
  type: string;
  title?: string;
  description?: string;
  url?: string;
  image?: string;
  imageWidth?: number;
  imageHeight?: number;
  imageAlt?: string;
  siteName?: string;
  locale?: string;
  localeAlternate?: string[];
  audio?: string;
  video?: string;
}

export interface TwitterPayload {
  card: string;
  title?: string;
  description?: string;
  image?: string;
  imageAlt?: string;
  site?: string;
  creator?: string;
}

/** Canonical snake_case wire format, shared with the Python/Rust APIs. */
export interface PayloadDict {
  title: string;
  description: string;
  canonical: string;
  robots: string;
  og: OpenGraphPayload;
  twitter: TwitterPayload;
  schema_jsonld?: object | object[];
}

export interface SEOPayload {
  title: string;
  description: string;
  canonical: string;
  robots: string;
  openGraph: OpenGraphPayload;
  twitter: TwitterPayload;
  schemaJsonLd?: object | object[];
  renderHtml(): string;
  renderOpengraph(): string;
  renderTwitter(): string;
  renderJsonld(): string;
  /** Plain camelCase object. `JSON.stringify(payload)` serializes this. */
  toObject(): SEOPayloadData;
  /** Alias of {@link toObject}; enables correct `JSON.stringify` behavior. */
  toJSON(): SEOPayloadData;
  /** Canonical snake_case object (matches the published JSON schema). */
  toDict(): PayloadDict;
  /** Canonical pretty-printed JSON string (matches Python `to_json`). */
  toJSONString(): string;
  /** Canonical pretty-printed JSON string. */
  toString(): string;
  /** Look up a field with a default, dict-style. */
  get(key: string, fallback?: unknown): unknown;
  /** Whether a field is present, dict-style. */
  has(key: string): boolean;
  /** Deep equality against another payload or a plain object. */
  equals(other: SEOPayload | SEOPayloadData | Record<string, unknown>): boolean;
  hash(): string;
  etag(): string;
}

/** The enumerable, camelCase data carried by a {@link SEOPayload}. */
export interface SEOPayloadData {
  title: string;
  description: string;
  canonical: string;
  robots: string;
  openGraph: OpenGraphPayload;
  twitter: TwitterPayload;
  schemaJsonLd?: object | object[];
}

/** Canonical snake_case wire format for a contract. */
export interface ContractDict {
  contract_version: string;
  generator: { name: string; version: string };
  site: { canonical_host: string; scheme: string };
  defaults: SEOExpectation;
  rules: SEOContractRule[];
  exceptions: Record<string, SEOExpectation>;
}

export interface SEOContractData {
  contractVersion: string;
  generator: { name: string; version: string };
  site: { canonicalHost: string; scheme: string };
  defaults: SEOExpectation;
  rules: SEOContractRule[];
  exceptions: Record<string, SEOExpectation>;
}

export interface SEOContract {
  contractVersion: string;
  generatorName: string;
  generatorVersion: string;
  site: { canonicalHost: string; scheme: string };
  defaults: SEOExpectation;
  rules: SEOContractRule[];
  exceptions: Record<string, SEOExpectation>;
  hash(): string;
  /** Plain camelCase object. `JSON.stringify(contract)` serializes this. */
  toObject(): SEOContractData;
  /** Alias of {@link toObject}; enables correct `JSON.stringify` behavior. */
  toJSON(): SEOContractData;
  /** Canonical snake_case object. */
  toDict(): ContractDict;
  /** Canonical JSON string. */
  toJSONString(): string;
  /** Canonical JSON string. */
  toString(): string;
}

export interface SEOContractConfig {
  canonicalHost: string;
  scheme?: string;
  defaults?: SEOExpectation;
  rules?: SEOContractRule[];
  exceptions?: Record<string, SEOExpectation>;
}

export interface SEOContractRule {
  match: string;
  expect: SEOExpectation;
  severity?: "error" | "warning" | "info";
}

export interface SEOExpectation {
  required?: boolean;
  forbidden?: boolean;
  equals?: string;
  notEquals?: string;
  contains?: string;
  matches?: string;
  oneOf?: string[];
  minLength?: number;
  maxLength?: number;
  minItems?: number;
  maxItems?: number;
  indexable?: boolean;
  canonical?: string;
  schemaRequired?: boolean;
  schemaTypes?: string[];
  ogRequired?: boolean;
  twitterRequired?: boolean;
  sitemapRequired?: boolean;
  hreflangRequired?: boolean;
  title?: SEOExpectation;
  description?: SEOExpectation;
}

export type ContractSeverity = "error" | "warning" | "info";

export interface SEOIssue {
  ruleId: string;
  severity: string;
  message: string;
  url?: string;
  details: Record<string, unknown>;
}

export declare function buildSeoPayload(
  entity: SEOEntity,
  route: string,
  config: SEOConfig,
  overrides?: SEOOverrides
): SEOPayload;

export declare function buildSeoPayloadWithOverrides(
  entity: SEOEntity,
  route: string,
  config: SEOConfig,
  overrides: SEOOverrides
): SEOPayload;

export declare function buildSeoContract(
  config: SEOContractConfig
): SEOContract;

export declare function validatePayload(
  payload: SEOPayload
): SEOIssue[];

export declare function normalizePath(
  path: string,
  options?: {
    enforceHttps?: boolean;
    lowercasePaths?: boolean;
    trailingSlash?: string;
    collapseDuplicateSlashes?: boolean;
    stripTrackingParams?: boolean;
    allowedQueryParams?: string[];
  }
): string;

export declare function normalizePublicUrl(
  url: string,
  config: SEOConfig
): string;

export declare function cleanUrl(url: string): {
  url: string;
  removedParams: Record<string, string>;
  cleanedParams: Record<string, string>;
};

export declare function cleanQuery(query: string): string;

export type SchemaGenerator = (
  entity: SEOEntity,
  config: SEOConfig,
  canonical: string,
  title: string,
  description: string | null,
  ogImage: string | null
) => object | null | undefined;

/** Config-scoped registry of custom JSON-LD generators. */
export declare class SchemaRegistry {
  register(schemaType: string, generator: SchemaGenerator): SchemaGenerator;
  register(generator: SchemaGenerator): SchemaGenerator;
  unregister(schemaType: string): void;
  get(schemaType: string): SchemaGenerator | undefined;
  has(schemaType: string): boolean;
  listTypes(): string[];
}

export type HookFunc = (
  payload: Record<string, unknown>,
  entity: SEOEntity,
  config: SEOConfig
) => Record<string, unknown>;

/** Config-scoped hook registry. */
export declare class HookRegistry {
  register(name: string, fn: HookFunc): HookFunc;
  hook(name: string): (fn: HookFunc) => HookFunc;
  unregister(name: string, fn: HookFunc): void;
  run(
    name: string,
    payload: Record<string, unknown>,
    entity: SEOEntity,
    config: SEOConfig
  ): Record<string, unknown>;
  clear(name?: string): void;
  size(): number;
}

/** Rust-backed type-name introspection registry. */
export interface RustSchemaRegistry {
  register(typeName: string): void;
  has(typeName: string): boolean;
  listTypes(): string[];
}

export declare function getSchemaRegistry(): RustSchemaRegistry;

/** Create a published blog post entity. */
export declare function fromBlogPost(input: {
  title: string;
  bodyHtml: string;
  slug?: string;
  author?: string;
  excerpt?: string;
  breadcrumbs?: Breadcrumb[];
}): SEOEntity;

/** Create a published product entity. */
export declare function fromProduct(input: {
  name: string;
  sku: string;
  price: string | number;
  currency?: string;
  availability?: string;
  description?: string;
  breadcrumbs?: Breadcrumb[];
}): SEOEntity;

/** Create a published FAQ page entity. */
export declare function fromFaq(input: {
  questions: FAQItem[];
  title?: string;
  description?: string;
  breadcrumbs?: Breadcrumb[];
}): SEOEntity;

export declare class EaseoError extends Error {
  code: string;
}
export declare class InvalidUrlError extends EaseoError {}
export declare class ConfigurationError extends EaseoError {}
export declare class EntityError extends EaseoError {}
export declare class SchemaError extends EaseoError {}
export declare class ContractError extends EaseoError {}
