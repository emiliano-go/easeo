---
title: "Multi Language"
description: "Locale metadata plus per-language canonical URLs and overrides."
---

# Recipe: Multi Language

Locale metadata plus per-language canonical URLs.

```python
from easeo import SEOConfig, SEOEntity, build_seo_payload

config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
    locale="en_US",
    locale_alternate=["es_UY", "pt_BR"],
)

entity = SEOEntity(entity_type="post", title="Hello", excerpt="A post.")

payload = build_seo_payload(entity, "/en/hello", config)
```

Output includes:

```html
<meta property="og:locale" content="en_US">
<meta property="og:locale:alternate" content="es_UY">
<meta property="og:locale:alternate" content="pt_BR">
```

Per-language canonical with overrides:

```python
from easeo import SEOOverrides

payload = build_seo_payload(
    entity,
    "/es/hola",
    {**config, "locale": "es_UY"} if isinstance(config, dict) else config,
    SEOOverrides(meta_title="Hola"),
)
```

In JS, pass overrides as the fourth argument:

```typescript
buildSeoPayload(entity, "/es/hola", config, { metaTitle: "Hola" });
```

For `hreflang` link emission, generate one payload per language and combine
the `canonical` values in your template.
