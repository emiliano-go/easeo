---
title: "Validation"
description: "The built-in validation rules, issue shape, and how to surface warnings."
---

# Validation { #validation }

`validate_payload` checks a built payload against SEO best practices and
returns a list of issues. It never throws on ordinary content; it reports.

## Running it { #running }

=== "Python"

    ```python
    from easeo import build_seo_payload, validate_payload

    payload = build_seo_payload(entity, "/blog/post", config)

    for issue in validate_payload(payload):
        print(issue.rule_id, issue.severity, issue.message)
    ```

=== "JavaScript"

    ```js
    const { buildSeoPayload, validatePayload } = require("@easeo/core");

    const payload = buildSeoPayload(entity, "/blog/post", config);

    for (const issue of validatePayload(payload)) {
      console.log(issue.ruleId, issue.severity, issue.message);
    }
    ```

## Issue shape { #shape }

| Field | Type | Description |
|---|---|---|
| `rule_id` / `ruleId` | `str` | Stable rule identifier, e.g. `EASEO101` |
| `severity` | `str` | `error`, `warning`, or `info` |
| `message` | `str` | Human-readable explanation |
| `url` | `str \| None` | Canonical URL the issue belongs to |
| `details` | `dict` | Rule-specific data |

## Rules { #rules }

| Rule | Severity | Checks |
|---|---|---|
| `EASEO101` | warning | Title is missing or still the default `"Untitled"` |
| `EASEO102` | warning | Title is longer than 60 characters |
| `EASEO103` | warning | Meta description is missing |
| `EASEO104` | warning | Meta description is longer than 160 characters |
| `EASEO105` | warning | Canonical URL is not absolute |
| `EASEO106` | warning | Open Graph image URL is not absolute |
| `EASEO107` | warning | Robots directive is not a known directive |
| `EASEO108` | warning | Open Graph image is missing (set `default_og_image` or an entity image) |

## Emitting warnings automatically { #emit }

Set `emit_warnings=True` on the config and easeo emits Python warnings during
`build_seo_payload`, so you can see issues without a second call:

```python
config = SEOConfig(
    canonical_host="example.com",
    public_base_url="https://example.com",
    emit_warnings=True,
)
```

## Validation in CI { #ci }

For build-time enforcement, prefer an SEO contract. Contracts describe intent
per route and fail the build on any violation. See
[Contracts in CI](../guides/contracts-in-ci.md).

## Recap { #recap }

* `validate_payload` reports best-practice issues.
* Issues carry a stable rule id, severity, and details.
* `emit_warnings=True` surfaces them during the build.
