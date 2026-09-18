---
title: "Concepts"
---

# Concepts { #concepts }

This track explains how easeo is put together and why. Read it once and the
API becomes predictable: there are no hidden states, no environment reads, and
no surprises in the output.

## Pages { #pages }

* [Determinism](determinism.md#determinism): the core guarantee and what it
  enables.
* [Entity Model](entity-model.md#entity-model): what a content entity is and
  which fields feed which output.
* [Payload Model](payload-model.md#payload-model): the shape of the output and
  the exact tag order.
* [Fallback Chains](fallback-chains.md#fallback-chains): how every field
  resolves.
* [URL Normalization](url-normalization.md#url-normalization): the canonical
  URL pipeline.
* [JSON-LD Schemas](schemas.md#schemas): the built-in schema types and how to
  extend them.
* [Validation](validation.md#validation): the built-in best-practice checks.

## The one-sentence model { #model }

`build_seo_payload` is a pure function:

```text
SEOEntity + route + SEOConfig (+ SEOOverrides) -> SEOPayload
```

Everything else in easeo is either a value type that feeds that function or a
convenience wrapper around it.
