# Changelog

## 0.2.0 (2026-10-01)

### Breaking changes

- `URLPolicy.lowercase_paths` now defaults to `false`: canonical URLs preserve
  path case, because `/Products/iPhone` and `/products/iphone` are not always
  the same resource. Set `lowercase_paths=True` to restore lowercasing.
- The default tracking-parameter list is now conservative. Only UTM
  parameters, vendor click ids, and unambiguous session ids are stripped.
  Generic names (`ref`, `source`, `tag`, `keyword`, `campaign`, `redirect`,
  `next`, `timestamp`, `t`, and similar) are preserved. Use
  `extra_tracking_params` to strip site-specific names.
- A non-published `SEOEntity.status` now forces `noindex`. Previously any
  status other than `"published"` fell back to `config.default_robots`, which
  defaulted to `index,follow`, so drafts could be indexed.
- `SEOOverrides.canonical_url` must be an absolute `http(s)` URL. Relative
  values now raise an error instead of silently entering the payload.

### Added

- `URLPolicy.extra_tracking_params` for site-specific tracking parameters.
- `SEOOverrides.canonical_path`, a canonical override that still goes through
  the URL normalization pipeline.
- Query parameters in an `allowed_query_params` allowlist are now kept even
  when they match a tracking pattern, as documented.
- `VideoObject` schemas now include `thumbnailUrl` and `uploadDate`.
- Fragments are dropped for relative routes as well as absolute URLs.

### Fixed

- Query parameter matching now decodes keys, so percent-encoded tracking
  parameters such as `%75tm_source` are stripped. Emitted queries preserve
  the original bytes, order, and duplicates.
- `status` documentation now matches the implementation (`"published"`, not
  `"publish"`).

## 0.1.1 (2026-10-01)

- Node.js bindings upgraded to napi-rs v3; the native loader and `native.d.ts` are now fully generated, and the TypeScript declarations are type checked in CI
- IDE autocomplete: generated Python type stubs (`_easeo_native.pyi`) with a CI freshness check
- Full JSDoc on the generated TypeScript declarations and the framework integrations
- Full API documentation: rustdoc for every public `easeo-core` item, Google-style docstrings for the Python layer, JSDoc for the Node bindings
- Per-package READMEs and LICENSE files for all seven npm packages
- Package metadata: keywords, homepage, bugs, and engines for npm; keywords, categories, and authors for crates.io
- PEP 639 license metadata in `pyproject.toml`
- First release published entirely through OIDC trusted publishing (PyPI, crates.io, npm)

## 0.1.0 (2026-09-14)

- Initial release
- Rust core with full SEO payload generation
- Python bindings via PyO3
- JavaScript/TypeScript bindings via napi-rs
- URL normalization and tracking parameter removal
- JSON-LD schema generation
- SEO contract system
- Deterministic output with SHA-256 hashing
