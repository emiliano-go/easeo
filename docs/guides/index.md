---
title: "Guides"
description: "Task-oriented guides for custom schemas, hooks, contracts, and framework recipes."
---

# Guides { #guides }

Task-oriented walkthroughs for the things people do most with easeo.

## Pages { #pages }

* [Custom JSON-LD](custom-schemas.md#custom-jsonld): per-page overrides and
  registered generators.
* [Hooks](hooks.md#hooks): config-scoped post-processing.
* [Contracts in CI](contracts-in-ci.md#contracts-in-ci): enforce SEO intent in
  your pipeline.
* [Framework Recipes](framework-recipes.md#framework-recipes): end-to-end
  patterns per framework.
* [Migration from seoslug](migration-from-seoslug.md#migration-from-seoslug):
  the API mapping and what changed.

## When to use which mechanism { #which }

| Need | Use |
|---|---|
| Change one field on one page | `SEOOverrides` |
| Same schema shape for a whole type | `SchemaRegistry` |
| Add a field to every payload | `HookRegistry` |
| Assert SEO in CI | SEO contract |
| Fix a URL shape globally | `URLPolicy` |
