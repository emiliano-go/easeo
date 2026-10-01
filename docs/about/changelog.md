---
title: "Changelog"
description: "Release notes for easeo, from the initial 0.1.0 release onward."
---

# Changelog { #changelog }

## 0.2.0

### Breaking changes

- `lowercase_paths` now defaults to `False`: canonical URLs preserve path case
- The default tracking-parameter list is narrow; add site-specific names with
  `extra_tracking_params`
- A non-published `SEOEntity.status` now forces `noindex`
- `SEOOverrides.canonical_url` must be an absolute `http(s)` URL; use the new
  `canonical_path` for overrides that should be normalized

### Added

- `URLPolicy.extra_tracking_params` for site-specific tracking parameters
- `SEOOverrides.canonical_path`, normalized through the URL policy
- `VideoObject` schemas now include `thumbnailUrl` and `uploadDate`
- Fragments are dropped for relative routes as well as absolute URLs

### Fixed

- Percent-encoded tracking parameters such as `%75tm_source` are stripped
- Query bytes, order, and duplicates are preserved
- `status` documentation now matches the implementation

## 0.1.1

- Node.js bindings upgraded to napi-rs v3, with generated native loader and type declarations
- Generated Python type stubs and full docstrings, JSDoc, and rustdoc across every language
- Per-package READMEs and LICENSE files for the npm packages
- Package metadata polish and PEP 639 license metadata
- First release published entirely through OIDC trusted publishing

## 0.1.0

- Rust core with full SEO payload generation
- Python bindings via PyO3
- JavaScript/TypeScript bindings via napi-rs
- URL normalization and tracking parameter removal
- JSON-LD schema generation
- SEO contract system
- Deterministic output with SHA-256 hashing
- Config-scoped hooks and schema registries
- Framework integrations for Next.js, Astro, Vite, Nuxt, SvelteKit, React,
  FastAPI, Django, Flask, and Zensical
