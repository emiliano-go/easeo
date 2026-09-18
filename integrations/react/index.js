// @easeo/react — React integration for easeo
// <EaseoHead /> keeps document.head in sync with the easeo payload.
// SSR-safe: renders nothing when `document` is unavailable
// (put payload.renderHtml() into your HTML template for SSR/SSG).

const { buildSeoPayload } = require("@easeo/core");

/**
 * @typedef {Object} EaseoHeadProps
 * @property {import("./index.d").SEOEntity} entity
 * @property {string} route
 * @property {import("./index.d").SEOConfig} config
 */

function setMeta(head, attrName, attrValue, content) {
  if (!content) return;
  let el = head.querySelector(`meta[${attrName}="${attrValue}"]`);
  if (!el) {
    el = document.createElement("meta");
    el.setAttribute(attrName, attrValue);
    head.appendChild(el);
  }
  el.content = content;
}

/**
 * React component for injecting SEO meta tags into <head>.
 * Always renders null; for SSR use payload.renderHtml() in your template.
 *
 * @param {EaseoHeadProps} props
 * @returns {null}
 *
 * @example
 * import { EaseoHead } from "@easeo/react";
 *
 * <EaseoHead
 *   entity={{ entityType: "product", title: product.name, description: product.description }}
 *   route={`/products/${product.slug}`}
 *   config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
 * />
 */
function EaseoHead({ entity, route, config }) {
  const payload = buildSeoPayload(entity, route, config);

  if (typeof document === "undefined") {
    return null;
  }

  const head = document.head;

  // Title
  let titleEl = head.querySelector("title");
  if (!titleEl) {
    titleEl = document.createElement("title");
    head.appendChild(titleEl);
  }
  titleEl.textContent = payload.title;

  // Meta description
  setMeta(head, "name", "description", payload.description);

  // Canonical
  let canonicalEl = head.querySelector('link[rel="canonical"]');
  if (!canonicalEl) {
    canonicalEl = document.createElement("link");
    canonicalEl.rel = "canonical";
    head.appendChild(canonicalEl);
  }
  canonicalEl.href = payload.canonical;

  // Robots
  setMeta(head, "name", "robots", payload.robots);

  // Open Graph
  setMeta(head, "property", "og:title", payload.openGraph.title);
  setMeta(head, "property", "og:description", payload.openGraph.description);
  setMeta(head, "property", "og:url", payload.openGraph.url);
  setMeta(head, "property", "og:image", payload.openGraph.image);
  setMeta(head, "property", "og:site_name", payload.openGraph.siteName);

  // Twitter
  setMeta(head, "name", "twitter:card", payload.twitter.card);
  setMeta(head, "name", "twitter:title", payload.twitter.title);
  setMeta(head, "name", "twitter:description", payload.twitter.description);

  return null;
}

module.exports = EaseoHead;
module.exports.EaseoHead = EaseoHead;
module.exports.default = EaseoHead;
