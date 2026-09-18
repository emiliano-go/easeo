---
title: "Errors"
---

# Errors { #errors }

easeo uses one error hierarchy across all three languages. In Python, every
error also inherits from `ValueError`, so existing code that catches
`ValueError` keeps working.

## The hierarchy { #hierarchy }

```text
EaseoError
├── InvalidUrlError
├── ConfigurationError
├── EntityError
├── SchemaError
└── ContractError
```

| Type | Raised when |
|---|---|
| `EaseoError` | Base class; also used for serialization failures |
| `InvalidUrlError` | A URL is malformed or a URL policy is invalid |
| `ConfigurationError` | A config or URL policy value fails validation |
| `EntityError` | An entity or overrides value fails validation |
| `SchemaError` | JSON-LD construction fails |
| `ContractError` | Contract generation fails |

## Catching { #catching }

=== "Python"

    ```python
    from easeo import ConfigurationError, EaseoError

    try:
        SEOConfig(canonical_host="", public_base_url="https://example.com")
    except ConfigurationError as err:
        print("bad config:", err)
    except EaseoError as err:
        print("some other easeo error:", err)

    # ValueError also catches every easeo error:
    try:
        SEOConfig(canonical_host="", public_base_url="https://example.com")
    except ValueError as err:
        print("caught as ValueError:", err)
    ```

=== "JavaScript"

    ```js
    const { buildSeoPayload, ConfigurationError, EaseoError } = require("@easeo/core");

    try {
      buildSeoPayload({ entityType: "page" }, "/x", {
        canonicalHost: "",
        publicBaseUrl: "",
      });
    } catch (err) {
      if (err instanceof ConfigurationError) {
        console.log("bad config:", err.code); // "EASEO_CONFIGURATION"
      } else if (err instanceof EaseoError) {
        console.log("other easeo error:", err.message);
      }
    }
    ```

## JavaScript error codes { #codes }

Every error class carries a stable `code`:

| Class | Code |
|---|---|
| `EaseoError` | `EASEO_ERROR` |
| `InvalidUrlError` | `EASEO_INVALID_URL` |
| `ConfigurationError` | `EASEO_CONFIGURATION` |
| `EntityError` | `EASEO_ENTITY` |
| `SchemaError` | `EASEO_SCHEMA` |
| `ContractError` | `EASEO_CONTRACT` |

## Argument errors { #arguments }

In JavaScript, a missing or wrong-typed argument raises `TypeError` with a
message naming the function and parameter:

```text
normalizePath: expected 'path' to be a string, received type undefined
```

Python raises `TypeError` for the same class of mistake.

## Recap { #recap }

* One hierarchy, five concrete types, one base.
* Python errors are also `ValueError`.
* JavaScript errors carry a stable `code`.
