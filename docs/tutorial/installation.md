---
title: "Installation"
description: "Install easeo for Python, JavaScript, and from source."
---

# Installation { #installation }

easeo ships as two packages backed by the same Rust core.

## Python { #python }

```bash
pip install easeo
```

Requires Python 3.10 or newer. The wheel bundles the compiled Rust extension,
so there is no build step and no separate Rust toolchain to install.

Optional extras add the framework adapters:

```bash
pip install "easeo[fastapi]"
pip install "easeo[django]"
pip install "easeo[flask]"
pip install "easeo[zensical]"
pip install "easeo[all]"     # every adapter
```

## JavaScript / TypeScript { #javascript }

```bash
npm install @easeo/core
```

The published package bundles a prebuilt native module for Linux (glibc and
musl), macOS, and Windows on x64 and arm64. Node 18 or newer is required.

Framework integrations are separate packages:

```bash
npm install @easeo/next
npm install @easeo/astro
npm install @easeo/vite
npm install @easeo/nuxt
npm install @easeo/sveltekit
npm install @easeo/react
```

## Verify the install { #verify }

=== "Python"

    ```python
    import easeo
    print(easeo.__version__)
    # 0.1.0
    ```

=== "JavaScript"

    ```js
    const easeo = require("@easeo/core");
    console.log(typeof easeo.buildSeoPayload);
    // function
    ```

## Building from source { #from-source }

You need a Rust toolchain (stable) plus `maturin` for Python or `@napi-rs/cli`
for Node.

```bash
# Rust core
cargo build --release

# Python bindings
maturin develop -m crates/easeo-python/Cargo.toml

# Node bindings
cd packages/core
napi build --platform --release --manifest-path ../../crates/easeo-node/Cargo.toml
cp ../../crates/easeo-node/*.node .
```

## Recap { #recap }

* `pip install easeo` or `npm install @easeo/core`.
* Extras add the framework adapters, not the core.
* Prebuilt wheels and native modules mean no compiler is needed.

**Next:** [First Payload](first-payload.md#first-payload).
