"""Tests for easeo Python bindings."""

import sys
import os

# Add the parent directory to the path so we can import easeo
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'python'))


def test_basic_import():
    """Test that all expected types can be imported."""
    from easeo import (
        SEOConfig, SEOEntity, SEOPayload, SEOOverrides,
        SEOImage, Breadcrumb, FAQItem, Robots, URLPolicy,
        SEOContract, SEOContractConfig, SEOIssue,
        build_seo_payload, build_seo_payload_dict,
        build_seo_contract, validate_payload,
        normalize_path, normalize_public_url,
        clean_url, clean_query,
    )
    assert SEOConfig is not None
    assert SEOEntity is not None
    assert build_seo_payload is not None


def test_build_payload():
    """Test basic payload generation."""
    from easeo import SEOConfig, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )

    entity = SEOEntity(
        entity_type="post",
        title="Hello World",
        excerpt="A test post.",
    )

    payload = build_seo_payload(entity, "/blog/hello", config)
    assert payload.title == "Hello World"
    assert payload.canonical == "https://example.com/blog/hello"
    assert payload.robots == "index,follow"


def test_render_html():
    """Test HTML rendering."""
    from easeo import SEOConfig, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )

    entity = SEOEntity(
        entity_type="page",
        title="Test Page",
        excerpt="A test.",
    )

    payload = build_seo_payload(entity, "/test", config)
    html = payload.render_html()
    assert "<title>Test Page</title>" in html
    assert "og:title" in html
    assert "twitter:card" in html


def test_hash_deterministic():
    """Test that hashing is deterministic."""
    from easeo import SEOConfig, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )

    entity = SEOEntity(
        entity_type="page",
        title="Stable",
    )

    h1 = build_seo_payload(entity, "/test", config).hash()
    h2 = build_seo_payload(entity, "/test", config).hash()
    assert h1 == h2
    assert len(h1) == 64  # SHA-256 hex


def test_contract():
    """Test contract generation."""
    from easeo import SEOContractConfig, build_seo_contract

    config = SEOContractConfig(
        canonical_host="example.com",
        scheme="https",
    )

    contract = build_seo_contract(config)
    assert contract.contract_version == "1"
    assert contract.site["canonical_host"] == "example.com"


def test_validation():
    """Test validation produces issues."""
    from easeo import SEOConfig, SEOEntity, build_seo_payload, validate_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )

    entity = SEOEntity(
        entity_type="page",
        title="A" * 70,  # Too long
    )

    payload = build_seo_payload(entity, "/test", config)
    issues = validate_payload(payload)
    assert len(issues) > 0
    assert any(i.rule_id == "EASEO102" for i in issues)


def test_url_normalization():
    """Test URL normalization."""
    from easeo import SEOConfig, SEOEntity, build_seo_payload

    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )

    entity = SEOEntity(
        entity_type="page",
        title="Test",
    )

    payload = build_seo_payload(entity, "//double//slashes", config)
    assert "//" not in payload.canonical


def test_detrack():
    """Test tracking parameter removal."""
    from easeo import clean_url, clean_query

    result = clean_url("https://example.com/page?utm_source=twitter&q=hello&fbclid=123")
    assert result["url"] == "https://example.com/page?q=hello"
    assert "utm_source" in result["removed_params"]
    assert "fbclid" in result["removed_params"]

    query = clean_query("a=1&utm_source=x&b=2")
    assert "utm_source" not in query
    assert "a=1" in query
    assert "b=2" in query
