---
title: "Fallback and Overrides"
---

# Fallback and Overrides { #fallback-and-overrides }

Every field in the payload resolves through a priority chain. The first
non-empty value wins. This page shows how to control that chain.

You do not need to set every field on every entity. Set sensible defaults in
the config, override per entity for most fields, and override per call for
edge cases.

## The precedence order { #precedence }

1. **`SEOOverrides`**: per-call overrides (highest priority).
2. **`SEOEntity`**: content fields.
3. **`SEOConfig`**: site-wide defaults.
4. **Hardcoded defaults**: library fallbacks (lowest priority).

## Setting a config default { #config-default }

A site-wide fallback image applies to every page that has no image of its own:

=== "Python"

    ```python
    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        default_og_image="https://cdn.example.com/default.jpg",
    )
    ```

=== "JavaScript"

    ```js
    const config = {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
      defaultOgImage: "https://cdn.example.com/default.jpg",
    };
    ```

## Overriding per entity { #entity-override }

A featured image on the entity beats the config default:

=== "Python"

    ```python
    from easeo import SEOImage

    entity = SEOEntity(
        entity_type="post",
        title="Introducing easeo",
        featured_image=SEOImage(
            url="https://cdn.example.com/hero.jpg",
            width=1200,
            height=630,
            alt="Hero",
        ),
    )
    ```

=== "JavaScript"

    ```js
    const entity = {
      entityType: "post",
      title: "Introducing easeo",
      image: "https://cdn.example.com/hero.jpg",
      imageWidth: 1200,
      imageHeight: 630,
      imageAlt: "Hero",
    };
    ```

## Overriding per call { #call-override }

`SEOOverrides` wins over both the entity and the config. Use it for one-off
pages, campaign tags, or a locked-down title.

=== "Python"

    ```python
    from easeo import SEOOverrides, build_seo_payload

    payload = build_seo_payload(
        entity,
        "/blog/introducing-easeo",
        config,
        SEOOverrides(
            meta_title="Introducing easeo (launch edition)",
            twitter_creator="@easeo",
            skip_title_template=True,
        ),
    )
    ```

=== "JavaScript"

    ```js
    const payload = buildSeoPayload(
      entity,
      "/blog/introducing-easeo",
      config,
      {
        metaTitle: "Introducing easeo (launch edition)",
        twitterCreator: "@easeo",
        skipTitleTemplate: true,
      }
    );
    ```

## A worked example { #worked-example }

```python
config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
    default_og_image="https://cdn.example.com/default.jpg",
)

entity = SEOEntity(
    entity_type="post",
    title="Post title",
    featured_image=SEOImage(url="https://cdn.example.com/hero.jpg"),
)

overrides = SEOOverrides(og_image=SEOImage(url="https://cdn.example.com/urgent.jpg"))

payload = build_seo_payload(entity, "/post", config, overrides)
assert payload.og.image == "https://cdn.example.com/urgent.jpg"
```

The `og:image` resolution ran `overrides.og_image` first, found a value, and
stopped. Without the override it would have used the entity image; without
that, the config default.

## Field-by-field chains { #chains }

The full chain for every field, including the entity-status rules for robots
and the cascade from Open Graph to Twitter, is in
[Fallback Chains](../concepts/fallback-chains.md).

## Recap { #recap }

* Resolution is Overrides > Entity > Config > default.
* Use config defaults for site-wide values.
* Use entity fields for content-specific values.
* Use `SEOOverrides` for per-call edge cases.

**Next:** [Rendering HTML](rendering.md#rendering-html).
