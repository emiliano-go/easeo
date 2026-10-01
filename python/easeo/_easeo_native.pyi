"""
Python bindings for the easeo Rust core.
"""

from _typeshed import Incomplete
from collections.abc import Sequence
from typing import Any, final

@final
class Breadcrumb:
    """
    A single entry in a breadcrumb trail.
    """
    def __new__(cls, /, name: str, url: str) -> Breadcrumb:
        """
        Creates a breadcrumb entry.
        
        Args:
            name: Breadcrumb label.
            url: Breadcrumb URL.
        """
    @property
    def name(self, /) -> str:
        """
        Human readable label for the breadcrumb.
        """
    @property
    def url(self, /) -> str:
        """
        URL the breadcrumb links to.
        """

@final
class FAQItem:
    """
    A single question and answer pair for FAQ schemas.
    """
    def __new__(cls, /, question: str, answer: str) -> FAQItem:
        """
        Creates a question and answer pair.
        
        Args:
            question: Question text.
            answer: Answer text.
        """
    @property
    def answer(self, /) -> str:
        """
        The answer text.
        """
    @property
    def question(self, /) -> str:
        """
        The question text.
        """

@final
class OGPayload:
    """
    Open Graph metadata for a payload. Read-only.
    """
    @property
    def audio(self, /) -> str |None:
        """
        Open Graph audio URL.
        """
    @property
    def description(self, /) -> str |None:
        """
        Open Graph description.
        """
    @property
    def image(self, /) -> str |None:
        """
        Absolute URL of the Open Graph image.
        """
    @property
    def image_alt(self, /) -> str |None:
        """
        Open Graph image alternative text.
        """
    @property
    def image_height(self, /) -> int |None:
        """
        Open Graph image height in pixels.
        """
    @property
    def image_width(self, /) -> int |None:
        """
        Open Graph image width in pixels.
        """
    @property
    def locale(self, /) -> str |None:
        """
        Locale, for example ``"en_US"``.
        """
    @property
    def locale_alternate(self, /) -> list[str] |None:
        """
        Alternate locales.
        """
    @property
    def site_name(self, /) -> str |None:
        """
        Site name.
        """
    @property
    def title(self, /) -> str |None:
        """
        Open Graph title.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the Open Graph metadata as a dictionary.
        """
    @property
    def type(self, /) -> str:
        """
        Open Graph object type, for example ``"article"`` or ``"website"``.
        """
    @property
    def url(self, /) -> str |None:
        """
        Open Graph canonical URL.
        """
    @property
    def video(self, /) -> str |None:
        """
        Open Graph video URL.
        """

@final
class Robots:
    """
    Robots directives for a page.
    """
    def __new__(cls, /, *, index: bool = True, follow: bool = True, max_snippet: int |None = None, max_image_preview: str |None = None, max_video_preview: int |None = None) -> Robots:
        """
        Creates robots directives. All arguments are keyword-only.
        
        Args:
            index: Whether search engines may index the page.
            follow: Whether search engines may follow links.
            max_snippet: Maximum snippet length.
            max_image_preview: Maximum image preview size.
            max_video_preview: Maximum video preview length in seconds.
        """
    @property
    def follow(self, /) -> bool:
        """
        Whether search engines may follow links. Defaults to ``True``.
        """
    @property
    def index(self, /) -> bool:
        """
        Whether search engines may index the page. Defaults to ``True``.
        """
    @property
    def max_image_preview(self, /) -> str |None:
        """
        Maximum image preview size, for example ``"large"``.
        """
    @property
    def max_snippet(self, /) -> int |None:
        """
        Maximum number of characters to show in a snippet.
        """
    @property
    def max_video_preview(self, /) -> int |None:
        """
        Maximum video preview length in seconds.
        """
    def serialize(self, /) -> str:
        """
        Serializes the directives into the meta robots string, for example
        ``"index,follow"``.
        """

@final
class SEOAuthor:
    """
    An author reference used in article schemas.
    """
    def __new__(cls, /, name: str, *, url: str |None = None) -> SEOAuthor:
        """
        Creates an author reference.
        
        Args:
            name: Display name of the author.
            url: Optional profile URL.
        """
    @property
    def name(self, /) -> str:
        """
        Display name of the author.
        """
    @property
    def url(self, /) -> str |None:
        """
        Optional profile URL for the author.
        """

@final
class SEOConfig:
    """
    Site-wide configuration for payload generation.
    """
    def __new__(cls, /, canonical_host: str, public_base_url: str, *, url_policy: URLPolicy |None = None, default_robots: Robots |None = None, default_og_image: SEOImage |None = None, site_name: str |None = None, title_template: str |None = None, search_robots: Robots |None = None, auto_generate_schema: bool = True, publisher_name: str |None = None, publisher_logo: str |None = None, locale: str |None = None, locale_alternate: Sequence[str] |None = None, twitter_site: str |None = None, emit_warnings: bool = False, schema_type_map: Sequence[tuple[str, str]] |None = None, search_url_template: str |None = None, hooks: Any |None = None, schema_registry: Any |None = None) -> SEOConfig:
        """
        Creates a site-wide configuration.
        
        Args:
            canonical_host: Canonical hostname without a scheme, for example
                ``"example.com"``.
            public_base_url: Full base URL for path resolution, for example
                ``"https://example.com"``.
            url_policy: URL normalization policy.
            default_robots: Robots directives for regular pages.
            default_og_image: Fallback Open Graph image.
            site_name: Site name for ``og:site_name`` and title templates.
            title_template: Title template containing ``{title}``.
            search_robots: Robots directives for search pages.
            auto_generate_schema: Generate JSON-LD from the entity type.
            publisher_name: Organization or publisher name.
            publisher_logo: Publisher logo URL.
            locale: Open Graph locale, for example ``"en_US"``.
            locale_alternate: Alternate locales.
            twitter_site: Twitter ``@handle`` for ``twitter:site``.
            emit_warnings: Emit Python warnings for validation issues.
            schema_type_map: Entity type to schema.org type overrides.
            search_url_template: Search URL template for the homepage
                ``WebSite`` ``SearchAction``.
            hooks: Config-scoped ``HookRegistry``.
            schema_registry: Config-scoped ``SchemaRegistry``.
        
        Raises:
            ConfigurationError: If a configuration value is invalid.
        """
    @property
    def hooks(self, /) -> Any |None:
        """
        Config-scoped hook registry, consulted after the payload is built.
        """
    @hooks.setter
    def hooks(self, /, hooks: Any |None) -> None:
        """
        Sets the config-scoped hook registry.
        """
    @property
    def schema_registry(self, /) -> Any |None:
        """
        Config-scoped schema registry with Python-callable generators.
        """
    @schema_registry.setter
    def schema_registry(self, /, registry: Any |None) -> None:
        """
        Sets the config-scoped schema registry.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the configuration as a dictionary.
        """

@final
class SEOContract:
    """
    A generated, machine-readable SEO contract.
    """
    @property
    def contract_version(self, /) -> str:
        """
        Version of the contract format itself.
        """
    def hash(self, /) -> str:
        """
        Returns the SHA-256 hash of the serialized contract.
        """
    @property
    def site(self, /) -> dict:
        """
        Site identity stored in the contract.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the contract as a dictionary.
        """
    def to_json(self, /) -> str:
        """
        Returns the contract as pretty printed JSON.
        """
    def write(self, /, path: str) -> None:
        """
        Writes the contract as JSON to the given path.
        """

@final
class SEOContractConfig:
    """
    Input configuration for building a contract.
    """
    def __new__(cls, /, canonical_host: str, *, scheme: str = "https", defaults: SEOExpectation |None = None, rules: Sequence[SEOContractRule] |None = None, exceptions: Sequence[tuple[str, SEOExpectation]] |None = None) -> SEOContractConfig:
        """
        Creates a contract configuration.
        
        Args:
            canonical_host: Canonical hostname without a scheme.
            scheme: URL scheme, defaults to ``"https"``.
            defaults: Default expectations applied to every route.
            rules: Route specific rules.
            exceptions: Per-route expectation overrides.
        """

@final
class SEOContractRule:
    """
    A contract rule matched against route paths.
    """
    def __new__(cls, /, match: str, expect: SEOExpectation, *, severity: str |None = None) -> SEOContractRule:
        """
        Creates a contract rule.
        
        Args:
            match: Route pattern, for example ``"/blog/*"``.
            expect: Expectations applied when the pattern matches.
            severity: Optional severity, one of ``"error"``, ``"warning"``,
                or ``"info"``.
        """
    @property
    def expect(self, /) -> SEOExpectation:
        """
        Expectations applied when the pattern matches.
        """
    @property
    def match(self, /) -> str:
        """
        Route pattern, for example ``"/blog/*"``.
        """
    @property
    def severity(self, /) -> str |None:
        """
        Optional severity for failures of this rule.
        """

@final
class SEOEntity:
    """
    A content entity to generate SEO metadata for.
    """
    def __new__(cls, /, entity_type: str, *, slug: str |None = None, title: str |None = None, excerpt: str |None = None, body_html: str |None = None, status: str |None = None, featured_image: SEOImage |None = None, published_at: str |None = None, updated_at: str |None = None, author_name: str |None = None, breadcrumbs: Sequence[Breadcrumb] |None = None, sku: str |None = None, price: str |None = None, price_currency: str |None = None, availability: str |None = None, same_as: Sequence[str] |None = None, address: str |None = None, faq_items: Sequence[FAQItem] |None = None) -> SEOEntity:
        """
        Creates a content entity.
        
        Args:
            entity_type: One of ``home``, ``post``, ``page``, ``video``,
                ``taxonomy``, ``search``, ``product``, ``organization``,
                ``local_business``, ``faq``, or ``other``.
            slug: URL slug.
            title: Page title.
            excerpt: Short description.
            body_html: Full content as HTML.
            status: Publication status. ``"published"`` (case-insensitive)
                keeps the page indexable; any other value becomes noindex.
            featured_image: Primary image.
            published_at: ISO date or datetime.
            updated_at: ISO date or datetime.
            author_name: Author display name.
            breadcrumbs: Breadcrumb trail.
            sku: Product SKU.
            price: Product price.
            price_currency: ISO currency code.
            availability: Product availability.
            same_as: Additional URLs for organization schemas.
            address: Postal address for local business schemas.
            faq_items: Question and answer pairs for FAQ schemas.
        
        Raises:
            EntityError: If ``entity_type`` is not a supported value.
        """
    @property
    def entity_type(self, /) -> str:
        """
        The entity type as a string.
        """
    @property
    def excerpt(self, /) -> str |None:
        """
        The short description.
        """
    @property
    def title(self, /) -> str |None:
        """
        The page title.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the entity as a dictionary.
        """

@final
class SEOExpectation:
    """
    Expectations applied to a page, a rule match, or contract defaults.
    """
    def __new__(cls, /, *, required: bool |None = None, forbidden: bool |None = None, equals: str |None = None, not_equals: str |None = None, contains: str |None = None, matches: str |None = None, one_of: Sequence[str] |None = None, min_length: int |None = None, max_length: int |None = None, min_items: int |None = None, max_items: int |None = None, indexable: bool |None = None, canonical: str |None = None, schema_required: bool |None = None, schema_types: Sequence[str] |None = None, og_required: bool |None = None, twitter_required: bool |None = None, sitemap_required: bool |None = None, hreflang_required: bool |None = None, title: SEOExpectation |None = None, description: SEOExpectation |None = None) -> SEOExpectation:
        """
        Creates an expectation. Every argument is optional; unset fields are
        not checked.
        
        Args:
            required: The field must be present.
            forbidden: The field must be absent.
            equals: The field must equal this value.
            not_equals: The field must not equal this value.
            contains: The field must contain this substring.
            matches: The field must match this regular expression.
            one_of: The field must be one of these values.
            min_length: Minimum string length.
            max_length: Maximum string length.
            min_items: Minimum number of items.
            max_items: Maximum number of items.
            indexable: Whether the page must be indexable.
            canonical: Expected canonical behavior, for example ``"self"``.
            schema_required: Whether a JSON-LD schema must be present.
            schema_types: Required schema.org types.
            og_required: Whether Open Graph metadata must be present.
            twitter_required: Whether Twitter Card metadata must be present.
            sitemap_required: Whether a sitemap entry must be present.
            hreflang_required: Whether hreflang annotations must be present.
            title: Nested expectations for the title.
            description: Nested expectations for the description.
        """
    @property
    def canonical(self, /) -> str |None:
        """
        Expected canonical behavior.
        """
    @property
    def indexable(self, /) -> bool |None:
        """
        Whether the page must be indexable.
        """
    @property
    def required(self, /) -> bool |None:
        """
        Whether the field is required.
        """

@final
class SEOImage:
    """
    An image reference used for Open Graph, Twitter Cards, and schemas.
    """
    def __new__(cls, /, url: str, *, width: int |None = None, height: int |None = None, alt: str |None = None) -> SEOImage:
        """
        Creates an image reference.
        
        Args:
            url: Absolute URL of the image.
            width: Image width in pixels.
            height: Image height in pixels.
            alt: Alternative text.
        """
    @property
    def alt(self, /) -> str |None:
        """
        Alternative text describing the image.
        """
    @property
    def height(self, /) -> int |None:
        """
        Image height in pixels.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the image as a dictionary.
        """
    @property
    def url(self, /) -> str:
        """
        Absolute URL of the image.
        """
    @property
    def width(self, /) -> int |None:
        """
        Image width in pixels.
        """

@final
class SEOIssue:
    """
    A single validation finding.
    """
    @property
    def details(self, /) -> Any:
        """
        Additional structured details about the finding.
        """
    @property
    def message(self, /) -> str:
        """
        Human readable description of the finding.
        """
    @property
    def rule_id(self, /) -> str:
        """
        Stable rule identifier, for example ``"EASEO101"``.
        """
    @property
    def severity(self, /) -> str:
        """
        Severity, one of ``"error"``, ``"warning"``, or ``"info"``.
        """
    @property
    def url(self, /) -> str |None:
        """
        Canonical URL the finding applies to.
        """

@final
class SEOOverrides:
    """
    Per-call overrides that take precedence over the entity and config.
    """
    def __new__(cls, /, *, meta_title: str |None = None, meta_description: str |None = None, canonical_url: str |None = None, canonical_path: str |None = None, robots: Robots |None = None, og_title: str |None = None, og_description: str |None = None, og_image: SEOImage |None = None, twitter_card: str |None = None, twitter_title: str |None = None, twitter_description: str |None = None, twitter_image: SEOImage |None = None, schema_jsonld: Any |None = None, omit_schema: bool = False, skip_title_template: bool = False, twitter_creator: str |None = None, og_audio: str |None = None, og_video: str |None = None) -> SEOOverrides:
        """
        Creates per-call overrides. All arguments are keyword-only and
        optional.
        
        Args:
            meta_title: Overrides the resolved title.
            meta_description: Overrides the resolved description.
            canonical_url: Overrides the resolved canonical URL. Trusted:
                used as-is after an absolute ``http(s)`` URL check.
            canonical_path: Overrides the route used to build the canonical
                URL, normalized through the URL policy. Ignored when
                ``canonical_url`` is set.
            robots: Overrides the robots directives.
            og_title: Overrides the Open Graph title.
            og_description: Overrides the Open Graph description.
            og_image: Overrides the Open Graph image.
            twitter_card: Overrides the Twitter Card type.
            twitter_title: Overrides the Twitter title.
            twitter_description: Overrides the Twitter description.
            twitter_image: Overrides the Twitter image.
            schema_jsonld: Replaces the generated JSON-LD schema.
            omit_schema: When ``True``, no JSON-LD is emitted.
            skip_title_template: When ``True``, the title template is skipped.
            twitter_creator: Overrides the Twitter creator handle.
            og_audio: Open Graph audio URL.
            og_video: Open Graph video URL.
        """

@final
class SEOPayload:
    """
    The resolved, deterministic SEO payload.
    
    Dict-compatible: supports ``payload["title"]``, ``payload.get(...)``,
    ``len(payload)``, iteration, and equality against another payload or a
    plain dictionary.
    """
    def __contains__(self, key: str, /) -> bool:
        """
        Returns whether ``key`` is present.
        """
    def __eq__(self, other: object, /) -> bool:
        """
        Equality against another payload or a plain dict. This is what makes
        `assert payload == expected_dict` work for snapshot testing.
        """
    def __getitem__(self, key: str, /) -> Any:
        """
        Dict-style access: payload["title"].
        """
    def __iter__(self, /) -> Any:
        """
        Iterates over the payload keys.
        """
    def __len__(self, /) -> int:
        """
        Returns the number of top-level payload fields.
        """
    @property
    def canonical(self, /) -> str:
        """
        The normalized canonical URL.
        """
    @property
    def description(self, /) -> str:
        """
        The resolved description.
        """
    def etag(self, /) -> str:
        """
        Returns the payload hash as a quoted HTTP ETag.
        """
    def get(self, /, key: str, default: Any |None = None) -> Any:
        """
        Returns the value for ``key``, or ``default`` when absent.
        """
    def hash(self, /) -> str:
        """
        Returns the SHA-256 hash of the payload.
        """
    def keys(self, /) -> list:
        """
        Returns the payload keys.
        """
    @property
    def og(self, /) -> OGPayload:
        """
        The Open Graph metadata.
        """
    def render_html(self, /) -> str:
        """
        Renders the full head snippet: title, description, canonical, robots,
        Open Graph, Twitter Cards, and JSON-LD.
        """
    def render_jsonld(self, /) -> str:
        """
        Renders only the JSON-LD script tag.
        """
    def render_opengraph(self, /) -> str:
        """
        Renders only the Open Graph meta tags.
        """
    def render_twitter(self, /) -> str:
        """
        Renders only the Twitter Card meta tags.
        """
    @property
    def robots(self, /) -> str:
        """
        The serialized robots directives.
        """
    @property
    def schema_jsonld(self, /) -> Any |None:
        """
        The generated JSON-LD schema, as a dict or list.
        """
    @property
    def title(self, /) -> str:
        """
        The resolved title.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the payload as a dictionary.
        """
    def to_json(self, /) -> str:
        """
        Returns the payload as a pretty printed JSON string.
        """
    @property
    def twitter(self, /) -> TwitterPayload:
        """
        The Twitter Card metadata.
        """

@final
class SchemaRegistry:
    """
    Native introspection handle for the Rust schema registry.
    
    Python callables are registered through ``easeo.registry.SchemaRegistry``,
    which stores them and applies the generated schema around the build. This
    native type only exposes ``has`` and ``list_types``.
    """
    def __new__(cls, /) -> SchemaRegistry:
        """
        Creates an empty native registry.
        """
    def has(self, /, schema_type: str) -> bool:
        """
        Returns whether a native builder is registered for ``schema_type``.
        """
    def list_types(self, /) -> list[str]:
        """
        Lists the schema types with a registered native builder.
        """

@final
class TwitterPayload:
    """
    Twitter Card metadata for a payload. Read-only.
    """
    @property
    def card(self, /) -> str:
        """
        Card type, for example ``"summary_large_image"``.
        """
    @property
    def creator(self, /) -> str |None:
        """
        Twitter ``@handle`` of the content creator.
        """
    @property
    def description(self, /) -> str |None:
        """
        Twitter description.
        """
    @property
    def image(self, /) -> str |None:
        """
        Absolute URL of the Twitter image.
        """
    @property
    def image_alt(self, /) -> str |None:
        """
        Twitter image alternative text.
        """
    @property
    def site(self, /) -> str |None:
        """
        Twitter ``@handle`` for the site.
        """
    @property
    def title(self, /) -> str |None:
        """
        Twitter title.
        """
    def to_dict(self, /) -> dict:
        """
        Returns the Twitter Card metadata as a dictionary.
        """

@final
class URLPolicy:
    """
    Controls how canonical URLs are normalized.
    """
    def __new__(cls, /, *, enforce_https: bool = True, lowercase_paths: bool = False, trailing_slash: str = "never", collapse_duplicate_slashes: bool = True, strip_tracking_params: bool = True, allowed_query_params: Sequence[str] |None = None, extra_tracking_params: Sequence[str] |None = None) -> URLPolicy:
        """
        Creates a URL policy. All arguments are keyword-only.
        
        Args:
            enforce_https: Rewrite ``http`` to ``https``.
            lowercase_paths: Lowercase path segments. Defaults to ``False``.
            trailing_slash: ``"always"``, ``"never"``, or ``"preserve"``.
            collapse_duplicate_slashes: Collapse repeated slashes.
            strip_tracking_params: Remove tracking parameters.
            allowed_query_params: Query parameters to keep.
            extra_tracking_params: Extra parameter names to strip.
        """
    @property
    def allowed_query_params(self, /) -> list[str]:
        """
        Query parameters to keep when tracking parameters are stripped. A
        listed parameter is kept even when it matches a tracking pattern.
        """
    @property
    def collapse_duplicate_slashes(self, /) -> bool:
        """
        Whether repeated slashes are collapsed.
        """
    @property
    def enforce_https(self, /) -> bool:
        """
        Whether ``http`` is rewritten to ``https``.
        """
    @property
    def extra_tracking_params(self, /) -> list[str]:
        """
        Additional query parameter names to strip, on top of the built-in
        tracking list.
        """
    @property
    def lowercase_paths(self, /) -> bool:
        """
        Whether path segments are lowercased. Off by default: path case is
        preserved because it can be significant.
        """
    @property
    def strip_tracking_params(self, /) -> bool:
        """
        Whether tracking parameters such as ``utm_*`` are removed.
        """
    @property
    def trailing_slash(self, /) -> str:
        """
        Trailing slash policy: ``"always"``, ``"never"``, or ``"preserve"``.
        """

def build_seo_contract(config: SEOContractConfig) -> SEOContract:
    """
    Builds a machine-readable SEO contract.
    
    Raises:
        ContractError: If contract generation fails.
    """

def build_seo_payload(entity: SEOEntity, route: str, config: SEOConfig, overrides: SEOOverrides |None = None) -> SEOPayload:
    """
    Builds a deterministic SEO payload for an entity at a route.
    
    Args:
        entity: Content entity.
        route: Route path, for example ``"/blog/hello"``.
        config: Site-wide configuration.
        overrides: Optional per-call overrides.
    
    Returns:
        The resolved payload.
    
    Raises:
        EaseoError: If the entity, route, or configuration is invalid.
    """

def build_seo_payload_dict(entity: SEOEntity, route: str, config: SEOConfig, overrides: SEOOverrides |None = None) -> dict:
    """
    Builds a payload and returns it as a plain dictionary.
    
    Raises:
        EaseoError: If the entity, route, or configuration is invalid.
    """

def build_seo_payload_with_overrides(entity: SEOEntity, route: str, config: SEOConfig, overrides: SEOOverrides) -> SEOPayload:
    """
    Builds a deterministic SEO payload with explicit overrides.
    
    Prefer :func:`build_seo_payload`; this variant requires ``overrides``.
    
    Raises:
        EaseoError: If the entity, route, overrides, or configuration is
            invalid.
    """

def clean_query_fn(query: str) -> str:
    """
    Removes tracking parameters from a query string.
    """

def clean_url_fn(url: str) -> dict:
    """
    Removes tracking parameters from a URL.
    
    Returns:
        A dict with ``url``, ``removed_params``, and ``cleaned_params``.
    """

def normalize_path_fn(path: str, policy: URLPolicy) -> str:
    """
    Normalizes a route path according to the URL policy.
    
    Raises:
        ValueError: If the path or policy is invalid.
    """

def normalize_public_url_fn(url: str, config: SEOConfig) -> str:
    """
    Resolves a path or URL against the configured public base URL.
    
    Raises:
        EaseoError: If the input or configuration is invalid.
    """

def validate_payload(payload: SEOPayload) -> list[SEOIssue]:
    """
    Runs the built-in validation checks against a payload.
    
    Returns:
        A list of :class:`SEOIssue` findings, empty when the payload passes.
    """

def __getattr__(name: str) -> Incomplete: ...

# Exception types are created dynamically at import time; declare them here
# so IDEs and type checkers can see them.
class EaseoError(ValueError): ...
class InvalidUrlError(EaseoError): ...
class ConfigurationError(EaseoError): ...
class EntityError(EaseoError): ...
class SchemaError(EaseoError): ...
class ContractError(EaseoError): ...
