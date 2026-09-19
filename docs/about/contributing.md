---
title: "Contributing"
description: "Build, test, and documentation conventions for contributing to easeo."
---

# Contributing { #contributing }

## Repository layout { #layout }

```text
easeo/
├── Cargo.toml                 # Rust workspace
├── pyproject.toml             # maturin / Python package
├── crates/
│   ├── easeo-core/            # all logic
│   ├── easeo-python/          # PyO3 bindings
│   └── easeo-node/            # napi-rs bindings
├── packages/core/             # @easeo/core wrapper
├── integrations/              # JS framework integrations
├── python/easeo/              # Python package, adapters, contrib
├── tests/                     # Python, JavaScript, conformance
├── fixtures/                  # shared fixtures
├── schemas/                   # JSON schemas
└── docs/                      # this documentation
    ├── overrides/             # Zensical theme overrides (main.html, partials)
    └── stylesheets/extra.css  # theme CSS (accent, mobile drawer, tab dropdowns)
```

## Build and test { #build }

```bash
# Rust
cargo test --workspace
cargo fmt --all: --check
cargo clippy --workspace: -D warnings

# Python
maturin develop -m crates/easeo-python/Cargo.toml
pytest tests/python/

# JavaScript
cd packages/core
napi build --platform --release --manifest-path ../../crates/easeo-node/Cargo.toml
cp ../../crates/easeo-node/*.node .
cd ../..
node --test tests/javascript/*.cjs

# Cross-language conformance
python tests/conformance/test_conformance.py
```

## Rules of the codebase { #rules }

* **All logic lives in Rust.** The Python and JavaScript packages are bindings
  and thin ergonomics wrappers. Do not duplicate resolution logic in a binding.
* **Determinism is non-negotiable.** No timestamps, randomness, environment
  reads, or unordered maps in output. Use `BTreeMap` for anything that
  serializes.
* **Cross-language parity.** A change to the Python API needs the JavaScript
  equivalent, and a conformance test where output could differ.
* **Escaping happens in the core.** HTML and JSON-LD escaping is centralized so
  every binding is safe.

## Documentation { #docs }

Docs live in `docs/` and build with Zensical. The site uses the easeo
`easeo.contrib.zensical` extension to generate per-page SEO tags, so easeo
must be importable by the same interpreter that runs Zensical.

[`uv`](https://docs.astral.sh/uv/) handles this in one command. It builds the
Rust extension from `crates/easeo-python/` into a local `.venv` and runs
Zensical with the `dev` dependency group, which includes `zensical` and
`markdown`:

```bash
uv run zensical serve     # live preview on http://localhost:8000
uv run zensical build     # writes site/
uv run python scripts/generate_llms_full.py   # regenerate docs/llms-full.txt
```

If you prefer a manual environment, install the extra and run Zensical
directly:

```bash
pip install -e ".[zensical]"
zensical serve
```

To reproduce the exact artifact that CI and Cloudflare Pages deploy (editable
easeo, regenerated `llms-full.txt`, then the build), use the build script:

```bash
bash scripts/build_docs.sh
```

See [Deploying the Docs](deploying-docs.md#deploying-the-docs) for the
Cloudflare Pages and GitHub Pages settings.

Keep prose free of em dashes and double-hyphen separators; use commas, colons,
parentheses, or semicolons.

## Adding a framework integration { #integration }

1. Create a directory under `integrations/` with `index.js`, `index.d.ts`, and
   `package.json`.
2. Support both default and named exports.
3. Call `@easeo/core` for all payload building. Never re-implement logic.
4. Add a test under `tests/javascript/`.
5. Add a page under `docs/integrations/` and a nav entry in `zensical.toml`.
