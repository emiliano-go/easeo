#!/usr/bin/env bash
#
# Regenerate the napi-rs native loader and its type declarations.
#
# The generated files are `packages/core/native.js` and
# `packages/core/native.d.ts`. The public wrapper in `index.js` and its
# declarations in `index.d.ts` are maintained by hand and are not touched.
# If the Rust type names change, update the aliases in `index.d.ts` to match.
#
# Usage:
#   bash scripts/generate_node_bindings.sh
#
set -euo pipefail

cd "$(dirname "$0")/.."

cd packages/core

napi build --platform --release \
  --manifest-path ../../crates/easeo-node/Cargo.toml \
  --output-dir . \
  --js native.js \
  --dts native.d.ts
