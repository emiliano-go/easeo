---
title: "Validation"
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
| `EASEO101` | warning | Title length outside the recommended range |
| `EASEO102` | warning | Description length outside the recommended range |
| `EASEO103` | error | Canonical URL is not absolute, or uses a non-HTTP scheme |
| `EASEO104` | warning | Open Graph image is missing |
| `EASEO105` | warning | Open Graph image URL is not absolute |
| `EASEO106` | warning | Robots directive is malformed |
| `EASEO107` | info | No JSON-LD schema was generated |

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
