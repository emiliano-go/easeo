---
title: "Custom Schema"
description: "Two ways to control structured data: per-page overrides and registered generators."
---

# Recipe: Custom JSON-LD

Two ways to control structured data: per-page overrides, or a registered
generator for an entire schema type.

## Per-page override

```python
from easeo import SEOConfig, SEOEntity, SEOOverrides, build_seo_payload

config = SEOConfig(canonical_host="example.com", public_base_url="https://example.com")
entity = SEOEntity(entity_type="page", title="Episode 1")

payload = build_seo_payload(
    entity,
    "/podcast/1",
    config,
    SEOOverrides(schema_jsonld={
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        "name": "Episode 1",
    }),
)
```

## Registered generator

When every page of a given schema type should use the same shape, register a
generator once. It runs whenever the resolved `@type` matches.

```python
from easeo import SEOConfig, SchemaRegistry

registry = SchemaRegistry()

@registry.register("Article")
def podcast_episode(entity, config, canonical, title, description, og_image):
    return {
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        "name": title,
        "url": canonical,
        "description": description,
    }

config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
    schema_registry=registry,
)
```

Return `None` to fall back to the built-in schema for that page.

## Site-wide JSON-LD with hooks

To inject an `Organization` schema on *every* page, use a hook:

```python
from easeo import HookRegistry

hooks = HookRegistry()

@hooks.hook("post_process")
def inject_organization(payload, entity, config):
    org = {
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": config.publisher_name or "Example",
        "url": config.public_base_url,
    }
    existing = payload.get("schema_jsonld")
    if isinstance(existing, list):
        payload["schema_jsonld"] = [org, *existing]
    elif existing is not None:
        payload["schema_jsonld"] = [org, existing]
    else:
        payload["schema_jsonld"] = org
    return payload

config = SEOConfig(..., hooks=hooks)
```
