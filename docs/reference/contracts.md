---
title: "Contracts"
---

# SEO Contracts { #seo-contracts }

easeo supports machine-readable SEO contracts that describe what a site's SEO should look like.

## Contract Format

```json
{
  "contract_version": "1",
  "generator": { "name": "easeo", "version": "0.1.0" },
  "site": { "canonical_host": "example.com", "scheme": "https" },
  "defaults": {
    "indexable": true,
    "canonical": "self",
    "title": { "required": true },
    "description": { "required": true }
  },
  "rules": [
    {
      "match": "/blog/*",
      "expect": {
        "schema": { "required": true, "types": ["Article"] }
      }
    }
  ]
}
```

## Usage

### Python

```python
from easeo import SEOContractConfig, build_seo_contract

config = SEOContractConfig(
    canonical_host="example.com",
    scheme="https",
)

contract = build_seo_contract(config)
contract.write("dist/.easeo/contract.json")
```

### JavaScript

```typescript
import { buildSeoContract } from "@easeo/core";

const contract = buildSeoContract({ canonicalHost: "example.com" });
```

## With cheseo

When `.easeo/contract.json` exists in your build output, `cheseo ./dist` automatically validates against it.
