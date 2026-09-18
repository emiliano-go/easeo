---
title: "Contracts in CI"
---

# Contracts in CI { #contracts-in-ci }

An SEO contract turns intent into a committed artifact. You generate it once,
commit it, and validate your build against it. When SEO drifts, CI fails.

## The workflow { #workflow }

1. Define the contract in code.
2. Write it to `.easeo/contract.json` and commit the file.
3. In CI, regenerate and diff, or validate generated payloads against it.

## Define and emit { #emit }

=== "Python"

    ```python
    # scripts/write_contract.py
    from easeo import (
        SEOContractConfig,
        SEOContractRule,
        SEOExpectation,
        build_seo_contract,
    )

    contract = build_seo_contract(
        SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
            defaults=SEOExpectation(
                title=SEOExpectation(min_length=20, max_length=60),
                description=SEOExpectation(min_length=70, max_length=160),
                og_required=True,
                schema_required=True,
            ),
            rules=[
                SEOContractRule(match="/blog/*", expect=SEOExpectation(schema_types=["Article"])),
                SEOContractRule(match="/search", expect=SEOExpectation(indexable=False)),
            ],
        )
    )

    contract.write(".easeo/contract.json")
    ```

=== "Astro"

    The Astro integration emits the contract automatically when you pass a
    `contract` option:

    ```js
    // astro.config.mjs
    import easeo from "@easeo/astro";

    export default defineConfig({
      integrations: [
        easeo({
          config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
          contract: { canonicalHost: "example.com", scheme: "https" },
        }),
      ],
    });
    ```

    The file is written to `<outDir>/.easeo/contract.json` after the build.

## Validate in CI { #validate }

A compact pipeline step that regenerates the contract and fails on drift:

```yaml
# .github/workflows/seo.yml
name: SEO
on: [push, pull_request]

jobs:
  contract:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: pip install easeo
      - name: Regenerate the contract
        run: python scripts/write_contract.py
      - name: Fail on drift
        run: git diff --exit-code .easeo/contract.json
```

If the contract changes, the diff is printed and the step fails, so a reviewer
sees exactly what changed.

## Asserting payloads { #assert }

Contracts are the site-wide gate; unit tests catch per-page regressions.
Because payloads are deterministic and comparable, a snapshot test is one
line:

```python
def test_blog_post_seo():
    payload = build_seo_payload(blog_entity, "/blog/post", config)
    assert payload == expected_payload  # committed fixture
```

## Recap { #recap }

* Emit the contract to `.easeo/contract.json` and commit it.
* Regenerate and `git diff --exit-code` in CI to catch drift.
* Snapshot individual payloads with `assert payload == fixture`.
