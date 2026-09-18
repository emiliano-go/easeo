---
title: "Contracts"
---

# Contracts { #contracts }

An **SEO contract** is a machine-readable description of your SEO intent. It
says which pages must have which metadata. You commit it to the repository and
validate your generated output against it in CI, so an accidental SEO change
fails the build instead of shipping.

## What a contract contains { #what }

A contract describes:

* the site (canonical host and scheme)
* default expectations for every page
* rules that match specific routes
* exceptions for known deviations

Each expectation can require fields, set length bounds, assert a schema type,
and more. See the [API reference](../reference/python-api.md) for the full
`SEOExpectation` field list.

## Build a contract { #build }

=== "Python"

    ```python
    from easeo import SEOContractConfig, build_seo_contract

    contract = build_seo_contract(
        SEOContractConfig(canonical_host="example.com", scheme="https")
    )

    print(contract.to_json())
    ```

=== "JavaScript"

    ```js
    const { buildSeoContract } = require("@easeo/core");

    const contract = buildSeoContract({
      canonicalHost: "example.com",
      scheme: "https",
    });

    console.log(contract.toJSONString());
    ```

Output:

```json
{
  "contract_version": "1",
  "generator": { "name": "easeo", "version": "0.1.0" },
  "site": { "canonical_host": "example.com", "scheme": "https" },
  "defaults": {},
  "rules": [],
  "exceptions": {}
}
```

## Add rules { #rules }

A rule matches a route and applies an expectation.

=== "Python"

    ```python
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
                SEOContractRule(
                    match="/blog/*",
                    expect=SEOExpectation(schema_types=["Article"]),
                ),
                SEOContractRule(
                    match="/search",
                    expect=SEOExpectation(indexable=False),
                ),
            ],
            exceptions={
                "/legal/terms": SEOExpectation(description=SEOExpectation(required=False)),
            },
        )
    )
    ```

=== "JavaScript"

    ```js
    const { buildSeoContract } = require("@easeo/core");

    const contract = buildSeoContract({
      canonicalHost: "example.com",
      scheme: "https",
      defaults: {
        title: { minLength: 20, maxLength: 60 },
        description: { minLength: 70, maxLength: 160 },
        ogRequired: true,
        schemaRequired: true,
      },
      rules: [
        { match: "/blog/*", expect: { schemaTypes: ["Article"] } },
        { match: "/search", expect: { indexable: false } },
      ],
      exceptions: {
        "/legal/terms": { description: { required: false } },
      },
    });
    ```

## Write it to disk { #write }

=== "Python"

    ```python
    contract.write(".easeo/contract.json")
    ```

=== "JavaScript"

    ```js
    const { writeFileSync } = require("node:fs");
    const { mkdirSync } = require("node:fs");

    mkdirSync(".easeo", { recursive: true });
    writeFileSync(".easeo/contract.json", JSON.stringify(contract.toDict(), null, 2));
    ```

The Astro integration emits this file automatically at build time. See
[Contracts in CI](../guides/contracts-in-ci.md) for the full workflow.

## Recap { #recap }

* A contract encodes SEO intent as data.
* Build it with `build_seo_contract` and commit the JSON.
* Validate generated payloads against it in CI.

**Next:** [Configuration](configuration.md#configuration).
