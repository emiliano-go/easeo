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
  emitWarnings?: boolean;
  defaultRobotsIndex?: boolean;
  defaultRobotsFollow?: boolean;
  searchRobotsIndex?: boolean;
  searchRobotsFollow?: boolean;
  schemaTypeMapJson?: string;
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
  toObject(): object;
  toJSON(): string;
  hash(): string;
  etag(): string;
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
  toJSON(): string;
  toDict(): object;
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
  details: string;
}

export declare function buildSeoPayload(
  entity: SEOEntity,
  route: string,
  config: SEOConfig
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
  removedParams: string;
  cleanedParams: string;
};

export declare function cleanQuery(query: string): string;

export interface SchemaRegistry {
  register(typeName: string): void;
  has(typeName: string): boolean;
  listTypes(): string[];
}

export declare function getSchemaRegistry(): SchemaRegistry;
