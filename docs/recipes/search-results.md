---
title: "Search Results"
---

# Recipe: Search Results

A search page should not be indexed, but its query parameters should survive
normalization so links stay shareable.

```python
from easeo import SEOConfig, SEOEntity, URLPolicy, build_seo_payload

config = SEOConfig(
    canonical_host="shop.example.com",
    public_base_url="https://shop.example.com",
    url_policy=URLPolicy(
        enforce_https=True,
        strip_tracking_params=True,
        allowed_query_params=["q", "page"],
    ),
    search_robots=None,  # defaults to noindex,follow
)

entity = SEOEntity(entity_type="search", title="Search")

payload = build_seo_payload(entity, "/search?q=headphones&utm_source=ad", config)
```

Result:

- **robots** → `noindex,follow` (search pages default to this)
- **canonical** → `https://shop.example.com/search?q=headphones`
  (`utm_source` stripped, `q` preserved)
- **schema_jsonld** → `SearchResultsPage`

Override the search robots directive site-wide:

```python
from easeo import Robots

config = SEOConfig(
    ...,
    search_robots=Robots(index=False, follow=True),
)
```
