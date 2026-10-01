# Changelog

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
