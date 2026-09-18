"""Async SEO payload builder.

Offloads the synchronous ``build_seo_payload`` call to a thread pool so it
does not block the event loop in async frameworks (FastAPI, Starlette,
Litestar, Quart, ...).

The Rust core releases the GIL, so this genuinely parallelizes.
"""

from __future__ import annotations

import asyncio
from concurrent.futures import ThreadPoolExecutor
from typing import TYPE_CHECKING

from easeo._easeo_native import build_seo_payload

if TYPE_CHECKING:
    from easeo._easeo_native import SEOConfig, SEOEntity, SEOOverrides, SEOPayload

__all__ = ["build_seo_payload_async", "set_executor"]

_default_executor: ThreadPoolExecutor | None = None


def _get_executor(max_workers: int = 4) -> ThreadPoolExecutor:
    global _default_executor
    if _default_executor is None:
        _default_executor = ThreadPoolExecutor(max_workers=max_workers)
    return _default_executor


def set_executor(executor: ThreadPoolExecutor | None) -> None:
    """Override the default thread pool executor (``None`` resets it)."""
    global _default_executor
    _default_executor = executor


async def build_seo_payload_async(
    entity: "SEOEntity",
    route: str,
    config: "SEOConfig",
    overrides: "SEOOverrides | None" = None,
    executor: ThreadPoolExecutor | None = None,
) -> "SEOPayload":
    """Async version of :func:`easeo.build_seo_payload`."""
    loop = asyncio.get_running_loop()
    ex = executor if executor is not None else _get_executor()
    return await loop.run_in_executor(ex, build_seo_payload, entity, route, config, overrides)
