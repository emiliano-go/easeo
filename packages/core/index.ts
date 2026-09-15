// @easeo/core - JavaScript bindings for easeo
// Loads the napi-rs native module

let native;
try {
  native = require('../easeo.node');
} catch (e) {
  throw new Error(
    '@easeo/core: Native bindings not available. ' +
    'Run `napi build --release` first, or install the pre-built package.'
  );
}

/**
 * Build an SEO payload for a given entity and route.
 * @param {import('./index.d').SEOEntity} entity
 * @param {string} route
 * @param {import('./index.d').SEOConfig} config
 * @returns {import('./index.d').SEOPayload}
 */
export function buildSeoPayload(entity, route, config) {
  return native.buildSeoPayload(entity, route, config);
}

/**
 * Build an SEO payload with overrides.
 * @param {import('./index.d').SEOEntity} entity
 * @param {string} route
 * @param {import('./index.d').SEOConfig} config
 * @param {import('./index.d').SEOOverrides} overrides
 * @returns {import('./index.d').SEOPayload}
 */
export function buildSeoPayloadWithOverrides(entity, route, config, overrides) {
  return native.buildSeoPayloadWithOverrides(entity, route, config, overrides);
}

/**
 * Build an SEO contract.
 * @param {import('./index.d').SEOContractConfig} config
 * @returns {import('./index.d').SEOContract}
 */
export function buildSeoContract(config) {
  const nativeConfig = {
    ...config,
    exceptions_json: config.exceptions ? JSON.stringify(config.exceptions) : undefined,
  };
  return native.buildSeoContract(nativeConfig);
}

/**
 * Validate an SEO payload against best practices.
 * @param {import('./index.d').SEOPayload} payload
 * @returns {import('./index.d').SEOIssue[]}
 */
export function validatePayload(payload) {
  return native.validatePayload(payload);
}

/**
 * Normalize a URL path.
 * @param {string} path
 * @param {object} options
 * @returns {string}
 */
export function normalizePath(path, options) {
  return native.normalizePath(
    path,
    options?.enforceHttps,
    options?.lowercasePaths,
    options?.trailingSlash,
    options?.collapseDuplicateSlashes,
    options?.stripTrackingParams,
    options?.allowedQueryParams,
  );
}

/**
 * Normalize a public URL.
 * @param {string} url
 * @param {import('./index.d').SEOConfig} config
 * @returns {string}
 */
export function normalizePublicUrl(url, config) {
  return native.normalizePublicUrl(url, config);
}

/**
 * Clean tracking parameters from a URL.
 * @param {string} url
 * @returns {{ url: string, removedParams: string, cleanedParams: string }}
 */
export function cleanUrl(url) {
  return native.cleanUrl(url);
}

/**
 * Clean tracking parameters from a query string.
 * @param {string} query
 * @returns {string}
 */
export function cleanQuery(query) {
  return native.cleanQuery(query);
}

/**
 * Get the schema registry for custom schema types.
 * @returns {import('./index.d').SchemaRegistry}
 */
export function getSchemaRegistry() {
  return native.getSchemaRegistry();
}
