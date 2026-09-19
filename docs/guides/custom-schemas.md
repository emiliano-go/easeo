---
title: "Custom JSON-LD"
description: "Control structured data with per-page overrides, registered generators, and hooks."
---

# Custom JSON-LD { #custom-jsonld }

There are two ways to control structured data with easeo: per-page overrides
and registered generators. This guide shows when to use each.

## Per-page override { #override }

Use `SEOOverrides` when a single page needs a different schema:

=== "Python"

    ```python
    from easeo import SEOOverrides, build_seo_payload

    payload = build_seo_payload(
        entity,
        "/podcast/ep-1",
        config,
        SEOOverrides(schema_jsonld={
            "@context": "https://schema.org",
            "@type": "PodcastEpisode",
            "name": "Episode 1",
            "url": "https://example.com/podcast/ep-1",
        }),
    )
    ```

=== "JavaScript"

    ```js
    const { buildSeoPayload } = require("@easeo/core");

    const payload = buildSeoPayload(entity, "/podcast/ep-1", config, {
      schemaJsonLd: {
        "@context": "https://schema.org",
        "@type": "PodcastEpisode",
        name: "Episode 1",
      },
    });
    ```

Overrides win over every other source and can be a single object or a list of
objects.

## Registered generator { #registry }

When every page of a given schema type should use the same shape, register a
generator once on the config:

=== "Python"

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

=== "JavaScript"

    ```js
    const { SchemaRegistry } = require("@easeo/core");

    const registry = new SchemaRegistry();

    registry.register("Article", (entity, config, canonical, title) => ({
      "@context": "https://schema.org",
      "@type": "PodcastEpisode",
      name: title,
      url: canonical,
    }));

    const config = {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
      schemaRegistry: registry,
    };
    ```

The generator receives `(entity, config, canonical, title, description,
og_image)` and returns a dict. Return `None` to fall back to the built-in
schema.

### Managing generators { #manage }

| Python | JavaScript | Purpose |
|---|---|---|
| `register` | `register` | Add or replace a generator |
| `unregister` | `unregister` | Remove a generator |
| `get` | `get` | Look up a generator |
| `has` | `has` | Check whether one is registered |
| `list_types` | `listTypes` | List registered type names |

## Site-wide injection with a hook { #hooks }

To add an `Organization` schema to every page, use a hook:

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

## Precedence { #precedence }

1. `omit_schema` produces `None`.
2. `SEOOverrides.schema_jsonld`.
3. A registered generator matching the resolved `@type`.
4. The auto-generated schema.

Breadcrumbs are appended in every case, and hooks run last.

## Recap { #recap }

* Use `SEOOverrides` for one page.
* Use `SchemaRegistry` for a whole schema type.
* Use a hook to inject fields into every payload.
