"""Schema registry — custom JSON-LD generators, registered from Python.

seoslug parity: register a callable for any schema.org type. When the
resolved schema type matches a registered generator, its output replaces
the built-in schema for that page.

The registry lives on the ``SEOConfig`` it is attached to (like hooks), so
``build_seo_payload`` stays a pure function of its inputs.

Usage::

    from easeo import SEOConfig, SchemaRegistry, build_seo_payload

    registry = SchemaRegistry()

    @registry.register("Podcast")
    def podcast(entity, config, canonical, title, description, og_image):
        return {
            "@context": "https://schema.org",
            "@type": "Podcast",
            "name": title,
            "url": canonical,
        }

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
        schema_registry=registry,
    )

Generators receive ``(entity, config, canonical, title, description, og_image)``
and must return a dict (or ``None`` to fall back to the built-in schema).
"""

from __future__ import annotations

import inspect
from typing import TYPE_CHECKING, Any, Callable

if TYPE_CHECKING:
    from easeo._easeo_native import SEOConfig, SEOEntity

SchemaGenerator = Callable[..., "dict | None"]

__all__ = ["SchemaRegistry"]


class SchemaRegistry:
    """Registry of custom JSON-LD generators keyed by schema type name."""

    def __init__(self) -> None:
        """Creates an empty registry."""
        self._generators: dict[str, SchemaGenerator] = {}

    def register(
        self, schema_type: str | SchemaGenerator, fn: SchemaGenerator | None = None
    ) -> Any:
        """Registers ``fn`` for ``schema_type``.

        Usable as a decorator (``@registry.register("Podcast")``), a bare
        decorator (``@registry.register``), or a plain call
        (``registry.register("Podcast", fn)``).

        Args:
            schema_type: Schema type name, or the generator itself when used
                as a bare decorator.
            fn: Generator to register when called directly.

        Returns:
            The generator when called directly or as a decorator, or a
            decorator when called with only a type name.

        Raises:
            ValueError: If the type name is empty or the generator is not
                callable.
        """
        # Plain call: register("Podcast", fn)
        if fn is not None:
            if not isinstance(schema_type, str) or not schema_type.strip():
                raise ValueError("schema_type must be a non-empty string")
            if not callable(fn):
                raise ValueError("schema generator must be callable")
            self._generators[schema_type.strip()] = fn
            return fn

        # Bare decorator: @registry.register  (schema_type is the function)
        if callable(schema_type):
            type_name = getattr(schema_type, "__name__", "")
            if not type_name:
                raise ValueError("register() requires an explicit schema type name")
            self._generators[type_name] = schema_type
            return schema_type

        # Decorator with an argument: @registry.register("Podcast")
        if not isinstance(schema_type, str) or not schema_type.strip():
            raise ValueError("schema_type must be a non-empty string")
        type_name = schema_type.strip()

        def decorator(generator: SchemaGenerator) -> SchemaGenerator:
            if not callable(generator):
                raise ValueError("schema generator must be callable")
            self._generators[type_name] = generator
            return generator

        return decorator

    def unregister(self, schema_type: str) -> None:
        """Removes a previously registered generator.

        Args:
            schema_type: Schema type name to remove. No-op when absent.
        """
        self._generators.pop(schema_type.strip(), None)

    def get(self, schema_type: str) -> SchemaGenerator | None:
        """Looks up a generator by schema type name.

        Args:
            schema_type: Schema type name.

        Returns:
            The generator, or ``None`` when not registered.
        """
        return self._generators.get(schema_type.strip())

    def has(self, schema_type: str) -> bool:
        """Returns whether a generator is registered for ``schema_type``."""
        return schema_type.strip() in self._generators

    def list_types(self) -> list[str]:
        """Returns the registered schema type names, sorted."""
        return sorted(self._generators)

    @property
    def types(self) -> list[str]:
        """Registered schema type names, sorted."""
        return self.list_types()

    def generate(
        self,
        schema_type: str,
        entity: "SEOEntity",
        config: "SEOConfig",
        canonical: str,
        title: str,
        description: str | None,
        og_image: str | None,
    ) -> dict | None:
        """Runs the generator registered for ``schema_type``, if any.

        Args:
            schema_type: Schema type name to match.
            entity: Entity being processed.
            config: Configuration in use.
            canonical: Resolved canonical URL.
            title: Resolved title.
            description: Resolved description.
            og_image: Resolved Open Graph image URL.

        Returns:
            The generated schema, or ``None`` when no generator is registered
            or the generator returns ``None``.

        Raises:
            ValueError: If the generator returns something other than a dict
                or ``None``.
        """
        generator = self._generators.get(schema_type)
        if generator is None:
            return None
        result = generator(entity, config, canonical, title, description, og_image)
        if result is None:
            return None
        if not isinstance(result, dict):
            raise ValueError(
                f"schema generator for '{schema_type}' must return a dict or None, "
                f"got {type(result).__name__}"
            )
        return result

    def __len__(self) -> int:
        """Returns the number of registered generators."""
        return len(self._generators)
