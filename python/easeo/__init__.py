"""easeo — Deterministic SEO payload generation for content platforms.

This package provides Python bindings for the easeo Rust core.
"""

__version__ = "0.1.0"

from easeo._easeo_native import (
    # Types
    SEOConfig,
    SEOEntity,
    SEOPayload,
    SEOOverrides,
    SEOImage,
    SEOAuthor,
    OGPayload,
    TwitterPayload,
    Breadcrumb,
    FAQItem,
    Robots,
    URLPolicy,
    SEOContract,
    SEOContractConfig,
    SEOContractRule,
    SEOExpectation,
    SEOIssue,
    # Functions
    build_seo_payload,
    build_seo_payload_with_overrides,
    build_seo_payload_dict,
    build_seo_contract,
    validate_payload,
    normalize_path_fn as normalize_path,
    normalize_public_url_fn as normalize_public_url,
    clean_url_fn as clean_url,
    clean_query_fn as clean_query,
)

from easeo.async_builder import build_seo_payload_async, set_executor
from easeo.builder import SEOEntityBuilder
from easeo.exceptions import (
    ConfigurationError,
    ContractError,
    EaseoError,
    EntityError,
    InvalidUrlError,
    SchemaError,
)
from easeo.factories import from_blog_post, from_faq, from_product
from easeo.hooks import HookRegistry
from easeo.registry import SchemaRegistry

__all__ = [
    # Exceptions
    "EaseoError",
    "InvalidUrlError",
    "ConfigurationError",
    "EntityError",
    "SchemaError",
    "ContractError",
    # Types
    "SEOConfig",
    "SEOEntity",
    "SEOPayload",
    "SEOOverrides",
    "SEOImage",
    "SEOAuthor",
    "OGPayload",
    "TwitterPayload",
    "Breadcrumb",
    "FAQItem",
    "Robots",
    "URLPolicy",
    "SEOContract",
    "SEOContractConfig",
    "SEOContractRule",
    "SEOExpectation",
    "SEOIssue",
    "SchemaRegistry",
    "SEOEntityBuilder",
    "HookRegistry",
    # Factories
    "from_blog_post",
    "from_product",
    "from_faq",
    # Functions
    "build_seo_payload",
    "build_seo_payload_with_overrides",
    "build_seo_payload_dict",
    "build_seo_payload_async",
    "set_executor",
    "build_seo_contract",
    "validate_payload",
    "normalize_path",
    "normalize_public_url",
    "clean_url",
    "clean_query",
]
