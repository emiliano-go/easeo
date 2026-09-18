"""Config-scoped hooks.

Hooks let you post-process the SEO payload after it is built — injecting
site-wide JSON-LD, normalizing a description, adding a custom tag.

Unlike seoslug there is **no global mutable registry**: hooks live on the
``HookRegistry`` you attach to an ``SEOConfig``. That keeps
``build_seo_payload`` a pure function of its inputs (config included), which
is the whole point of easeo. Two configs can carry different hooks without
interfering.

Usage::

    from easeo import SEOConfig, HookRegistry, build_seo_payload

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

Hooks receive the payload as a plain ``dict`` and must return it. They run
in registration order; the payload passed to each hook includes the changes
made by the previous one. A hook that raises stops the chain immediately.
"""

from __future__ import annotations

import functools
import threading
from collections.abc import Callable
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from easeo._easeo_native import SEOConfig, SEOEntity

HookFunc = Callable[[dict, "SEOEntity", "SEOConfig"], dict]

__all__ = ["HookRegistry", "run_hooks"]


class HookRegistry:
    """Instance-scoped, thread-safe hook registry.

    Attach one to an ``SEOConfig`` via ``SEOConfig(..., hooks=registry)``.
    """

    def __init__(self) -> None:
        self._hooks: dict[str, list[HookFunc]] = {}
        self._lock = threading.Lock()

    def register(self, name: str, fn: HookFunc) -> None:
        """Register *fn* to run when hook *name* is triggered."""
        if not isinstance(name, str) or not name.strip():
            raise ValueError("hook name must be a non-empty string")
        if not callable(fn):
            raise ValueError("hook function must be callable")
        with self._lock:
            self._hooks.setdefault(name, []).append(fn)

    def hook(self, name: str) -> Callable[[HookFunc], HookFunc]:
        """Decorator form of :meth:`register`."""

        def decorator(fn: HookFunc) -> HookFunc:
            self.register(name, fn)

            @functools.wraps(fn)
            def wrapper(payload: dict, entity: "SEOEntity", config: "SEOConfig") -> dict:
                return fn(payload, entity, config)

            return wrapper

        return decorator

    def unregister(self, name: str, fn: HookFunc) -> None:
        """Remove a previously registered hook (no-op if absent)."""
        with self._lock:
            hooks = self._hooks.get(name)
            if hooks and fn in hooks:
                hooks.remove(fn)

    def run(self, name: str, payload: dict, entity: "SEOEntity", config: "SEOConfig") -> dict:
        """Run all hooks registered under *name*, in registration order."""
        with self._lock:
            hooks = list(self._hooks.get(name, []))
        for fn in hooks:
            payload = fn(payload, entity, config)
        return payload

    def clear(self, name: str | None = None) -> None:
        """Remove all hooks, or only those under *name*."""
        with self._lock:
            if name is not None:
                self._hooks.pop(name, None)
            else:
                self._hooks.clear()

    def get_registered(self) -> dict[str, list[HookFunc]]:
        """Return a copy of all registered hooks (for inspection / tests)."""
        with self._lock:
            return {k: list(v) for k, v in self._hooks.items()}

    def __len__(self) -> int:
        with self._lock:
            return sum(len(v) for v in self._hooks.values())


def run_hooks(
    name: str,
    payload: dict[str, Any],
    entity: "SEOEntity",
    config: "SEOConfig",
) -> dict[str, Any]:
    """Run the config-scoped hooks for *name*.

    Returns the payload unchanged when the config has no hooks.
    """
    registry = getattr(config, "hooks", None)
    if registry is None:
        return payload
    return registry.run(name, payload, entity, config)
