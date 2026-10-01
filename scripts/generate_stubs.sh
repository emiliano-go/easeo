#!/usr/bin/env bash
#
# Regenerate the Python type stub for the native extension module.
#
# PyO3 embeds introspection data for exported classes and functions, so the
# stub is generated from the built extension. The exception types are created
# dynamically at import time and have no static declaration, so they are
# appended after generation.
#
# Usage:
#   bash scripts/generate_stubs.sh
#
set -euo pipefail

cd "$(dirname "$0")/.."

OUT="$(mktemp -d)"
trap 'rm -rf "$OUT"' EXIT

maturin generate-stubs --out "$OUT" -m crates/easeo-python/Cargo.toml

# Older maturin writes the stub at the output root, newer versions mirror the
# module path. Find whichever was produced.
GENERATED="$(find "$OUT" -name '_easeo_native.pyi' -print -quit)"
if [ -z "$GENERATED" ]; then
  echo "ERROR: maturin did not produce _easeo_native.pyi" >&2
  exit 1
fi

cp "$GENERATED" python/easeo/_easeo_native.pyi

cat >> python/easeo/_easeo_native.pyi <<'EOF'

# Exception types are created dynamically at import time; declare them here
# so IDEs and type checkers can see them.
class EaseoError(ValueError): ...
class InvalidUrlError(EaseoError): ...
class ConfigurationError(EaseoError): ...
class EntityError(EaseoError): ...
class SchemaError(EaseoError): ...
class ContractError(EaseoError): ...
EOF
