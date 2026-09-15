// @easeo/react — React integration for easeo
// Provides EaseoHead component for SEO metadata in <head>

import { buildSeoPayload, type SEOConfig, type SEOEntity, type SEOPayload } from "@easeo/core";

export interface EaseoHeadProps {
  entity: SEOEntity;
  route: string;
  config: SEOConfig;
}

/**
 * React component for injecting SEO meta tags into <head>.
 * Uses React 18+ createPortal to render into document.head.
 *
 * @example
 * ```tsx
 * import { EaseoHead } from "@easeo/react";
 *
 * function ProductPage({ product }) {
 *   return (
 *     <>
 *       <EaseoHead
 *         entity={{ entityType: "product", title: product.name, description: product.description }}
 *         route={`/products/${product.slug}`}
 *         config={{ publicBaseUrl: "https://example.com", canonicalHost: "example.com" }}
 *       />
 *       <h1>{product.name}</h1>
 *     </>
 *   );
 * }
 * ```
 */
export function EaseoHead({ entity, route, config }: EaseoHeadProps): React.ReactElement | null {
  const payload: SEOPayload = buildSeoPayload(entity, route, config);

  // For SSR: return null (meta tags should be in the HTML template)
  // For CSR: could use createPortal to inject into head
  // This is a simplified implementation
  if (typeof document === "undefined") {
    return null;
  }

  // Client-side: update document head
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
  let canonicalEl = head.querySelector('link[rel="canonical"]') as HTMLLinkElement | null;
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

function setMeta(head: HTMLHeadElement, attrName: string, attrValue: string, content: string | undefined): void {
  if (!content) return;

  let el = head.querySelector(`meta[${attrName}="${attrValue}"]`) as HTMLMetaElement | null;
  if (!el) {
    el = document.createElement("meta");
    el.setAttribute(attrName, attrValue);
    head.appendChild(el);
  }
  el.content = content;
}

export default EaseoHead;
