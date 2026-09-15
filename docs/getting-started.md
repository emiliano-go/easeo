# Getting Started with easeo

## Installation

### Python

```bash
pip install easeo
```

### JavaScript/TypeScript

```bash
npm install @easeo/core
```

## Quick Start

### Python

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

### JavaScript/TypeScript

```typescript
import { buildSeoPayload } from "@easeo/core";

const payload = buildSeoPayload(
  { entityType: "post", title: "Hello World", description: "An example post." },
  "/blog/hello",
  { publicBaseUrl: "https://example.com" }
);

console.log(payload.renderHtml());
```

## Next Steps

- [Python API](python-api.md)
- [JavaScript API](javascript-api.md)
- [Contracts](contracts.md)
