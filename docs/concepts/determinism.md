---
title: "Determinism"
---

# Determinism { #determinism }

Identical inputs always produce identical outputs. Always.

=== "Python"

    ```python
    p1 = build_seo_payload(entity, "/blog/post", config)
    p2 = build_seo_payload(entity, "/blog/post", config)
    assert p1 == p2
    ```

=== "JavaScript"

    ```js
    const p1 = buildSeoPayload(entity, "/blog/post", config);
    const p2 = buildSeoPayload(entity, "/blog/post", config);
    console.assert(p1.equals(p2));
    ```

## What is forbidden in the output { #forbidden }

* Current timestamp
* Random UUID
* Unordered serialization
* Environment-dependent values
* Hash maps in place of ordered maps

Mapping output uses `BTreeMap`, so key order is sorted and stable. JSON-LD
objects, Open Graph, and the canonical dict all serialize identically across
runs.

## What this enables { #enables }

| Capability | How it works |
|---|---|
| Snapshot testing | Commit expected payloads and assert equality in tests |
| CI validation | A changed payload fails the build instead of shipping |
| Caching | `@lru_cache` on `build_seo_payload` is safe forever |
| Content-addressed artifacts | `payload.hash()` is stable across machines |
| Deployment diffs | Compare staging and production payloads to find drift |

## Equality and hashing { #equality }

Python payloads compare against other payloads and against plain dicts:

```python
assert payload == other_payload
assert payload == payload.to_dict()
```

Both languages expose a stable SHA-256 hash and an HTTP ETag:

=== "Python"

    ```python
    payload.hash()   # 64 hex characters
    payload.etag()   # '"<hash>"'
    ```

=== "JavaScript"

    ```js
    payload.hash();
    payload.etag();
    ```

## Hooks and determinism { #hooks }

easeo allows post-processing through config-scoped hooks. Because the hooks
registry is part of the `SEOConfig`, it is an ordinary input: the same config
produces the same output every time. There is no global mutable registry, so
two configs in the same process cannot interfere.

!!! note "Determinism is a property of your hooks too"

    A hook that reads the clock or a random source breaks determinism for the
    config that carries it. Keep hooks pure.

## Recap { #recap }

* Same inputs, same bytes, everywhere.
* Ordered maps and no ambient state are what make it true.
* This is the foundation for snapshot testing, caching, and CI diffs.
