// Type-level smoke test for @easeo/core. Compiled by `tsc --noEmit` in CI.
// Keep this file free of runtime imports: it only exercises the declarations.

import {
  buildSeoContract,
  buildSeoPayload,
  buildSeoPayloadWithOverrides,
  cleanQuery,
  cleanUrl,
  ConfigurationError,
  EaseoError,
  fromBlogPost,
  fromFaq,
  fromProduct,
  getSchemaRegistry,
  HookRegistry,
  normalizePath,
  normalizePublicUrl,
  SchemaRegistry,
  validatePayload,
} from "../../packages/core/index";

import type {
  Breadcrumb,
  CleanResult,
  ConfigurationError as ConfigurationErrorType,
  FAQItem,
  OGPayload,
  SEOConfig,
  SEOContract,
  SEOContractConfig,
  SEOEntity,
  SEOIssue,
  SEOOverrides,
  SEOPayload,
  TwitterPayload,
} from "../../packages/core/index";

const hooks = new HookRegistry();
hooks.register("post_process", (payload, entity, config) => {
  void entity;
  void config;
  return payload;
});

const schemaRegistry = new SchemaRegistry();
schemaRegistry.register("Podcast", (entity, config, canonical, title, description, ogImage) => {
  void entity;
  void config;
  void canonical;
  void title;
  void description;
  void ogImage;
  return { "@type": "Podcast" };
});

const config: SEOConfig = {
  canonicalHost: "example.com",
  publicBaseUrl: "https://example.com",
  hooks,
  schemaRegistry,
};

const entity: SEOEntity = {
  entityType: "post",
  title: "Hello",
  description: "An example post.",
};

const payload: SEOPayload = buildSeoPayload(entity, "/blog/hello", config);
const overridden: SEOPayload = buildSeoPayloadWithOverrides(entity, "/blog/hello", config, {
  metaTitle: "Overridden",
} satisfies SEOOverrides);

const html: string = payload.renderHtml();
const dict: Record<string, unknown> = payload.toDict();
const object: Record<string, unknown> = payload.toObject();
const hash: string = payload.hash();
const etag: string = payload.etag();
const equal: boolean = payload.equals(overridden);
const value: unknown = payload.get("title", null);
const present: boolean = payload.has("title");
const openGraph: OGPayload = payload.openGraph;
const twitter: TwitterPayload = payload.twitter;

const contractConfig: SEOContractConfig = {
  canonicalHost: "example.com",
  exceptions: {},
};
const contract: SEOContract = buildSeoContract(contractConfig);
const contractJson: string = contract.toJSONString();
const contractHash: string = contract.hash();

const issues: SEOIssue[] = validatePayload(payload);
const clean: CleanResult = cleanUrl("https://example.com/?utm_source=x");
const query: string = cleanQuery("a=1&utm_source=x");
const normalized: string = normalizePath("/blog//hello", { trailingSlash: "never" });
const publicUrl: string = normalizePublicUrl("/blog/hello", config);
const rustRegistry = getSchemaRegistry();
const hasType: boolean = rustRegistry.has("Article");

const post = fromBlogPost({ title: "T", bodyHtml: "<p>body</p>" });
const product = fromProduct({ name: "Widget", sku: "W-1", price: 9.99 });
const faq = fromFaq({ questions: [{ question: "Q?", answer: "A." }] });
const breadcrumb: Breadcrumb = { name: "Home", url: "/" };
const faqItem: FAQItem = { question: "Q?", answer: "A." };
const error: EaseoError = new ConfigurationError("invalid");
const errorType: ConfigurationErrorType = error;

// Keep every binding referenced so the compiler checks all of them.
export const surface = {
  html,
  dict,
  object,
  hash,
  etag,
  equal,
  value,
  present,
  openGraph,
  twitter,
  contractJson,
  contractHash,
  issues,
  clean,
  query,
  normalized,
  publicUrl,
  hasType,
  post,
  product,
  faq,
  breadcrumb,
  faqItem,
  errorType,
};
