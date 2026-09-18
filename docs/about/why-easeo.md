---
title: "Why easeo"
---

# Why easeo { #why-easeo }

Most SEO libraries do too much. They score content, analyze keywords, rewrite
descriptions, and pull in a browser engine. easeo does one thing: **generate
deterministic SEO metadata from content entities**.

## The problem with generated metadata { #problem }

SEO metadata is usually assembled ad hoc: a title here, an Open Graph block
there, a JSON-LD template somewhere else. It drifts. Nothing tests it. When a
deploy changes a canonical URL, no one notices until rankings move.

The root cause is that the output is not treated like code. It has no
deterministic contract, so it cannot be snapshotted or diffed.

## The easeo answer { #answer }

Turn the metadata into a pure function with a stable output.

```text
SEOEntity + route + SEOConfig -> SEOPayload
```

Same inputs, same bytes, every time. No timestamps, no randomness, no
environment reads, no hidden I/O. That makes the output:

- **snapshot testable**: assert against a committed fixture
- **hashable**: generate stable ETags
- **cacheable**: memoize without invalidation logic
- **diffable**: compare staging and production
- **CI-validatable**: commit SEO intent as a contract

## Why Rust { #rust }

The core is Rust so the same behavior ships to Python and JavaScript, not two
implementations that drift apart. The bindings are thin; all logic lives in one
place. Cross-language conformance tests assert that Python and Rust produce
byte-identical output.

## What it is not { #not }

- Not a crawler
- Not a scorer
- Not a keyword tool
- Not a browser automation framework
- Not an analytics platform

## Design principles { #principles }

1. **Deterministic**: same input, same output.
2. **Pure**: no network, no I/O, no randomness, no environment reads.
3. **Framework-agnostic**: the core knows nothing about React or Django.
4. **Minimal surface**: one primary function.
5. **Contract-first**: SEO intent is machine-readable and testable.
6. **Zero ceremony**: adapters are plug-and-play.
