#![deny(clippy::all)]

use easeo_core as core;
use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;

/// Emit `console.warn` for validation issues when `emitWarnings` is set.
fn emit_warnings(env: &Env, payload: &core::SEOPayload, config: &core::SEOConfig) {
    if !config.emit_warnings {
        return;
    }
    let issues = core::validate_payload(payload);
    if issues.is_empty() {
        return;
    }
    let Ok(global) = env.get_global() else {
        return;
    };
    let Ok(console) = global.get_named_property::<Object>("console") else {
        return;
    };
    let Ok(warn) = console.get_named_property::<Function<JsString, Unknown>>("warn") else {
        return;
    };
    for issue in issues {
        let location = issue
            .url
            .as_deref()
            .map(|u| format!(" ({u})"))
            .unwrap_or_default();
        let message = format!("[{}] {}{}", issue.rule_id, issue.message, location);
        if let Ok(value) = env.create_string(&message) {
            let _ = warn.call(value);
        }
    }
}

// ── Node wrapper for SEOConfig ────────────────────────────────────────

/// Site-wide configuration for payload generation.
#[napi(object)]
pub struct NodeSeoConfig {
    /// Canonical hostname without a scheme, for example `"example.com"`.
    pub canonical_host: String,
    /// Full base URL for path resolution, for example `"https://example.com"`.
    pub public_base_url: String,
    /// Site name for `og:site_name` and title templates.
    pub site_name: Option<String>,
    /// Title template containing `{title}`.
    pub title_template: Option<String>,
    /// Fallback Open Graph image URL.
    pub default_og_image: Option<String>,
    /// Rewrite `http` to `https`. Defaults to `true`.
    pub enforce_https: Option<bool>,
    /// Lowercase path segments. Defaults to `false` (preserve case).
    pub lowercase_paths: Option<bool>,
    /// Trailing slash policy: `"always"`, `"never"`, or `"preserve"`.
    pub trailing_slash: Option<String>,
    /// Collapse repeated slashes. Defaults to `true`.
    pub collapse_duplicate_slashes: Option<bool>,
    /// Remove tracking parameters. Defaults to `true`.
    pub strip_tracking_params: Option<bool>,
    /// Query parameters to keep when tracking parameters are stripped. A
    /// listed parameter is kept even when it matches a tracking pattern.
    pub allowed_query_params: Option<Vec<String>>,
    /// Additional query parameter names to strip, on top of the built-in list.
    pub extra_tracking_params: Option<Vec<String>>,
    /// Open Graph locale, for example `"en_US"`.
    pub locale: Option<String>,
    /// Alternate locales.
    pub locale_alternate: Option<Vec<String>>,
    /// Twitter `@handle` for `twitter:site`.
    pub twitter_site: Option<String>,
    /// Organization or publisher name used in schemas.
    pub publisher_name: Option<String>,
    /// Publisher logo URL used in schemas.
    pub publisher_logo: Option<String>,
    /// Generate JSON-LD from the entity type. Defaults to `true`.
    pub auto_generate_schema: Option<bool>,
    /// Emit `console.warn` for validation issues. Defaults to `false`.
    pub emit_warnings: Option<bool>,
    /// Default `index` directive for regular pages. Defaults to `true`.
    pub default_robots_index: Option<bool>,
    /// Default `follow` directive for regular pages. Defaults to `true`.
    pub default_robots_follow: Option<bool>,
    /// `index` directive for search pages. Defaults to `false`.
    pub search_robots_index: Option<bool>,
    /// `follow` directive for search pages. Defaults to `true`.
    pub search_robots_follow: Option<bool>,
    /// JSON object mapping entity types to schema.org types.
    pub schema_type_map_json: Option<String>,
    /// Search URL template for the homepage `WebSite` `SearchAction`.
    pub search_url_template: Option<String>,
}

impl From<&NodeSeoConfig> for core::SEOConfig {
    fn from(c: &NodeSeoConfig) -> Self {
        let trailing = match c.trailing_slash.as_deref() {
            Some("always") => core::TrailingSlash::Always,
            Some("preserve") => core::TrailingSlash::Preserve,
            _ => core::TrailingSlash::Never,
        };
        Self {
            canonical_host: c.canonical_host.clone(),
            public_base_url: c.public_base_url.clone(),
            url_policy: core::URLPolicy {
                enforce_https: c.enforce_https.unwrap_or(true),
                lowercase_paths: c.lowercase_paths.unwrap_or(false),
                trailing_slash: trailing,
                collapse_duplicate_slashes: c.collapse_duplicate_slashes.unwrap_or(true),
                strip_tracking_params: c.strip_tracking_params.unwrap_or(true),
                allowed_query_params: c.allowed_query_params.clone().unwrap_or_default(),
                extra_tracking_params: c.extra_tracking_params.clone().unwrap_or_default(),
            },
            site_name: c.site_name.clone(),
            title_template: c.title_template.clone(),
            default_robots: core::Robots {
                index: c.default_robots_index.unwrap_or(true),
                follow: c.default_robots_follow.unwrap_or(true),
                ..Default::default()
            },
            default_og_image: c.default_og_image.as_ref().map(|url| core::SEOImage {
                url: url.clone(),
                width: None,
                height: None,
                alt: None,
            }),
            search_robots: core::Robots {
                index: c.search_robots_index.unwrap_or(false),
                follow: c.search_robots_follow.unwrap_or(true),
                ..Default::default()
            },
            schema_type_map: c
                .schema_type_map_json
                .as_ref()
                .and_then(|json| {
                    serde_json::from_str::<std::collections::HashMap<String, Option<String>>>(json)
                        .ok()
                        .map(|m| m.into_iter().collect())
                })
                .unwrap_or_else(|| core::SEOConfig::default().schema_type_map),
            auto_generate_schema: c.auto_generate_schema.unwrap_or(true),
            publisher_name: c.publisher_name.clone(),
            publisher_logo: c.publisher_logo.clone(),
            locale: c.locale.clone(),
            locale_alternate: c.locale_alternate.clone(),
            twitter_site: c.twitter_site.clone(),
            emit_warnings: c.emit_warnings.unwrap_or(false),
            search_url_template: c.search_url_template.clone(),
        }
    }
}

// ── Node wrapper for SEOEntity ────────────────────────────────────────

/// A content entity to generate SEO metadata for.
#[napi(object)]
pub struct NodeSeoEntity {
    /// One of `home`, `post`, `page`, `video`, `taxonomy`, `search`,
    /// `product`, `organization`, `local_business`, `faq`, or `other`.
    pub entity_type: String,
    /// Page title.
    pub title: Option<String>,
    /// Short description. `description` is an alias.
    pub excerpt: Option<String>,
    /// Alias for `excerpt`.
    pub description: Option<String>,
    /// URL slug.
    pub slug: Option<String>,
    /// Full content as HTML, used to derive a snippet when no excerpt is set.
    pub body_html: Option<String>,
    /// Publication status. `"published"` (case-insensitive) keeps the page
    /// indexable; any other value becomes noindex.
    pub status: Option<String>,
    /// Absolute URL of the primary image.
    pub image: Option<String>,
    /// Primary image width in pixels.
    pub image_width: Option<u32>,
    /// Primary image height in pixels.
    pub image_height: Option<u32>,
    /// Primary image alternative text.
    pub image_alt: Option<String>,
    /// Publication date as an ISO date or datetime.
    pub published_at: Option<String>,
    /// Last update date as an ISO date or datetime.
    pub updated_at: Option<String>,
    /// Author display name.
    pub author_name: Option<String>,
    /// Product stock keeping unit.
    pub sku: Option<String>,
    /// Product price as a string.
    pub price: Option<String>,
    /// ISO currency code for the price.
    pub price_currency: Option<String>,
    /// Product availability, for example `"InStock"`.
    pub availability: Option<String>,
    /// Additional URLs used as `sameAs` in organization schemas.
    pub same_as: Option<Vec<String>>,
    /// Postal address for local business schemas.
    pub address: Option<String>,
    /// Breadcrumb trail.
    pub breadcrumbs: Option<Vec<NodeBreadcrumb>>,
    /// Question and answer pairs for FAQ schemas.
    pub faq_items: Option<Vec<NodeFaqItem>>,
}

/// A single entry in a breadcrumb trail.
#[napi(object)]
pub struct NodeBreadcrumb {
    /// Human readable label for the breadcrumb.
    pub name: String,
    /// URL the breadcrumb links to.
    pub url: String,
}

/// A single question and answer pair for FAQ schemas.
#[napi(object)]
pub struct NodeFaqItem {
    /// The question text.
    pub question: String,
    /// The answer text.
    pub answer: String,
}

impl From<&NodeSeoEntity> for core::SEOEntity {
    fn from(e: &NodeSeoEntity) -> Self {
        let et = core::EntityType::from_str(&e.entity_type).unwrap_or(core::EntityType::Page);
        let bcs = e.breadcrumbs.as_ref().map(|v| {
            v.iter()
                .map(|b| core::Breadcrumb {
                    name: b.name.clone(),
                    url: b.url.clone(),
                })
                .collect()
        });
        let faq = e.faq_items.as_ref().map(|v| {
            v.iter()
                .map(|f| core::FAQItem {
                    question: f.question.clone(),
                    answer: f.answer.clone(),
                })
                .collect()
        });
        Self {
            entity_type: et,
            title: e.title.clone(),
            excerpt: e.excerpt.clone().or_else(|| e.description.clone()),
            slug: e.slug.clone(),
            body_html: e.body_html.clone(),
            status: e.status.clone(),
            featured_image: e.image.as_ref().map(|url| core::SEOImage {
                url: url.clone(),
                width: e.image_width,
                height: e.image_height,
                alt: e.image_alt.clone(),
            }),
            published_at: e.published_at.clone(),
            updated_at: e.updated_at.clone(),
            author_name: e.author_name.clone(),
            breadcrumbs: bcs,
            sku: e.sku.clone(),
            price: e.price.clone(),
            price_currency: e.price_currency.clone(),
            availability: e.availability.clone(),
            same_as: e.same_as.clone(),
            address: e.address.clone(),
            faq_items: faq,
        }
    }
}

// ── Node output types ─────────────────────────────────────────────────

/// Open Graph metadata for a payload.
#[napi(object)]
#[derive(Clone)]
pub struct NodeOpenGraph {
    /// Open Graph object type, for example `"article"` or `"website"`.
    #[napi(js_name = "type")]
    pub og_type: String,
    /// Open Graph title.
    pub title: Option<String>,
    /// Open Graph description.
    pub description: Option<String>,
    /// Open Graph canonical URL.
    pub url: Option<String>,
    /// Absolute URL of the Open Graph image.
    pub image: Option<String>,
    /// Open Graph image width in pixels.
    pub image_width: Option<u32>,
    /// Open Graph image height in pixels.
    pub image_height: Option<u32>,
    /// Open Graph image alternative text.
    pub image_alt: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Locale, for example `"en_US"`.
    pub locale: Option<String>,
    /// Alternate locales.
    pub locale_alternate: Option<Vec<String>>,
    /// Open Graph audio URL.
    pub audio: Option<String>,
    /// Open Graph video URL.
    pub video: Option<String>,
}

/// Twitter Card metadata for a payload.
#[napi(object)]
#[derive(Clone)]
pub struct NodeTwitter {
    /// Card type, for example `"summary_large_image"`.
    pub card: String,
    /// Twitter title.
    pub title: Option<String>,
    /// Twitter description.
    pub description: Option<String>,
    /// Absolute URL of the Twitter image.
    pub image: Option<String>,
    /// Twitter image alternative text.
    pub image_alt: Option<String>,
    /// Twitter `@handle` for the site.
    pub site: Option<String>,
    /// Twitter `@handle` of the content creator.
    pub creator: Option<String>,
}

/// The resolved, deterministic SEO payload.
#[napi]
#[derive(Clone)]
pub struct NodeSeoPayload {
    title: String,
    description: String,
    canonical: String,
    robots: String,
    open_graph: NodeOpenGraph,
    twitter: NodeTwitter,
    schema_json_ld: Option<serde_json::Value>,
    extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[napi]
impl NodeSeoPayload {
    /// Resolved title.
    #[napi(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// Resolved description.
    #[napi(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Normalized canonical URL.
    #[napi(getter)]
    pub fn canonical(&self) -> String {
        self.canonical.clone()
    }

    /// Serialized robots directives, for example `"index,follow"`.
    #[napi(getter)]
    pub fn robots(&self) -> String {
        self.robots.clone()
    }

    /// Open Graph metadata.
    #[napi(getter, js_name = "openGraph")]
    pub fn open_graph(&self) -> NodeOpenGraph {
        self.open_graph.clone()
    }

    /// Twitter Card metadata.
    #[napi(getter)]
    pub fn twitter(&self) -> NodeTwitter {
        self.twitter.clone()
    }

    /// Generated JSON-LD, when schema generation is enabled.
    #[napi(getter, js_name = "schemaJsonLd")]
    pub fn schema_json_ld(&self) -> Option<serde_json::Value> {
        self.schema_json_ld.clone()
    }

    /// Extra fields added by config-scoped hooks.
    #[napi(getter)]
    pub fn extra(&self) -> std::collections::BTreeMap<String, serde_json::Value> {
        self.extra.clone()
    }

    /// Renders the full head snippet: title, description, canonical, robots,
    /// Open Graph, Twitter Cards, and JSON-LD.
    #[napi]
    pub fn render_html(&self) -> Result<String> {
        let payload = self.to_core();
        payload
            .render_html()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Renders only the Open Graph meta tags.
    #[napi]
    pub fn render_opengraph(&self) -> String {
        let payload = self.to_core();
        payload.render_opengraph()
    }

    /// Renders only the Twitter Card meta tags.
    #[napi]
    pub fn render_twitter(&self) -> String {
        let payload = self.to_core();
        payload.render_twitter()
    }

    /// Renders only the JSON-LD script tag.
    #[napi]
    pub fn render_jsonld(&self) -> Result<String> {
        let payload = self.to_core();
        payload
            .render_jsonld()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the payload as a pretty printed JSON string.
    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String> {
        let payload = self.to_core();
        payload
            .to_json_pretty()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the payload as a plain object.
    #[napi]
    pub fn to_object(&self) -> Result<serde_json::Value> {
        let payload = self.to_core();
        payload
            .to_dict()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the SHA-256 hash of the payload.
    #[napi]
    pub fn hash(&self) -> Result<String> {
        let payload = self.to_core();
        core::hash_payload(&payload).map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the payload hash as a quoted HTTP ETag.
    #[napi]
    pub fn etag(&self) -> Result<String> {
        let payload = self.to_core();
        core::etag_payload(&payload).map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

impl NodeSeoPayload {
    fn to_core(&self) -> core::SEOPayload {
        let og = core::OGPayload {
            og_type: self.open_graph.og_type.clone(),
            title: self.open_graph.title.clone(),
            description: self.open_graph.description.clone(),
            url: self.open_graph.url.clone(),
            image: self.open_graph.image.clone(),
            image_width: self.open_graph.image_width,
            image_height: self.open_graph.image_height,
            image_alt: self.open_graph.image_alt.clone(),
            site_name: self.open_graph.site_name.clone(),
            locale: self.open_graph.locale.clone(),
            locale_alternate: self.open_graph.locale_alternate.clone(),
            audio: self.open_graph.audio.clone(),
            video: self.open_graph.video.clone(),
        };
        let twitter = core::TwitterPayload {
            card: self.twitter.card.clone(),
            title: self.twitter.title.clone(),
            description: self.twitter.description.clone(),
            image: self.twitter.image.clone(),
            image_alt: self.twitter.image_alt.clone(),
            site: self.twitter.site.clone(),
            creator: self.twitter.creator.clone(),
        };
        let schema = self.schema_json_ld.clone();
        core::SEOPayload {
            title: self.title.clone(),
            description: self.description.clone(),
            canonical: self.canonical.clone(),
            robots: self.robots.clone(),
            og,
            twitter,
            schema_jsonld: schema,
            extra: self.extra.clone(),
        }
    }

    /// Build a Node payload from its core equivalent.
    pub fn from_core(payload: core::SEOPayload) -> Self {
        Self {
            title: payload.title,
            description: payload.description,
            canonical: payload.canonical,
            robots: payload.robots,
            open_graph: NodeOpenGraph {
                og_type: payload.og.og_type,
                title: payload.og.title,
                description: payload.og.description,
                url: payload.og.url,
                image: payload.og.image,
                image_width: payload.og.image_width,
                image_height: payload.og.image_height,
                image_alt: payload.og.image_alt,
                site_name: payload.og.site_name,
                locale: payload.og.locale,
                locale_alternate: payload.og.locale_alternate,
                audio: payload.og.audio,
                video: payload.og.video,
            },
            twitter: NodeTwitter {
                card: payload.twitter.card,
                title: payload.twitter.title,
                description: payload.twitter.description,
                image: payload.twitter.image,
                image_alt: payload.twitter.image_alt,
                site: payload.twitter.site,
                creator: payload.twitter.creator,
            },
            schema_json_ld: payload.schema_jsonld,
            extra: payload.extra,
        }
    }
}

// ── Main build function ───────────────────────────────────────────────

/// Builds a deterministic SEO payload for an entity at a route.
#[napi]
pub fn build_seo_payload(
    env: Env,
    entity: NodeSeoEntity,
    route: String,
    config: NodeSeoConfig,
) -> Result<NodeSeoPayload> {
    let core_config: core::SEOConfig = (&config).into();
    core_config
        .validate()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    core::EntityType::from_str(&entity.entity_type)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let core_entity: core::SEOEntity = (&entity).into();

    let payload = core::build_seo_payload(&core_entity, &route, &core_config)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    emit_warnings(&env, &payload, &core_config);
    Ok(NodeSeoPayload::from_core(payload))
}

/// Rebuild a payload from its wire dict. Used by the JS hook layer, which
/// post-processes `toObject()` and needs the result to flow back into
/// `hash()`, `renderHtml()`, `etag()`, etc.
#[napi]
pub fn payload_from_dict(dict: serde_json::Value) -> Result<NodeSeoPayload> {
    let payload =
        core::SEOPayload::from_dict(&dict).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(NodeSeoPayload::from_core(payload))
}

// ── Node wrapper for SEOOverrides ────────────────────────────────────

/// Per-call overrides that take precedence over the entity and config.
#[napi(object)]
pub struct NodeSeoOverrides {
    /// Overrides the resolved title.
    pub meta_title: Option<String>,
    /// Overrides the resolved description.
    pub meta_description: Option<String>,
    /// Overrides the resolved canonical URL. Trusted: used as-is after an
    /// absolute `http(s)` URL check, bypassing the URL policy.
    pub canonical_url: Option<String>,
    /// Overrides the route used to build the canonical URL, through the full
    /// URL normalization pipeline. Ignored when `canonical_url` is set.
    pub canonical_path: Option<String>,
    /// Overrides the robots `index` directive.
    pub robots_index: Option<bool>,
    /// Overrides the robots `follow` directive.
    pub robots_follow: Option<bool>,
    /// Overrides the robots `max-snippet` directive.
    pub robots_max_snippet: Option<i32>,
    /// Overrides the robots `max-image-preview` directive.
    pub robots_max_image_preview: Option<String>,
    /// Overrides the robots `max-video-preview` directive.
    pub robots_max_video_preview: Option<i32>,
    /// Overrides the Open Graph title.
    pub og_title: Option<String>,
    /// Overrides the Open Graph description.
    pub og_description: Option<String>,
    /// Overrides the Open Graph image URL.
    pub og_image_url: Option<String>,
    /// Open Graph image width in pixels.
    pub og_image_width: Option<u32>,
    /// Open Graph image height in pixels.
    pub og_image_height: Option<u32>,
    /// Open Graph image alternative text.
    pub og_image_alt: Option<String>,
    /// Overrides the Twitter Card type.
    pub twitter_card: Option<String>,
    /// Overrides the Twitter title.
    pub twitter_title: Option<String>,
    /// Overrides the Twitter description.
    pub twitter_description: Option<String>,
    /// Overrides the Twitter image URL.
    pub twitter_image_url: Option<String>,
    /// Replaces the generated JSON-LD schema.
    pub schema_jsonld: Option<serde_json::Value>,
    /// When `true`, no JSON-LD is emitted.
    pub omit_schema: Option<bool>,
    /// When `true`, the config title template is skipped.
    pub skip_title_template: Option<bool>,
    /// Overrides the Twitter creator handle.
    pub twitter_creator: Option<String>,
    /// Open Graph audio URL.
    pub og_audio: Option<String>,
    /// Open Graph video URL.
    pub og_video: Option<String>,
}

impl From<&NodeSeoOverrides> for core::SEOOverrides {
    fn from(o: &NodeSeoOverrides) -> Self {
        let robots = match (o.robots_index, o.robots_follow) {
            (Some(index), Some(follow)) => Some(core::Robots {
                index,
                follow,
                max_snippet: o.robots_max_snippet,
                max_image_preview: o.robots_max_image_preview.clone(),
                max_video_preview: o.robots_max_video_preview,
            }),
            (Some(index), None) => Some(core::Robots {
                index,
                follow: true,
                max_snippet: o.robots_max_snippet,
                max_image_preview: o.robots_max_image_preview.clone(),
                max_video_preview: o.robots_max_video_preview,
            }),
            (None, Some(follow)) => Some(core::Robots {
                index: true,
                follow,
                max_snippet: o.robots_max_snippet,
                max_image_preview: o.robots_max_image_preview.clone(),
                max_video_preview: o.robots_max_video_preview,
            }),
            (None, None) => None,
        };
        let og_image = o.og_image_url.as_ref().map(|url| core::SEOImage {
            url: url.clone(),
            width: o.og_image_width,
            height: o.og_image_height,
            alt: o.og_image_alt.clone(),
        });
        let twitter_image = o.twitter_image_url.as_ref().map(|url| core::SEOImage {
            url: url.clone(),
            width: None,
            height: None,
            alt: None,
        });
        Self {
            meta_title: o.meta_title.clone(),
            meta_description: o.meta_description.clone(),
            canonical_url: o.canonical_url.clone(),
            canonical_path: o.canonical_path.clone(),
            robots,
            og_title: o.og_title.clone(),
            og_description: o.og_description.clone(),
            og_image,
            twitter_card: o.twitter_card.clone(),
            twitter_title: o.twitter_title.clone(),
            twitter_description: o.twitter_description.clone(),
            twitter_image,
            schema_jsonld: o.schema_jsonld.clone(),
            omit_schema: o.omit_schema.unwrap_or(false),
            skip_title_template: o.skip_title_template.unwrap_or(false),
            twitter_creator: o.twitter_creator.clone(),
            og_audio: o.og_audio.clone(),
            og_video: o.og_video.clone(),
        }
    }
}

// ── Build with overrides ─────────────────────────────────────────────

/// Builds a deterministic SEO payload with explicit overrides.
#[napi]
pub fn build_seo_payload_with_overrides(
    env: Env,
    entity: NodeSeoEntity,
    route: String,
    config: NodeSeoConfig,
    overrides: NodeSeoOverrides,
) -> Result<NodeSeoPayload> {
    let core_config: core::SEOConfig = (&config).into();
    core_config
        .validate()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    core::EntityType::from_str(&entity.entity_type)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let core_entity: core::SEOEntity = (&entity).into();
    let core_overrides: core::SEOOverrides = (&overrides).into();

    let payload =
        core::build_seo_payload_with_overrides(&core_entity, &route, &core_config, &core_overrides)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    emit_warnings(&env, &payload, &core_config);
    Ok(NodeSeoPayload::from_core(payload))
}

// ── Contract ──────────────────────────────────────────────────────────

/// Input configuration for building a contract.
#[napi(object)]
pub struct NodeSeoContractConfig {
    /// Canonical hostname without a scheme.
    pub canonical_host: String,
    /// URL scheme, defaults to `"https"`.
    pub scheme: Option<String>,
    /// Default expectations applied to every route.
    pub defaults: Option<NodeSeoExpectation>,
    /// Route specific rules.
    pub rules: Option<Vec<NodeSeoContractRule>>,
    /// JSON object of per-route expectation overrides.
    pub exceptions_json: Option<String>,
}

/// A contract rule matched against route paths.
#[napi(object)]
#[derive(Clone)]
pub struct NodeSeoContractRule {
    /// Route pattern, for example `"/blog/*"`.
    pub r#match: String,
    /// Expectations applied when the pattern matches.
    pub expect: NodeSeoExpectation,
    /// Optional severity: `"error"`, `"warning"`, or `"info"`.
    pub severity: Option<String>,
}

/// A generated, machine-readable SEO contract.
#[napi]
#[derive(Clone)]
pub struct NodeSeoContract {
    contract_version: String,
    generator_name: String,
    generator_version: String,
    site: NodeContractSite,
    defaults: NodeSeoExpectation,
    rules: Vec<NodeSeoContractRule>,
    exceptions: std::collections::BTreeMap<String, NodeSeoExpectation>,
}

#[napi]
impl NodeSeoContract {
    /// Version of the contract format itself.
    #[napi(getter, js_name = "contractVersion")]
    pub fn contract_version(&self) -> String {
        self.contract_version.clone()
    }

    /// Generator name.
    #[napi(getter, js_name = "generatorName")]
    pub fn generator_name(&self) -> String {
        self.generator_name.clone()
    }

    /// Generator version.
    #[napi(getter, js_name = "generatorVersion")]
    pub fn generator_version(&self) -> String {
        self.generator_version.clone()
    }

    /// Site identity stored in the contract.
    #[napi(getter)]
    pub fn site(&self) -> NodeContractSite {
        self.site.clone()
    }

    /// Default expectations applied to every route.
    #[napi(getter)]
    pub fn defaults(&self) -> NodeSeoExpectation {
        self.defaults.clone()
    }

    /// Route specific rules.
    #[napi(getter)]
    pub fn rules(&self) -> Vec<NodeSeoContractRule> {
        self.rules.clone()
    }

    /// Per-route expectation overrides.
    #[napi(getter)]
    pub fn exceptions(&self) -> std::collections::BTreeMap<String, NodeSeoExpectation> {
        self.exceptions.clone()
    }

    /// Returns the SHA-256 hash of the serialized contract.
    #[napi]
    pub fn hash(&self) -> Result<String> {
        let contract = self.to_core();
        contract
            .hash()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the contract as pretty printed JSON.
    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String> {
        let contract = self.to_core();
        contract
            .to_json()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Returns the contract as a plain object.
    #[napi]
    pub fn to_dict(&self) -> Result<serde_json::Value> {
        let contract = self.to_core();
        contract
            .to_dict()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

impl NodeSeoContract {
    fn to_core(&self) -> core::SEOContract {
        core::SEOContract {
            contract_version: self.contract_version.clone(),
            generator: core::ContractGenerator {
                name: self.generator_name.clone(),
                version: self.generator_version.clone(),
            },
            site: core::ContractSite {
                canonical_host: self.site.canonical_host.clone(),
                scheme: self.site.scheme.clone(),
            },
            defaults: self.defaults.to_core(),
            rules: self.rules.iter().map(|r| r.to_core()).collect(),
            exceptions: self
                .exceptions
                .iter()
                .map(|(k, v)| (k.clone(), v.to_core()))
                .collect(),
        }
    }
}

/// Site identity stored in a contract.
#[napi(object)]
#[derive(Clone)]
pub struct NodeContractSite {
    /// Canonical hostname.
    pub canonical_host: String,
    /// URL scheme.
    pub scheme: String,
}

/// Builds a machine-readable SEO contract.
#[napi]
pub fn build_seo_contract(config: NodeSeoContractConfig) -> Result<NodeSeoContract> {
    let defaults = config.defaults.map(|d| d.to_core()).unwrap_or_default();
    let rules = config
        .rules
        .map(|r| r.iter().map(|r| r.to_core()).collect())
        .unwrap_or_default();
    let exceptions = config
        .exceptions_json
        .as_ref()
        .and_then(|json| {
            serde_json::from_str::<std::collections::BTreeMap<String, core::SEOExpectation>>(json)
                .ok()
        })
        .unwrap_or_default();
    let cfg = core::SEOContractConfig {
        canonical_host: config.canonical_host,
        scheme: config.scheme.unwrap_or_else(|| "https".to_string()),
        defaults,
        rules,
        exceptions,
    };
    let contract =
        core::build_seo_contract(&cfg).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(NodeSeoContract {
        contract_version: contract.contract_version,
        generator_name: contract.generator.name,
        generator_version: contract.generator.version,
        site: NodeContractSite {
            canonical_host: contract.site.canonical_host,
            scheme: contract.site.scheme,
        },
        defaults: NodeSeoExpectation::from_core(&contract.defaults),
        rules: contract
            .rules
            .iter()
            .map(NodeSeoContractRule::from_core)
            .collect(),
        exceptions: contract
            .exceptions
            .iter()
            .map(|(k, v)| (k.clone(), NodeSeoExpectation::from_core(v)))
            .collect(),
    })
}

// ── Node wrapper for SEOExpectation ────────────────────────────────────

/// Expectations applied to a page, a rule match, or contract defaults.
#[napi(object)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSeoExpectation {
    /// The field must be present.
    pub required: Option<bool>,
    /// The field must be absent.
    pub forbidden: Option<bool>,
    /// The field must equal this value.
    pub equals: Option<String>,
    /// The field must not equal this value.
    pub not_equals: Option<String>,
    /// The field must contain this substring.
    pub contains: Option<String>,
    /// The field must match this regular expression.
    pub matches: Option<String>,
    /// The field must be one of these values.
    pub one_of: Option<Vec<String>>,
    /// Minimum string length.
    pub min_length: Option<u32>,
    /// Maximum string length.
    pub max_length: Option<u32>,
    /// Minimum number of items.
    pub min_items: Option<u32>,
    /// Maximum number of items.
    pub max_items: Option<u32>,
    /// Whether the page must be indexable.
    pub indexable: Option<bool>,
    /// Expected canonical behavior, for example `"self"`.
    pub canonical: Option<String>,
    /// Whether a JSON-LD schema must be present.
    pub schema_required: Option<bool>,
    /// Required schema.org types.
    pub schema_types: Option<Vec<String>>,
    /// Whether Open Graph metadata must be present.
    pub og_required: Option<bool>,
    /// Whether Twitter Card metadata must be present.
    pub twitter_required: Option<bool>,
    /// Whether a sitemap entry must be present.
    pub sitemap_required: Option<bool>,
    /// Whether hreflang annotations must be present.
    pub hreflang_required: Option<bool>,
    /// Nested expectations for the title.
    pub title: Option<serde_json::Value>,
    /// Nested expectations for the description.
    pub description: Option<serde_json::Value>,
}

impl NodeSeoExpectation {
    fn to_core(&self) -> core::SEOExpectation {
        core::SEOExpectation {
            required: self.required,
            forbidden: self.forbidden,
            equals: self.equals.clone(),
            not_equals: self.not_equals.clone(),
            contains: self.contains.clone(),
            matches: self.matches.clone(),
            one_of: self.one_of.clone(),
            min_length: self.min_length.map(|v| v as usize),
            max_length: self.max_length.map(|v| v as usize),
            min_items: self.min_items.map(|v| v as usize),
            max_items: self.max_items.map(|v| v as usize),
            indexable: self.indexable,
            canonical: self.canonical.clone(),
            schema: self
                .schema_required
                .map(|required| core::SchemaExpectation {
                    required: Some(required),
                    types: self.schema_types.clone(),
                }),
            open_graph: self.og_required.map(|required| core::FieldExpectation {
                required: Some(required),
            }),
            twitter: self
                .twitter_required
                .map(|required| core::FieldExpectation {
                    required: Some(required),
                }),
            sitemap: self
                .sitemap_required
                .map(|required| core::FieldExpectation {
                    required: Some(required),
                }),
            hreflang: self
                .hreflang_required
                .map(|required| core::FieldExpectation {
                    required: Some(required),
                }),
            title: self.title.as_ref().and_then(|v| {
                serde_json::from_value::<NodeSeoExpectation>(v.clone())
                    .ok()
                    .map(|n| Box::new(n.to_core()))
            }),
            description: self.description.as_ref().and_then(|v| {
                serde_json::from_value::<NodeSeoExpectation>(v.clone())
                    .ok()
                    .map(|n| Box::new(n.to_core()))
            }),
        }
    }

    fn from_core(e: &core::SEOExpectation) -> Self {
        Self {
            required: e.required,
            forbidden: e.forbidden,
            equals: e.equals.clone(),
            not_equals: e.not_equals.clone(),
            contains: e.contains.clone(),
            matches: e.matches.clone(),
            one_of: e.one_of.clone(),
            min_length: e.min_length.and_then(|v| u32::try_from(v).ok()),
            max_length: e.max_length.and_then(|v| u32::try_from(v).ok()),
            min_items: e.min_items.and_then(|v| u32::try_from(v).ok()),
            max_items: e.max_items.and_then(|v| u32::try_from(v).ok()),
            indexable: e.indexable,
            canonical: e.canonical.clone(),
            schema_required: e.schema.as_ref().and_then(|s| s.required),
            schema_types: e.schema.as_ref().and_then(|s| s.types.clone()),
            og_required: e.open_graph.as_ref().and_then(|o| o.required),
            twitter_required: e.twitter.as_ref().and_then(|t| t.required),
            sitemap_required: e.sitemap.as_ref().and_then(|s| s.required),
            hreflang_required: e.hreflang.as_ref().and_then(|h| h.required),
            title: e.title.as_ref().map(|t| {
                let node = NodeSeoExpectation::from_core(t);
                serde_json::to_value(&node).unwrap_or(serde_json::Value::Null)
            }),
            description: e.description.as_ref().map(|d| {
                let node = NodeSeoExpectation::from_core(d);
                serde_json::to_value(&node).unwrap_or(serde_json::Value::Null)
            }),
        }
    }
}

impl NodeSeoContractRule {
    fn to_core(&self) -> core::SEOContractRule {
        core::SEOContractRule {
            r#match: self.r#match.clone(),
            expect: self.expect.to_core(),
            severity: match self.severity.as_deref() {
                Some("error") => Some(core::ContractSeverity::Error),
                Some("warning") => Some(core::ContractSeverity::Warning),
                Some("info") => Some(core::ContractSeverity::Info),
                _ => None,
            },
        }
    }

    fn from_core(r: &core::SEOContractRule) -> Self {
        Self {
            r#match: r.r#match.clone(),
            expect: NodeSeoExpectation::from_core(&r.expect),
            severity: match r.severity {
                Some(core::ContractSeverity::Error) => Some("error".to_string()),
                Some(core::ContractSeverity::Warning) => Some("warning".to_string()),
                Some(core::ContractSeverity::Info) => Some("info".to_string()),
                None => None,
            },
        }
    }
}

// ── Validation ────────────────────────────────────────────────────────

/// A single validation finding.
#[napi(object)]
pub struct NodeSeoIssue {
    /// Stable rule identifier, for example `"EASEO101"`.
    pub rule_id: String,
    /// Severity: `"error"`, `"warning"`, or `"info"`.
    pub severity: String,
    /// Human readable description of the finding.
    pub message: String,
    /// Canonical URL the finding applies to.
    pub url: Option<String>,
    /// Additional structured details about the finding.
    pub details: std::collections::BTreeMap<String, serde_json::Value>,
}

/// Runs the built-in validation checks against a payload.
#[napi]
pub fn validate_payload(payload: &NodeSeoPayload) -> Vec<NodeSeoIssue> {
    let core_payload = payload.to_core();
    core::validate_payload(&core_payload)
        .iter()
        .map(|i| NodeSeoIssue {
            rule_id: i.rule_id.clone(),
            severity: match i.severity {
                core::validation::Severity::Error => "error".to_string(),
                core::validation::Severity::Warning => "warning".to_string(),
                core::validation::Severity::Info => "info".to_string(),
            },
            message: i.message.clone(),
            url: i.url.clone(),
            details: i.details.clone(),
        })
        .collect()
}

// ── URL normalization ─────────────────────────────────────────────────

/// Normalizes a route path according to the given policy fields.
#[napi]
#[allow(clippy::too_many_arguments)]
pub fn normalize_path(
    path: String,
    enforce_https: Option<bool>,
    lowercase_paths: Option<bool>,
    trailing_slash: Option<String>,
    collapse_duplicate_slashes: Option<bool>,
    strip_tracking_params: Option<bool>,
    allowed_query_params: Option<Vec<String>>,
    extra_tracking_params: Option<Vec<String>>,
) -> Result<String> {
    let policy = core::URLPolicy {
        enforce_https: enforce_https.unwrap_or(true),
        lowercase_paths: lowercase_paths.unwrap_or(false),
        trailing_slash: match trailing_slash.as_deref() {
            Some("always") => core::TrailingSlash::Always,
            Some("preserve") => core::TrailingSlash::Preserve,
            _ => core::TrailingSlash::Never,
        },
        collapse_duplicate_slashes: collapse_duplicate_slashes.unwrap_or(true),
        strip_tracking_params: strip_tracking_params.unwrap_or(true),
        allowed_query_params: allowed_query_params.unwrap_or_default(),
        extra_tracking_params: extra_tracking_params.unwrap_or_default(),
    };
    core::url::normalize_path(&path, &policy).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Resolves a path or URL against the configured public base URL.
#[napi]
pub fn normalize_public_url(url_or_path: String, config: NodeSeoConfig) -> Result<String> {
    let core_config: core::SEOConfig = (&config).into();
    core::url::normalize_public_url(&url_or_path, &core_config)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

// ── detrack ───────────────────────────────────────────────────────────

/// Result of cleaning a URL's tracking parameters.
#[napi(object)]
pub struct NodeCleanResult {
    /// URL with tracking parameters removed.
    pub url: String,
    /// Parameters that were removed, keyed by parameter name.
    pub removed_params: std::collections::BTreeMap<String, String>,
    /// Parameters that were kept, keyed by parameter name.
    pub cleaned_params: std::collections::BTreeMap<String, String>,
}

/// Removes tracking parameters from a URL.
#[napi]
pub fn clean_url(url: String) -> NodeCleanResult {
    let result = core::detrack::clean_url(&url);
    NodeCleanResult {
        url: result.url,
        removed_params: result.removed_params,
        cleaned_params: result.cleaned_params,
    }
}

/// Removes tracking parameters from a query string.
#[napi]
pub fn clean_query(query: String) -> String {
    core::detrack::clean_query(&query)
}

// ── Schema Registry (JS can't store callbacks in Rust, tracks type names) ──

use std::sync::Mutex;

static GLOBAL_REGISTRY: Mutex<Option<NodeSchemaRegistryInner>> = Mutex::new(None);

struct NodeSchemaRegistryInner {
    types: std::collections::BTreeSet<String>,
}

/// Native introspection handle for the Rust schema registry.
///
/// JavaScript callables are registered through the `SchemaRegistry` class
/// exported by `@easeo/core`, which stores them and applies the generated
/// schema around the build. This native type only exposes `has` and
/// `listTypes`.
#[napi]
pub struct NodeSchemaRegistry;

#[napi]
impl NodeSchemaRegistry {
    // Custom schema generators are registered from JavaScript through the
    // `SchemaRegistry` class exported by `@easeo/core`, which stores
    // callables and applies them around the build. This native type is
    // introspection only.

    /// Returns whether a native builder is registered for the schema type.
    #[napi]
    pub fn has(&self, schema_type: String) -> bool {
        let guard = GLOBAL_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .as_ref()
            .is_some_and(|r| r.types.contains(&schema_type))
    }

    /// Lists the schema types with a registered native builder.
    #[napi]
    pub fn list_types(&self) -> Vec<String> {
        let guard = GLOBAL_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .as_ref()
            .map_or_else(Vec::new, |r| r.types.iter().cloned().collect())
    }
}

/// Returns the native schema registry introspection handle.
#[napi]
pub fn get_schema_registry() -> NodeSchemaRegistry {
    NodeSchemaRegistry
}
