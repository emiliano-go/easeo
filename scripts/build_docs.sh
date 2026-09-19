#!/usr/bin/env bash
#
# Build the documentation site with easeo's own Zensical SEO extension.
#
# Installs the LOCAL source tree (editable, so the freshly built Rust
# extension is used) instead of the released package, then runs the Zensical
# build. Works locally and in CI, including Cloudflare Pages.
#
# The script creates a virtual environment in .docs-venv when one is not
# already active, which keeps it working on PEP 668 (externally managed)
# systems.
#
# Usage:
#   bash scripts/build_docs.sh
#
set -euo pipefail

cd "$(dirname "$0")/.."

# Rust is required because easeo is built from source. Install it on demand so
# the script works on minimal CI images (Cloudflare Pages, for example).
if ! command -v cargo >/dev/null 2>&1; then
  echo "==> cargo not found; installing Rust toolchain"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: cargo (Rust toolchain) is still not available on PATH." >&2
  echo "Install Rust: https://rustup.rs" >&2
  exit 1
fi

# Use the active virtualenv, or create one. CF Pages sets neither, so this
# makes the script self-contained.
if [ -n "${VIRTUAL_ENV:-}" ]; then
  PYTHON="${PYTHON:-python3}"
  echo "==> Using active virtualenv: $VIRTUAL_ENV"
else
  VENV_DIR="${DOCS_VENV_DIR:-.docs-venv}"
  if [ ! -d "$VENV_DIR" ]; then
    echo "==> Creating virtual environment in $VENV_DIR"
    "${PYTHON:-python3}" -m venv "$VENV_DIR"
  fi
  # shellcheck disable=SC1091
  source "$VENV_DIR/bin/activate"
  PYTHON="python"
  echo "==> Using virtualenv: $VENV_DIR"
fi

echo "==> Python: $($PYTHON --version)"

echo "==> Installing build and docs dependencies"
"$PYTHON" -m pip install --upgrade pip
"$PYTHON" -m pip install maturin "zensical" "markdown>=3.5"

echo "==> Installing easeo from the local source tree (editable)"
"$PYTHON" -m pip install -e . --no-build-isolation

echo "==> Verifying easeo is importable"
"$PYTHON" -c "import easeo; print('easeo', easeo.__version__)"

echo "==> Regenerating llms-full.txt"
"$PYTHON" scripts/generate_llms_full.py

echo "==> Building the site"
zensical build

# Zensical copies the theme's custom_dir (docs/overrides) into the output as
# static files. They are Jinja templates, not pages, so remove them to avoid
# shipping 'overrides/main.html' with no head at all.
rm -rf site/overrides

echo "==> Done. Output in site/"
