#![deny(clippy::all)]

use easeo_core as core;
use napi::{Env, JsFunction, JsObject};
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
    let Ok(console) = global.get_named_property::<JsObject>("console") else {
        return;
    };
    let Ok(warn) = console.get_named_property::<JsFunction>("warn") else {
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
            let _ = warn.call(None, &[value]);
        }
    }
}

// ── Node wrapper for SEOConfig ────────────────────────────────────────

#[napi(object)]
pub struct NodeSEOConfig {
    pub canonical_host: String,
    pub public_base_url: String,
    pub site_name: Option<String>,
    pub title_template: Option<String>,
    pub default_og_image: Option<String>,
    pub enforce_https: Option<bool>,
    pub lowercase_paths: Option<bool>,
    pub trailing_slash: Option<String>,
    pub collapse_duplicate_slashes: Option<bool>,
    pub strip_tracking_params: Option<bool>,
    pub allowed_query_params: Option<Vec<String>>,
    pub locale: Option<String>,
    pub locale_alternate: Option<Vec<String>>,
    pub twitter_site: Option<String>,
    pub publisher_name: Option<String>,
    pub publisher_logo: Option<String>,
    pub auto_generate_schema: Option<bool>,
    pub emit_warnings: Option<bool>,
    pub default_robots_index: Option<bool>,
    pub default_robots_follow: Option<bool>,
    pub search_robots_index: Option<bool>,
    pub search_robots_follow: Option<bool>,
    pub schema_type_map_json: Option<String>,
    pub search_url_template: Option<String>,
}

impl From<&NodeSEOConfig> for core::SEOConfig {
    fn from(c: &NodeSEOConfig) -> Self {
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
                lowercase_paths: c.lowercase_paths.unwrap_or(true),
                trailing_slash: trailing,
                collapse_duplicate_slashes: c.collapse_duplicate_slashes.unwrap_or(true),
                strip_tracking_params: c.strip_tracking_params.unwrap_or(true),
                allowed_query_params: c.allowed_query_params.clone().unwrap_or_default(),
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

#[napi(object)]
pub struct NodeSEOEntity {
    pub entity_type: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub body_html: Option<String>,
    pub status: Option<String>,
    pub image: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_alt: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    pub author_name: Option<String>,
    pub sku: Option<String>,
    pub price: Option<String>,
    pub price_currency: Option<String>,
    pub availability: Option<String>,
    pub same_as: Option<Vec<String>>,
    pub address: Option<String>,
    pub breadcrumbs: Option<Vec<NodeBreadcrumb>>,
    pub faq_items: Option<Vec<NodeFAQItem>>,
}

#[napi(object)]
pub struct NodeBreadcrumb {
    pub name: String,
    pub url: String,
}

#[napi(object)]
pub struct NodeFAQItem {
    pub question: String,
    pub answer: String,
}

impl From<&NodeSEOEntity> for core::SEOEntity {
    fn from(e: &NodeSEOEntity) -> Self {
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

#[napi(object)]
#[derive(Clone)]
pub struct NodeOpenGraph {
    #[napi(js_name = "type")]
    pub og_type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_alt: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    pub locale_alternate: Option<Vec<String>>,
    pub audio: Option<String>,
    pub video: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct NodeTwitter {
    pub card: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub site: Option<String>,
    pub creator: Option<String>,
}

#[napi]
#[derive(Clone)]
pub struct NodeSEOPayload {
    pub title: String,
    pub description: String,
    pub canonical: String,
    pub robots: String,
    pub open_graph: NodeOpenGraph,
    pub twitter: NodeTwitter,
    pub schema_json_ld: Option<serde_json::Value>,
    /// Extra fields added by config-scoped hooks. Kept sorted for
    /// deterministic serialization.
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[napi]
impl NodeSEOPayload {
    #[napi]
    pub fn render_html(&self) -> Result<String, napi::Error> {
        let payload = self.to_core();
        payload
            .render_html()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn render_opengraph(&self) -> String {
        let payload = self.to_core();
        payload.render_opengraph()
    }

    #[napi]
    pub fn render_twitter(&self) -> String {
        let payload = self.to_core();
        payload.render_twitter()
    }

    #[napi]
    pub fn render_jsonld(&self) -> Result<String, napi::Error> {
        let payload = self.to_core();
        payload
            .render_jsonld()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String, napi::Error> {
        let payload = self.to_core();
        payload
            .to_json_pretty()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn to_object(&self) -> Result<serde_json::Value, napi::Error> {
        let payload = self.to_core();
        payload
            .to_dict()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn hash(&self) -> Result<String, napi::Error> {
        let payload = self.to_core();
        core::hash_payload(&payload).map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn etag(&self) -> Result<String, napi::Error> {
        let payload = self.to_core();
        core::etag_payload(&payload).map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

impl NodeSEOPayload {
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

#[napi]
pub fn build_seo_payload(
    env: Env,
    entity: NodeSEOEntity,
    route: String,
    config: NodeSEOConfig,
) -> Result<NodeSEOPayload, napi::Error> {
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
    Ok(NodeSEOPayload::from_core(payload))
}

/// Rebuild a payload from its wire dict. Used by the JS hook layer, which
/// post-processes `toObject()` and needs the result to flow back into
/// `hash()`, `renderHtml()`, `etag()`, etc.
#[napi]
pub fn payload_from_dict(dict: serde_json::Value) -> Result<NodeSEOPayload, napi::Error> {
    let payload =
        core::SEOPayload::from_dict(&dict).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(NodeSEOPayload::from_core(payload))
}

// ── Node wrapper for SEOOverrides ────────────────────────────────────

#[napi(object)]
pub struct NodeSEOOverrides {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub robots_index: Option<bool>,
    pub robots_follow: Option<bool>,
    pub robots_max_snippet: Option<i32>,
    pub robots_max_image_preview: Option<String>,
    pub robots_max_video_preview: Option<i32>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub og_image_width: Option<u32>,
    pub og_image_height: Option<u32>,
    pub og_image_alt: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image_url: Option<String>,
    pub schema_jsonld: Option<serde_json::Value>,
    pub omit_schema: Option<bool>,
    pub skip_title_template: Option<bool>,
    pub twitter_creator: Option<String>,
    pub og_audio: Option<String>,
    pub og_video: Option<String>,
}

impl From<&NodeSEOOverrides> for core::SEOOverrides {
    fn from(o: &NodeSEOOverrides) -> Self {
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

#[napi]
pub fn build_seo_payload_with_overrides(
    env: Env,
    entity: NodeSEOEntity,
    route: String,
    config: NodeSEOConfig,
    overrides: NodeSEOOverrides,
) -> Result<NodeSEOPayload, napi::Error> {
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
    Ok(NodeSEOPayload::from_core(payload))
}

// ── Contract ──────────────────────────────────────────────────────────

#[napi(object)]
pub struct NodeSEOContractConfig {
    pub canonical_host: String,
    pub scheme: Option<String>,
    pub defaults: Option<NodeSEOExpectation>,
    pub rules: Option<Vec<NodeSEOContractRule>>,
    pub exceptions_json: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct NodeSEOContractRule {
    pub r#match: String,
    pub expect: NodeSEOExpectation,
    pub severity: Option<String>,
}

#[napi]
#[derive(Clone)]
pub struct NodeSEOContract {
    pub contract_version: String,
    pub generator_name: String,
    pub generator_version: String,
    pub site: NodeContractSite,
    pub defaults: NodeSEOExpectation,
    pub rules: Vec<NodeSEOContractRule>,
    pub exceptions: std::collections::BTreeMap<String, NodeSEOExpectation>,
}

#[napi]
impl NodeSEOContract {
    #[napi]
    pub fn hash(&self) -> Result<String, napi::Error> {
        let contract = self.to_core();
        contract
            .hash()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String, napi::Error> {
        let contract = self.to_core();
        contract
            .to_json()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn to_dict(&self) -> Result<serde_json::Value, napi::Error> {
        let contract = self.to_core();
        contract
            .to_dict()
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

impl NodeSEOContract {
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

#[napi(object)]
#[derive(Clone)]
pub struct NodeContractSite {
    pub canonical_host: String,
    pub scheme: String,
}

#[napi]
pub fn build_seo_contract(config: NodeSEOContractConfig) -> Result<NodeSEOContract, napi::Error> {
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
    Ok(NodeSEOContract {
        contract_version: contract.contract_version,
        generator_name: contract.generator.name,
        generator_version: contract.generator.version,
        site: NodeContractSite {
            canonical_host: contract.site.canonical_host,
            scheme: contract.site.scheme,
        },
        defaults: NodeSEOExpectation::from_core(&contract.defaults),
        rules: contract
            .rules
            .iter()
            .map(NodeSEOContractRule::from_core)
            .collect(),
        exceptions: contract
            .exceptions
            .iter()
            .map(|(k, v)| (k.clone(), NodeSEOExpectation::from_core(v)))
            .collect(),
    })
}

// ── Node wrapper for SEOExpectation ────────────────────────────────────

#[napi(object)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSEOExpectation {
    pub required: Option<bool>,
    pub forbidden: Option<bool>,
    pub equals: Option<String>,
    pub not_equals: Option<String>,
    pub contains: Option<String>,
    pub matches: Option<String>,
    pub one_of: Option<Vec<String>>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub min_items: Option<u32>,
    pub max_items: Option<u32>,
    pub indexable: Option<bool>,
    pub canonical: Option<String>,
    pub schema_required: Option<bool>,
    pub schema_types: Option<Vec<String>>,
    pub og_required: Option<bool>,
    pub twitter_required: Option<bool>,
    pub sitemap_required: Option<bool>,
    pub hreflang_required: Option<bool>,
    pub title: Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
}

impl NodeSEOExpectation {
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
                serde_json::from_value::<NodeSEOExpectation>(v.clone())
                    .ok()
                    .map(|n| Box::new(n.to_core()))
            }),
            description: self.description.as_ref().and_then(|v| {
                serde_json::from_value::<NodeSEOExpectation>(v.clone())
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
                let node = NodeSEOExpectation::from_core(t);
                serde_json::to_value(&node).unwrap_or(serde_json::Value::Null)
            }),
            description: e.description.as_ref().map(|d| {
                let node = NodeSEOExpectation::from_core(d);
                serde_json::to_value(&node).unwrap_or(serde_json::Value::Null)
            }),
        }
    }
}

impl NodeSEOContractRule {
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
            expect: NodeSEOExpectation::from_core(&r.expect),
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

#[napi(object)]
pub struct NodeSEOIssue {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
    pub url: Option<String>,
    pub details: std::collections::BTreeMap<String, serde_json::Value>,
}

#[napi]
pub fn validate_payload(payload: &NodeSEOPayload) -> Vec<NodeSEOIssue> {
    let core_payload = payload.to_core();
    core::validate_payload(&core_payload)
        .iter()
        .map(|i| NodeSEOIssue {
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

#[napi]
pub fn normalize_path(
    path: String,
    enforce_https: Option<bool>,
    lowercase_paths: Option<bool>,
    trailing_slash: Option<String>,
    collapse_duplicate_slashes: Option<bool>,
    strip_tracking_params: Option<bool>,
    allowed_query_params: Option<Vec<String>>,
) -> Result<String, napi::Error> {
    let policy = core::URLPolicy {
        enforce_https: enforce_https.unwrap_or(true),
        lowercase_paths: lowercase_paths.unwrap_or(true),
        trailing_slash: match trailing_slash.as_deref() {
            Some("always") => core::TrailingSlash::Always,
            Some("preserve") => core::TrailingSlash::Preserve,
            _ => core::TrailingSlash::Never,
        },
        collapse_duplicate_slashes: collapse_duplicate_slashes.unwrap_or(true),
        strip_tracking_params: strip_tracking_params.unwrap_or(true),
        allowed_query_params: allowed_query_params.unwrap_or_default(),
    };
    core::url::normalize_path(&path, &policy).map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn normalize_public_url(
    url_or_path: String,
    config: NodeSEOConfig,
) -> Result<String, napi::Error> {
    let core_config: core::SEOConfig = (&config).into();
    core::url::normalize_public_url(&url_or_path, &core_config)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

// ── detrack ───────────────────────────────────────────────────────────

#[napi(object)]
pub struct NodeCleanResult {
    pub url: String,
    pub removed_params: std::collections::BTreeMap<String, String>,
    pub cleaned_params: std::collections::BTreeMap<String, String>,
}

#[napi]
pub fn clean_url(url: String) -> NodeCleanResult {
    let result = core::detrack::clean_url(&url);
    NodeCleanResult {
        url: result.url,
        removed_params: result.removed_params,
        cleaned_params: result.cleaned_params,
    }
}

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

#[napi]
pub struct NodeSchemaRegistry;

#[napi]
impl NodeSchemaRegistry {
    // Custom schema generators are registered from JavaScript through the
    // `SchemaRegistry` class exported by `@easeo/core`, which stores
    // callables and applies them around the build. This native type is
    // introspection only.

    #[napi]
    pub fn has(&self, schema_type: String) -> bool {
        let guard = GLOBAL_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .as_ref()
            .is_some_and(|r| r.types.contains(&schema_type))
    }

    #[napi]
    pub fn list_types(&self) -> Vec<String> {
        let guard = GLOBAL_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .as_ref()
            .map_or_else(Vec::new, |r| r.types.iter().cloned().collect())
    }
}

#[napi]
pub fn get_schema_registry() -> NodeSchemaRegistry {
    NodeSchemaRegistry
}
