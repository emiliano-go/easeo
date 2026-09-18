---
title: "First Payload"
---

# First Payload { #first-payload }

By the end of this page you will build an SEO payload, read its fields, and
serialize it. Make sure easeo is installed first.

```bash
pip install easeo
```

## Step 1: Configure the site { #step-1-configure }

`SEOConfig` holds values that are the same for every page on your site. Two
fields are required:

* `canonical_host`: the hostname only, with no scheme and no path.
* `public_base_url`: the absolute base URL used to resolve canonical paths.

=== "Python"

    ```python
    from easeo import SEOConfig

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        site_name="Example",
        title_template="{title} - Example",
    )
    ```

=== "JavaScript"

    ```js
    const config = {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
      siteName: "Example",
      titleTemplate: "{title} - Example",
    };
    ```

## Step 2: Describe the content { #step-2-describe }

`SEOEntity` describes one piece of content. Only `entity_type` is required.

=== "Python"

    ```python
    from easeo import SEOEntity

    entity = SEOEntity(
        entity_type="post",
        title="Introducing easeo",
        excerpt="Deterministic SEO payloads for content platforms.",
    )
    ```

=== "JavaScript"

    ```js
    const entity = {
      entityType: "post",
      title: "Introducing easeo",
      description: "Deterministic SEO payloads for content platforms.",
    };
    ```

Valid entity types are: `home`, `post`, `page`, `video`, `taxonomy`, `search`,
`product`, `organization`, `local_business`, `faq`, and `other`.

## Step 3: Build the payload { #step-3-build }

The one function you need:

=== "Python"

    ```python
    from easeo import build_seo_payload

    payload = build_seo_payload(entity, "/blog/introducing-easeo", config)
    ```

=== "JavaScript"

    ```js
    const { buildSeoPayload } = require("@easeo/core");

    const payload = buildSeoPayload(entity, "/blog/introducing-easeo", config);
    ```

## Step 4: Read the result { #step-4-read }

=== "Python"

    ```python
    print(payload.title)
    # "Introducing easeo - Example"

    print(payload.canonical)
    # "https://example.com/blog/introducing-easeo"

    print(payload.robots)
    # "index,follow"

    print(payload.og.type)
    # "article"

    print(payload.schema_jsonld["@type"])
    # "Article"
    ```

=== "JavaScript"

    ```js
    console.log(payload.title);
    // "Introducing easeo - Example"

    console.log(payload.canonical);
    // "https://example.com/blog/introducing-easeo"

    console.log(payload.openGraph.type);
    // "article"

    console.log(payload.schemaJsonLd["@type"]);
    // "Article"
    ```

## Step 5: Serialize { #step-5-serialize }

=== "Python"

    ```python
    payload.to_dict()      # canonical snake_case dict
    payload.to_json()      # canonical JSON string
    ```

=== "JavaScript"

    ```js
    payload.toDict();          // canonical snake_case object
    payload.toObject();        // camelCase object
    JSON.stringify(payload);   // uses toObject()
    payload.toJSONString();    // canonical JSON string
    ```

## Determinism check { #determinism-check }

Building the same payload twice always yields the same bytes:

=== "Python"

    ```python
    a = build_seo_payload(entity, "/blog/introducing-easeo", config)
    b = build_seo_payload(entity, "/blog/introducing-easeo", config)

    assert a == b
    assert a.hash() == b.hash()
    ```

=== "JavaScript"

    ```js
    const a = buildSeoPayload(entity, "/blog/introducing-easeo", config);
    const b = buildSeoPayload(entity, "/blog/introducing-easeo", config);

    console.assert(a.equals(b));
    console.assert(a.hash() === b.hash());
    ```

## Recap { #recap }

* `SEOConfig` is site-wide; `SEOEntity` is per page.
* `build_seo_payload(entity, route, config)` returns an `SEOPayload`.
* The payload is structured, hashable, and serializable.

**Next:** [Fallback and Overrides](fallback-and-overrides.md#fallback-and-overrides).
