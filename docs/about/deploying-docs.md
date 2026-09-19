---
title: "Deploying the Docs"
description: "Build and deploy the easeo documentation site, including Cloudflare Pages settings."
---

# Deploying the Docs { #deploying-the-docs }

The documentation site dogfoods easeo: `easeo.contrib.zensical` generates the
per-page SEO tags, so easeo must be installed from the local source tree for
the site to have a real head. The build installs easeo **editable**, which
compiles the Rust extension and uses the working tree rather than the released
package.

## One command { #one-command }

`scripts/build_docs.sh` does the whole thing: it creates a virtual
environment, installs the build and docs dependencies, installs easeo
editable, regenerates `llms-full.txt`, and runs the Zensical build.

```bash
bash scripts/build_docs.sh
```

The output is written to `site/`.

Under the hood it runs the equivalent of:

```bash
python -m venv .docs-venv
source .docs-venv/bin/activate
pip install maturin zensical "markdown>=3.5"
pip install -e . --no-build-isolation
python scripts/generate_llms_full.py
zensical build
```

`pip install -e .` uses the `[tool.maturin]` `manifest-path` in
`pyproject.toml`, so it builds `crates/easeo-python/` and not the workspace
root.

## Requirements { #requirements }

* Python 3.10 or newer (pinned to 3.12 in `.python-version`).
* A Rust toolchain (stable). The script installs one with `rustup` when `cargo`
  is missing, so minimal CI images work without a custom build image.

## Cloudflare Pages { #cloudflare-pages }

Connect the repository and set:

| Setting | Value |
|---|---|
| Framework preset | None |
| Build command | `bash scripts/build_docs.sh` |
| Build output directory | `site` |
| Root directory | `/` |

Environment variables:

| Variable | Value |
|---|---|
| `PYTHON_VERSION` | `3.12` |
| `DOCS_VENV_DIR` | `.docs-venv` (optional, this is the default) |

Cloudflare runs the build in a container that already has Python and pip. The
script creates its own virtualenv and installs Rust with `rustup` when it is
missing, so no global install and no `--break-system-packages` are needed.

## GitHub Pages { #github-pages }

The repository also ships a GitHub Pages workflow at
`.github/workflows/docs.yml`. It installs easeo with the `zensical` extra and
runs `zensical build`. Enable Pages with the GitHub Actions source in the
repository settings.

## Custom domain { #domain }

`docs/CNAME` contains the custom domain, so it is copied into `site/` at build
time:

```text
easeo.emiliano-go.com
```

Point the domain at the Pages project and keep `site_url` in `zensical.toml`
in sync, because the URL feeds canonical tags and the sitemap.

## Post-build SEO checklist { #checklist }

After a build, confirm:

* Every page has exactly one `<title>`.
* Every page has `<link rel="canonical">` with the production URL.
* Every page has a real `<meta name="description">` from front matter.
* Every page has Open Graph and Twitter tags and a JSON-LD block.
* `site/robots.txt` points at the easeo sitemap.
* `site/sitemap.xml` lists the production URLs.
