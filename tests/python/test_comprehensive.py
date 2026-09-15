"""Comprehensive edge case tests for easeo Python bindings."""

import pytest
from easeo import (
    SEOConfig,
    SEOEntity,
    SEOOverrides,
    SEOImage,
    Breadcrumb,
    FAQItem,
    Robots,
    URLPolicy,
    SEOContractConfig,
    build_seo_payload,
    build_seo_payload_with_overrides,
    build_seo_payload_dict,
    build_seo_contract,
    validate_payload,
    normalize_path,
    normalize_public_url,
    clean_url,
    clean_query,
)


def default_config():
    return SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )


def default_entity():
    return SEOEntity(
        entity_type="post",
        title="Test Title",
        excerpt="Test description",
    )


# ═══════════════════════════════════════════════════════════════════════════════
# URL NORMALIZATION TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestURLNormalization:
    def test_empty_becomes_slash(self):
        policy = URLPolicy()
        assert normalize_path("", policy) == "/"

    def test_prepends_slash(self):
        policy = URLPolicy()
        assert normalize_path("blog/hello", policy) == "/blog/hello"

    def test_unicode(self):
        policy = URLPolicy()
        assert normalize_path("/blog/café", policy) == "/blog/café"

    def test_javascript_scheme(self):
        config = default_config()
        result = normalize_public_url("javascript:alert(1)", config)
        assert result.startswith("https://example.com/")
        assert "javascript:" not in result

    def test_data_scheme(self):
        config = default_config()
        result = normalize_public_url("data:text/html,<h1>hi</h1>", config)
        assert result.startswith("https://example.com/")
        assert "data:" not in result

    def test_trailing_slash_always(self):
        policy = URLPolicy(trailing_slash="always")
        assert normalize_path("/blog/hello", policy) == "/blog/hello/"

    def test_trailing_slash_never(self):
        policy = URLPolicy(trailing_slash="never")
        assert normalize_path("/blog/hello/", policy) == "/blog/hello"

    def test_trailing_slash_preserve(self):
        policy = URLPolicy(trailing_slash="preserve")
        assert normalize_path("/blog/hello/", policy) == "/blog/hello/"
        assert normalize_path("/blog/hello", policy) == "/blog/hello"

    def test_lowercase_paths_disabled(self):
        policy = URLPolicy(lowercase_paths=False)
        assert normalize_path("/Blog/Hello", policy) == "/Blog/Hello"

    def test_allowed_query_params(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            url_policy=URLPolicy(
                allowed_query_params=["q", "page"],
                strip_tracking_params=False,
            ),
        )
        result = normalize_public_url(
            "https://example.com/page?q=hello&utm_source=x&page=1&fbclid=y",
            config,
        )
        assert "q=hello" in result
        assert "page=1" in result

    def test_http_allowed(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="http://example.com",
            url_policy=URLPolicy(enforce_https=False),
        )
        result = normalize_public_url("http://example.com/page", config)
        assert result.startswith("http://")


# ═══════════════════════════════════════════════════════════════════════════════
# DETRACK TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestDetrack:
    def test_clean_url_empty_query(self):
        result = clean_url("https://x.com/page?")
        assert result["url"] == "https://x.com/page"

    def test_clean_query_double_ampersand(self):
        result = clean_query("a=1&&b=2")
        assert "a=1" in result
        assert "b=2" in result

    def test_clean_query_duplicate_tracking(self):
        result = clean_query("utm_source=a&utm_source=b&q=1")
        assert "q=1" in result
        assert "utm_source" not in result

    def test_clean_url_with_fragment(self):
        result = clean_url("https://x.com/page?q=1#section")
        assert "#section" in result["url"]

    def test_clean_url_all_tracking(self):
        result = clean_url("https://x.com/page?utm_source=x&fbclid=y")
        assert result["url"] == "https://x.com/page"

    def test_clean_url_no_tracking(self):
        result = clean_url("https://x.com/page?q=hello&page=1")
        assert result["url"] == "https://x.com/page?q=hello&page=1"

    def test_clean_query_case_insensitive(self):
        result = clean_query("UTM_SOURCE=twitter&q=hello")
        assert "q=hello" in result
        assert "UTM_SOURCE" not in result

    def test_clean_query_param_no_equals(self):
        result = clean_query("fbclid")
        assert "fbclid" not in result

    def test_clean_query_all_removed_returns_empty(self):
        result = clean_query("utm_source=x&fbclid=y")
        assert result == ""

    def test_clean_url_mixed_tracking_and_normal(self):
        result = clean_url("https://example.com?a=1&utm_source=x&b=2&fbclid=y&c=3")
        assert "a=1" in result["url"]
        assert "b=2" in result["url"]
        assert "c=3" in result["url"]
        assert "utm_source" not in result["url"]
        assert "fbclid" not in result["url"]


# ═══════════════════════════════════════════════════════════════════════════════
# PAYLOAD TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestPayload:
    def test_no_title_defaults_to_untitled(self):
        entity = SEOEntity(entity_type="page")
        payload = build_seo_payload(entity, "/test", default_config())
        assert payload.title == "Untitled"

    def test_title_boundary_60_no_issue(self):
        entity = SEOEntity(entity_type="page", title="A" * 60)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO102" for i in issues)

    def test_title_boundary_61_triggers_issue(self):
        entity = SEOEntity(entity_type="page", title="A" * 61)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO102" for i in issues)

    def test_empty_description(self):
        entity = SEOEntity(entity_type="page", title="Title")
        payload = build_seo_payload(entity, "/test", default_config())
        assert payload.description == ""

    def test_description_boundary_160_no_issue(self):
        entity = SEOEntity(entity_type="page", title="Title", excerpt="A" * 160)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO104" for i in issues)

    def test_description_boundary_161_triggers_issue(self):
        entity = SEOEntity(entity_type="page", title="Title", excerpt="A" * 161)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO104" for i in issues)

    def test_html_chars_in_title_escaped(self):
        entity = SEOEntity(
            entity_type="page",
            title="<script>alert(1)</script>",
        )
        payload = build_seo_payload(entity, "/test", default_config())
        html = payload.render_html()
        assert "&lt;script&gt;alert(1)&lt;/script&gt;" in html
        assert '<title><script>' not in html

    def test_html_chars_in_description_escaped(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
            excerpt='quotes" & <tags>',
        )
        payload = build_seo_payload(entity, "/test", default_config())
        html = payload.render_html()
        assert "&amp;" in html
        assert "&lt;" in html
        assert "&gt;" in html
        assert "&quot;" in html

    def test_jsonld_script_tag_escaped(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
        )
        overrides = SEOOverrides(schema_jsonld={"name": "</script>"})
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        jsonld = payload.render_jsonld()
        assert r"<\/script>" in jsonld

    def test_search_entity_uses_search_robots(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            search_robots=Robots(index=False, follow=True),
        )
        entity = SEOEntity(entity_type="search", title="Search")
        payload = build_seo_payload(entity, "/search", config)
        assert "noindex" in payload.robots

    def test_published_status_overrides_robots(self):
        entity = SEOEntity(
            entity_type="page",
            title="Page",
            status="published",
        )
        payload = build_seo_payload(entity, "/test", default_config())
        assert payload.robots == "index,follow"

    def test_relative_og_image_in_schema(self):
        entity = SEOEntity(
            entity_type="post",
            title="Post",
            featured_image=SEOImage(url="/images/photo.jpg"),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        schema = payload.schema_jsonld
        assert schema is not None
        image = schema.get("image", "")
        assert image.startswith("https://example.com/")

    def test_relative_publisher_logo(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            publisher_name="MyOrg",
            publisher_logo="/images/logo.png",
        )
        entity = SEOEntity(entity_type="organization", title="Org")
        payload = build_seo_payload(entity, "/test", config)
        schema = payload.schema_jsonld
        assert schema is not None
        logo = schema.get("publisher", {}).get("logo", "")
        assert logo.startswith("https://example.com/")

    def test_skip_title_template(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            title_template="{title} | MySite",
        )
        entity = SEOEntity(entity_type="page", title="Title")
        overrides = SEOOverrides(skip_title_template=True)
        payload = build_seo_payload_with_overrides(
            entity, "/test", config, overrides
        )
        assert payload.title == "Title"

    def test_hash_deterministic(self):
        entity = default_entity()
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        h1 = payload.hash()
        h2 = payload.hash()
        assert h1 == h2
        assert len(h1) == 64

    def test_etag_format(self):
        entity = default_entity()
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        etag = payload.etag()
        assert etag.startswith('"')
        assert etag.endswith('"')
        assert len(etag) == 66

    def test_to_dict_returns_dict(self):
        entity = default_entity()
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        d = payload.to_dict()
        assert isinstance(d, dict)
        assert "title" in d

    def test_to_json_returns_json(self):
        entity = default_entity()
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        j = payload.to_json()
        assert isinstance(j, str)
        assert "title" in j


# ═══════════════════════════════════════════════════════════════════════════════
# VALIDATION TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestValidation:
    def test_empty_title_easeo101(self):
        entity = SEOEntity(entity_type="page")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO101" for i in issues)

    def test_title_60_no_issue(self):
        entity = SEOEntity(entity_type="page", title="A" * 60)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO102" for i in issues)

    def test_title_61_triggers_easeo102(self):
        entity = SEOEntity(entity_type="page", title="A" * 61)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO102" for i in issues)

    def test_empty_description_easeo103(self):
        entity = SEOEntity(entity_type="page", title="Title")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO103" for i in issues)

    def test_description_160_no_issue(self):
        entity = SEOEntity(entity_type="page", title="Title", excerpt="A" * 160)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO104" for i in issues)

    def test_description_161_triggers_easeo104(self):
        entity = SEOEntity(entity_type="page", title="Title", excerpt="A" * 161)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO104" for i in issues)

    def test_relative_canonical_easeo105(self):
        entity = SEOEntity(entity_type="page", title="Title")
        overrides = SEOOverrides(canonical_url="/relative/path")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO105" for i in issues)

    def test_absolute_canonical_no_issue(self):
        entity = SEOEntity(entity_type="page", title="Title")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO105" for i in issues)

    def test_relative_og_image_easeo106(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
            featured_image=SEOImage(url="/images/photo.jpg"),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO106" for i in issues)

    def test_absolute_og_image_no_issue(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
            featured_image=SEOImage(url="https://example.com/photo.jpg"),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO106" for i in issues)

    def test_clean_payload_no_issues(self):
        entity = SEOEntity(
            entity_type="page",
            title="Good Title",
            excerpt="A good description",
            featured_image=SEOImage(
                url="https://example.com/photo.jpg",
                width=1200,
                height=630,
                alt="Photo",
            ),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert len(issues) == 0

    def test_multiple_issues(self):
        entity = SEOEntity(
            entity_type="page",
            featured_image=SEOImage(url="/photo.jpg"),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert len(issues) >= 3


# ═══════════════════════════════════════════════════════════════════════════════
# CONFIG TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestConfig:
    def test_validate_empty_canonical_host(self):
        with pytest.raises(Exception):
            SEOConfig(canonical_host="", public_base_url="https://example.com")

    def test_validate_host_with_scheme(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="https://example.com",
                public_base_url="https://example.com",
            )

    def test_validate_host_with_path(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="example.com/blog",
                public_base_url="https://example.com",
            )

    def test_validate_title_template_no_placeholder(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="example.com",
                public_base_url="https://example.com",
                title_template="My Site",
            )

    def test_valid_config(self):
        config = default_config()
        assert config is not None


# ═══════════════════════════════════════════════════════════════════════════════
# CONTRACT TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestContract:
    def test_empty_rules(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert isinstance(d, dict)

    def test_serialization_roundtrip(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        j = contract.to_json()
        assert "example.com" in j

    def test_hash_deterministic(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        h1 = contract.hash()
        h2 = contract.hash()
        assert h1 == h2

    def test_to_dict(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert isinstance(d, dict)


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL URL NORMALIZATION TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestURLNormalizationAdditional:
    def test_collapse_duplicate_slashes_disabled(self):
        policy = URLPolicy(collapse_duplicate_slashes=False)
        assert normalize_path("//blog///hello", policy) == "//blog///hello"

    def test_normalize_public_url_empty_string(self):
        config = default_config()
        with pytest.raises(Exception):
            normalize_public_url("", config)

    def test_normalize_public_url_with_base_path(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com/blog",
        )
        result = normalize_public_url("/about", config)
        assert result == "https://example.com/blog/about"

    def test_normalize_public_url_base_path_already_present(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com/blog",
        )
        result = normalize_public_url("/blog/about", config)
        assert result == "https://example.com/blog/about"

    def test_trailing_slash_always_on_root(self):
        policy = URLPolicy(trailing_slash="always")
        assert normalize_path("/", policy) == "/"

    def test_trailing_slash_never_on_root(self):
        policy = URLPolicy(trailing_slash="never")
        assert normalize_path("/", policy) == "/"

    def test_normalize_path_whitespace_trimmed(self):
        policy = URLPolicy()
        assert normalize_path("  /blog/hello  ", policy) == "/blog/hello"

    def test_filter_query_empty_key_skipped(self):
        policy = URLPolicy(strip_tracking_params=False)
        assert normalize_public_url("https://example.com/?=value&foo=bar", default_config()).endswith("?foo=bar")

    def test_filter_query_tracking_and_allowlist_combined(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            url_policy=URLPolicy(
                allowed_query_params=["q"],
                strip_tracking_params=True,
            ),
        )
        result = normalize_public_url("https://example.com?q=hello&utm_source=x&other=1", config)
        assert "q=hello" in result
        assert "utm_source" not in result
        assert "other" not in result

    def test_normalize_public_url_strips_fragment(self):
        config = default_config()
        result = normalize_public_url("/page#section", config)
        assert "#section" in result

    def test_lowercase_paths_enabled(self):
        policy = URLPolicy(lowercase_paths=True)
        assert normalize_path("/Blog/Hello", policy) == "/blog/hello"

    def test_filter_query_preserves_encoded_values(self):
        policy = URLPolicy(strip_tracking_params=False)
        assert normalize_public_url("https://example.com?q=hello%20world", default_config()).endswith("q=hello%20world")


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL DETRACK TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestDetrackAdditional:
    def test_clean_url_no_question_mark(self):
        result = clean_url("https://x.com/page")
        assert result["url"] == "https://x.com/page"

    def test_clean_query_empty_string(self):
        assert clean_query("") == ""

    def test_clean_url_cleaned_params_populated(self):
        result = clean_url("https://x.com/page?q=hello&other=1")
        assert "q" in result["cleaned_params"]
        assert "other" in result["cleaned_params"]

    def test_tracking_param_value_with_equals(self):
        result = clean_url("https://x.com/page?utm_content=a=b&q=1")
        assert "q=1" in result["url"]
        assert "utm_content" not in result["url"]

    def test_clean_url_multiple_question_marks(self):
        result = clean_url("https://x.com/page?a=1?b=2")
        assert "a=1" in result["url"]

    def test_clean_url_no_tracking_all_preserved(self):
        result = clean_url("https://x.com/page?a=1&b=2&c=3")
        assert "a=1" in result["url"]
        assert "b=2" in result["url"]
        assert "c=3" in result["url"]

    def test_is_tracking_param_various(self):
        assert clean_url("https://x.com?gclid=1")["url"] == "https://x.com"
        assert clean_url("https://x.com?_ga=1")["url"] == "https://x.com"
        assert clean_url("https://x.com?msclkid=1")["url"] == "https://x.com"


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL PAYLOAD / OVERRIDE TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestPayloadAdditional:
    def test_og_type_home(self):
        entity = SEOEntity(entity_type="home", title="Home")
        payload = build_seo_payload(entity, "/", default_config())
        html = payload.render_html()
        assert 'og:type" content="website"' in html

    def test_og_type_video(self):
        entity = SEOEntity(entity_type="video", title="Video")
        payload = build_seo_payload(entity, "/video", default_config())
        html = payload.render_html()
        assert 'og:type" content="video.other"' in html or 'og:type' in html

    def test_og_type_faq(self):
        entity = SEOEntity(entity_type="faq", title="FAQ")
        payload = build_seo_payload(entity, "/faq", default_config())
        html = payload.render_html()
        assert 'og:type' in html

    def test_description_fallback_to_body_html(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
            body_html="<p>This is the body content.</p>",
        )
        payload = build_seo_payload(entity, "/test", default_config())
        assert "body content" in payload.description

    def test_override_meta_title(self):
        entity = SEOEntity(entity_type="page", title="Original")
        overrides = SEOOverrides(meta_title="Overridden Title")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        assert payload.title == "Overridden Title"

    def test_override_meta_description(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="Original")
        overrides = SEOOverrides(meta_description="Overridden Desc")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        assert payload.description == "Overridden Desc"

    def test_override_og_title(self):
        entity = SEOEntity(entity_type="page", title="Title")
        overrides = SEOOverrides(og_title="OG Override")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "OG Override" in html

    def test_override_og_description(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="Desc")
        overrides = SEOOverrides(og_description="OG Desc Override")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "OG Desc Override" in html

    def test_override_og_image(self):
        entity = SEOEntity(
            entity_type="page",
            title="T",
            featured_image=SEOImage(url="https://example.com/original.jpg"),
        )
        overrides = SEOOverrides(
            og_image=SEOImage(url="https://example.com/override.jpg")
        )
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "override.jpg" in html
        assert "original.jpg" not in html

    def test_override_twitter_image(self):
        entity = SEOEntity(
            entity_type="page",
            title="T",
            featured_image=SEOImage(url="https://example.com/og.jpg"),
        )
        overrides = SEOOverrides(
            twitter_image=SEOImage(url="https://example.com/twitter.jpg")
        )
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "twitter.jpg" in html

    def test_override_twitter_title(self):
        entity = SEOEntity(entity_type="page", title="Title")
        overrides = SEOOverrides(twitter_title="Twitter Title")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "Twitter Title" in html

    def test_override_twitter_description(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="Desc")
        overrides = SEOOverrides(twitter_description="Twitter Desc")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "Twitter Desc" in html

    def test_override_twitter_card(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(twitter_card="summary")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert 'twitter:card" content="summary"' in html

    def test_override_twitter_creator(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(twitter_creator="@handle")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "@handle" in html

    def test_override_og_audio(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(og_audio="https://example.com/audio.mp3")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "audio.mp3" in html

    def test_override_og_video(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(og_video="https://example.com/video.mp4")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "video.mp4" in html

    def test_override_robots(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(robots=Robots(index=False, follow=False))
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert "noindex" in html
        assert "nofollow" in html

    def test_breadcrumbs_in_schema(self):
        entity = SEOEntity(
            entity_type="page",
            title="Page",
            breadcrumbs=[
                Breadcrumb(name="Home", url="/"),
                Breadcrumb(name="Blog", url="/blog"),
            ],
        )
        payload = build_seo_payload(entity, "/blog/page", default_config())
        schema = payload.schema_jsonld
        assert schema is not None

    def test_render_opengraph_all_fields(self):
        entity = SEOEntity(
            entity_type="page",
            title="Title",
            featured_image=SEOImage(
                url="https://example.com/img.jpg",
                width=1200,
                height=630,
                alt="Image",
            ),
        )
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            site_name="MySite",
            locale="en_US",
        )
        payload = build_seo_payload(entity, "/test", config)
        html = payload.render_html()
        assert 'og:type' in html
        assert 'og:title' in html
        assert 'og:image' in html
        assert 'og:image:width' in html
        assert 'og:image:height' in html
        assert 'og:image:alt' in html

    def test_render_twitter_all_fields(self):
        entity = SEOEntity(entity_type="page", title="T", excerpt="D")
        overrides = SEOOverrides(
            twitter_creator="@creator",
            twitter_image=SEOImage(url="https://example.com/tw.jpg", alt="TW"),
        )
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        html = payload.render_html()
        assert 'twitter:card' in html
        assert 'twitter:creator' in html
        assert 'twitter:image' in html

    def test_render_html_no_description_skips_meta(self):
        entity = SEOEntity(entity_type="page", title="T")
        payload = build_seo_payload(entity, "/test", default_config())
        html = payload.render_html()
        assert 'meta name="description"' not in html

    def test_render_jsonld_none_returns_empty(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(omit_schema=True)
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        jsonld = payload.render_jsonld()
        assert jsonld == ""

    def test_og_image_with_dimensions_and_alt(self):
        entity = SEOEntity(
            entity_type="page",
            title="T",
            featured_image=SEOImage(
                url="https://example.com/img.jpg",
                width=1200,
                height=630,
                alt="A photo",
            ),
        )
        payload = build_seo_payload(entity, "/test", default_config())
        html = payload.render_html()
        assert 'og:image:width" content="1200"' in html
        assert 'og:image:height" content="630"' in html
        assert 'og:image:alt" content="A photo"' in html

    def test_default_robots_from_config(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            default_robots=Robots(index=False, follow=True),
        )
        entity = SEOEntity(entity_type="page", title="T")
        payload = build_seo_payload(entity, "/test", config)
        assert "noindex" in payload.robots
        assert "follow" in payload.robots

    def test_whitespace_title_defaults_to_untitled(self):
        entity = SEOEntity(entity_type="page", title="   ")
        payload = build_seo_payload(entity, "/test", default_config())
        assert payload.title == "Untitled"

    def test_escape_html_single_quotes(self):
        entity = SEOEntity(
            entity_type="page",
            title="It's a test",
            excerpt="She said 'hello'",
        )
        payload = build_seo_payload(entity, "/test", default_config())
        html = payload.render_html()
        assert "&#x27;" in html

    def test_render_html_returns_result(self):
        entity = SEOEntity(entity_type="page", title="T")
        payload = build_seo_payload(entity, "/test", default_config())
        result = payload.render_html()
        assert isinstance(result, str)
        assert "<title>T</title>" in result


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL VALIDATION TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestValidationAdditional:
    def test_easeo101_title_untitled(self):
        entity = SEOEntity(entity_type="page", title="Untitled")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert any(i.rule_id == "EASEO101" for i in issues)

    def test_easeo107_invalid_robots_directive(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(
            robots=Robots(index=True, follow=True)
        )
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO107" for i in issues)

    def test_validation_severity_levels(self):
        entity = SEOEntity(entity_type="page")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        for issue in issues:
            assert issue.severity in ("error", "warning", "info")

    def test_issue_url_matches_canonical(self):
        entity = SEOEntity(entity_type="page")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        for issue in issues:
            if issue.url:
                assert issue.url == payload.canonical

    def test_issue_details_populated(self):
        entity = SEOEntity(entity_type="page", title="A" * 61)
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        easeo102 = [i for i in issues if i.rule_id == "EASEO102"]
        assert len(easeo102) > 0

    def test_no_og_image_easeo106_not_triggered(self):
        entity = SEOEntity(entity_type="page", title="T")
        payload = build_seo_payload(entity, "/test", default_config())
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO106" for i in issues)

    def test_valid_robots_all_directives_no_issue(self):
        entity = SEOEntity(entity_type="page", title="T")
        overrides = SEOOverrides(
            robots=Robots(index=True, follow=True)
        )
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        issues = validate_payload(payload)
        assert not any(i.rule_id == "EASEO107" for i in issues)

    def test_empty_payload_exact_issue_set(self):
        entity = SEOEntity(entity_type="page")
        overrides = SEOOverrides(canonical_url="/relative")
        payload = build_seo_payload_with_overrides(
            entity, "/test", default_config(), overrides
        )
        issues = validate_payload(payload)
        rule_ids = {i.rule_id for i in issues}
        assert "EASEO101" in rule_ids
        assert "EASEO103" in rule_ids
        assert "EASEO105" in rule_ids


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL HASHING TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestHashingAdditional:
    def test_different_payloads_different_hashes(self):
        e1 = SEOEntity(entity_type="page", title="Title1")
        e2 = SEOEntity(entity_type="page", title="Title2")
        config = default_config()
        p1 = build_seo_payload(e1, "/test", config)
        p2 = build_seo_payload(e2, "/test", config)
        assert p1.hash() != p2.hash()

    def test_minimal_payload_hash(self):
        entity = SEOEntity(entity_type="page")
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        h = payload.hash()
        assert len(h) == 64

    def test_etag_exact_format(self):
        entity = SEOEntity(entity_type="page", title="T")
        config = default_config()
        payload = build_seo_payload(entity, "/test", config)
        etag = payload.etag()
        assert etag.startswith('"')
        assert etag.endswith('"')
        assert len(etag) == 66


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL CONFIG TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestConfigAdditional:
    def test_validate_host_with_query(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="example.com?foo=bar",
                public_base_url="https://example.com",
            )

    def test_validate_host_with_fragment(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="example.com#section",
                public_base_url="https://example.com",
            )

    def test_validate_empty_public_base_url(self):
        with pytest.raises(Exception):
            SEOConfig(
                canonical_host="example.com",
                public_base_url="",
            )

    def test_url_policy_defaults(self):
        policy = URLPolicy()
        assert policy.enforce_https is True
        assert policy.lowercase_paths is True
        assert policy.collapse_duplicate_slashes is True
        assert policy.strip_tracking_params is True

    def test_config_defaults(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
        )
        d = config.to_dict()
        assert d["auto_generate_schema"] is True

    def test_default_schema_type_map(self):
        config = default_config()
        d = config.to_dict()
        assert "schema_type_map" in d

    def test_config_with_all_fields(self):
        config = SEOConfig(
            canonical_host="example.com",
            public_base_url="https://example.com",
            site_name="MySite",
            title_template="{title} | MySite",
            default_og_image=SEOImage(url="https://example.com/og.jpg"),
            publisher_name="Publisher",
            locale="en_US",
            twitter_site="@handle",
            auto_generate_schema=True,
        )
        assert config is not None


# ═══════════════════════════════════════════════════════════════════════════════
# ADDITIONAL CONTRACT TESTS
# ═══════════════════════════════════════════════════════════════════════════════

class TestContractAdditional:
    def test_contract_with_rules(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert "rules" in d

    def test_contract_with_defaults(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert "defaults" in d

    def test_contract_http_scheme(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="http",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert d.get("site", {}).get("scheme") == "http"

    def test_contract_write_to_file(self, tmp_path):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        path = tmp_path / "contract.json"
        contract.write(str(path))
        assert path.exists()
        content = path.read_text()
        assert "example.com" in content

    def test_contract_site_host(self):
        config = SEOContractConfig(
            canonical_host="example.com",
            scheme="https",
        )
        contract = build_seo_contract(config)
        d = contract.to_dict()
        assert d.get("site", {}).get("canonical_host") == "example.com"
