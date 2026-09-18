---
title: "Examples"
---

# Examples { #examples }

Short, self-contained programs. Each one builds a payload and prints the
generated HTML.

## Python { #python }

```python
from easeo import SEOConfig, SEOEntity, build_seo_payload

config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
)

entity = SEOEntity(
    entity_type="post",
    title="Hello World",
    excerpt="An example post.",
)

payload = build_seo_payload(entity, "/blog/hello", config)
print(payload.render_html())
```

## JavaScript { #javascript }

```js
const { buildSeoPayload } = require("@easeo/core");

const payload = buildSeoPayload(
  { entityType: "post", title: "Hello World", description: "An example post." },
  "/blog/hello",
  { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
);

console.log(payload.renderHtml());
```

## Rust { #rust }

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
println!("{}", payload.render_html()?);
```

## Runnable files { #files }

The repository ships runnable copies under `examples/`:

```text
examples/
├── python_basic.py
└── node_basic.mjs
```

```bash
python examples/python_basic.py
node examples/node_basic.mjs
```

## Determinism demo { #determinism }

```python
from easeo import SEOConfig, SEOEntity, build_seo_payload

config = SEOConfig(canonical_host="example.com", public_base_url="https://example.com")
entity = SEOEntity(entity_type="page", title="Stable")

a = build_seo_payload(entity, "/x", config)
b = build_seo_payload(entity, "/x", config)

assert a == b
assert a.hash() == b.hash()
print("stable:", a.etag())
```

## Contract demo { #contract }

```python
from easeo import SEOContractConfig, SEOExpectation, build_seo_contract

contract = build_seo_contract(
    SEOContractConfig(
        canonical_host="example.com",
        scheme="https",
        defaults=SEOExpectation(og_required=True, schema_required=True),
    )
)

print(contract.to_json())
```
