---
title: easeo documentation
description: "Deterministic SEO metadata generation for content platforms. Rust core with Python and JavaScript/TypeScript bindings."
---

<div align="center">
  <h1 id="easeo-documentation">easeo documentation</h1>
</div>

`easeo` is a deterministic SEO metadata generator. It turns a content entity,
a route path, and a site configuration into one structured payload: canonical
URL, title, description, robots directives, Open Graph, Twitter Cards, and
JSON-LD. The core is written in Rust and shipped to Python and
JavaScript/TypeScript, so the same inputs produce byte-for-byte identical
output in every language.

```text
content entity + route + config
    |
    v
easeo (library)
    |
    v
SEOPayload
    |
    +--> framework adapter --> <head>
    |
    +--> SEO contract --> validation
```

## What easeo is { #what-easeo-is }

- A Rust workspace with three crates:
  - [`easeo-core`](reference/rust-api.md): all the logic, with no I/O.
  - `easeo-python`: PyO3 bindings.
  - `easeo-node`: napi-rs bindings.
- One primary function: `build_seo_payload(entity, route, config)`.
- Pure and deterministic: no timestamps, no randomness, no environment reads.
- Framework-agnostic: adapters for Next.js, Astro, Vite, Nuxt, SvelteKit,
  React, FastAPI, Django, Flask, and Zensical.

## Why easeo { #why-easeo }

- **Deterministic output.** Same inputs, same bytes. Snapshot test it, hash
  it, cache it forever, diff it across deployments.
- **One implementation, three languages.** The Rust core guarantees that
  Python and JavaScript agree, verified by cross-language conformance tests.
- **Contract-first.** Encode your SEO intent as a machine-readable contract
  and fail the build when it drifts.
- **Zero ceremony.** One function call returns a payload that renders itself
  to safe, ready-to-inject head HTML.

## What easeo is not { #what-easeo-is-not }

- **Not an SEO crawler.** It does not fetch your site.
- **Not a score generator.** It does not grade content.
- **Not a keyword tool.** It formats the data you give it.
- **Not a browser automation framework.** It performs no I/O.
- **Not an analytics platform.** It stores no state.

## Quick start { #quick-start }

=== "Python"

    ```bash
    pip install easeo
    ```

    ```python
    from easeo import SEOConfig, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        site_name="Example",
    )

    entity = SEOEntity(
        entity_type="post",
        title="Hello World",
        excerpt="An example post.",
    )

    payload = build_seo_payload(entity, "/blog/hello", config)
    print(payload.render_html())
    ```

=== "JavaScript"

    ```bash
    npm install @easeo/core
    ```

    ```js
    const { buildSeoPayload } = require("@easeo/core");

    const payload = buildSeoPayload(
      { entityType: "post", title: "Hello World", description: "An example post." },
      "/blog/hello",
      { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
    );

    console.log(payload.renderHtml());
    ```

=== "Rust"

    ```rust
    use easeo_core::{SEOConfig, SEOEntity, EntityType, build_seo_payload};

    let config = SEOConfig {
        canonical_host: "example.com".into(),
        public_base_url: "https://example.com".into(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Hello World".into()),
        excerpt: Some("An example post.".into()),
        ..Default::default()
    };

    let payload = build_seo_payload(&entity, "/blog/hello", &config)?;
    assert_eq!(payload.canonical, "https://example.com/blog/hello");
    ```

## Guides { #guides }

- [Tutorial](tutorial/index.md): install, first payload, fallbacks, rendering,
  contracts, configuration.
- [Concepts](concepts/index.md): determinism, the entity and payload models,
  fallback chains, URL normalization, schemas, validation.
- [Guides](guides/index.md): custom JSON-LD, hooks, contracts in CI, framework
  recipes, migration.
- [Reference](reference/python-api.md): Python, JavaScript, and Rust APIs.
- [Integrations](integrations/index.md): per-framework setup.
- [Examples](examples/index.md): a simple and a complex example for every
  integration.
- [Recipes](recipes/index.md): real-world patterns.

## Repository layout { #repository-layout }

```text
easeo/
├── Cargo.toml                 # Rust workspace
├── pyproject.toml             # maturin / Python package
├── README.md
├── LICENSE
├── crates/
│   ├── easeo-core/            # all logic
│   ├── easeo-python/          # PyO3 bindings
│   └── easeo-node/            # napi-rs bindings
├── packages/core/             # @easeo/core wrapper
├── integrations/              # JS framework integrations
├── python/easeo/              # Python package and adapters
├── tests/                     # Python, JavaScript, conformance
└── docs/
    └── (this directory)
```

## License { #license }

MIT.
