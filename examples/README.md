# easeo examples

Minimal, runnable examples for each binding. Every example takes the same
entity/route/config and prints the generated `<head>` HTML.

## Python

```bash
pip install easeo
python examples/python_basic.py
```

## Node.js

```bash
npm install @easeo/core
node examples/node_basic.mjs
```

## Rust

The Rust core is exercised by the doctest-style examples in
`crates/easeo-core/src/lib.rs` (`cargo test --workspace`). The
`README.md` quick start shows the full call.
