---
title: "Hooks"
description: "Config-scoped hooks that post-process the payload while staying deterministic."
---

# Hooks { #hooks }

Hooks post-process the payload after it is built. Use them to add a field to
every page, rewrite a description per section, or inject site-wide metadata.

Hooks are **config-scoped**. They live on the `SEOConfig` that carries them,
so `build_seo_payload` stays a pure function of its inputs and two configs in
the same process cannot interfere.

## Registering a hook { #register }

=== "Python"

    ```python
    from easeo import HookRegistry, SEOConfig

    hooks = HookRegistry()

    @hooks.hook("post_process")
    def add_generator(payload, entity, config):
        payload["generator"] = "easeo"
        return payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        hooks=hooks,
    )
    ```

=== "JavaScript"

    ```js
    const { HookRegistry } = require("@easeo/core");

    const hooks = new HookRegistry();

    hooks.register("post_process", (payload, entity, config) => {
      payload.generator = "easeo";
      return payload;
    });

    const config = {
      canonicalHost: "example.com",
      publicBaseUrl: "https://example.com",
      hooks,
    };
    ```

## The hook signature { #signature }

A hook receives three arguments and must return the payload:

```text
hook(payload, entity, config) -> payload
```

* `payload` is a plain dict in the canonical snake_case format, including the
  changes made by previous hooks.
* `entity` is the original `SEOEntity`.
* `config` is the `SEOConfig` that carried the hook.

## Hook points { #points }

| Name | When it runs |
|---|---|
| `post_process` | At the end of the build, before returning |

`post_process` is the only built-in hook point.

## Order and scoping { #order }

Hooks run in registration order; the last writer of a field wins. Because the
registry is part of the config, hooks are scoped: a config without hooks is
unaffected.

=== "Python"

    ```python
    hooks_a = HookRegistry()
    hooks_a.register("post_process", lambda p, e, c: {**p, "site": "A"})

    hooks_b = HookRegistry()
    hooks_b.register("post_process", lambda p, e, c: {**p, "site": "B"})

    a = build_seo_payload(entity, "/x", config_a)  # config_a has hooks_a
    b = build_seo_payload(entity, "/x", config_b)  # config_b has hooks_b
    assert a["site"] == "A"
    assert b["site"] == "B"
    ```

## Managing hooks { #manage }

| Python | JavaScript | Purpose |
|---|---|---|
| `register` | `register` | Add a hook |
| `hook` | `hook` | Decorator form |
| `unregister` | `unregister` | Remove a hook |
| `run` | `run` | Run all hooks for a name |
| `clear` | `clear` | Remove all hooks, or those under a name |
| `get_registered` | `size` | Inspect the registry |

## Determinism and purity { #purity }

Keep hooks pure: no clock reads, no random values, no network calls. A hook
that reads the environment breaks the determinism guarantee for the config
that carries it.

If a hook raises, the exception propagates and the remaining hooks are
skipped. Errors should be loud; a failing hook is a bug.

## Recap { #recap }

* Hooks post-process the payload and return it.
* They are config-scoped, ordered, and deterministic when kept pure.
* Use `SEOOverrides` for per-page changes and hooks for site-wide ones.
