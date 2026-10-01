use crate::config::SEOConfig;
use crate::entity::{EntityType, SEOEntity, SEOImage, SEOOverrides};
use crate::error::EaseoError;
use crate::opengraph::resolve_og_type;
use crate::text::build_description_snippet;
use crate::twitter::default_twitter_card;
use crate::url::normalize_public_url;
use serde::{Deserialize, Serialize};

/// Open Graph metadata for a payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OGPayload {
    /// Open Graph object type, for example `"article"` or `"website"`.
    #[serde(rename = "type")]
    pub og_type: String,
    /// Open Graph title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Open Graph description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Open Graph canonical URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Absolute URL of the Open Graph image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Open Graph image width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<u32>,
    /// Open Graph image height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<u32>,
    /// Open Graph image alternative text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_alt: Option<String>,
    /// Site name, rendered as `og:site_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// Locale, for example `"en_US"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Alternate locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_alternate: Option<Vec<String>>,
    /// Open Graph audio URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// Open Graph video URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
}

/// Twitter Card metadata for a payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterPayload {
    /// Card type, for example `"summary_large_image"`.
    pub card: String,
    /// Twitter title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Twitter description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Absolute URL of the Twitter image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Twitter image alternative text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_alt: Option<String>,
    /// Twitter `@handle` for the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    /// Twitter `@handle` of the content creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

/// The resolved, deterministic SEO payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOPayload {
    /// Resolved title, with the config title template applied.
    pub title: String,
    /// Resolved description.
    pub description: String,
    /// Normalized canonical URL.
    pub canonical: String,
    /// Serialized robots directives, for example `"index,follow"`.
    pub robots: String,
    /// Open Graph metadata.
    pub og: OGPayload,
    /// Twitter Card metadata.
    pub twitter: TwitterPayload,
    /// Generated JSON-LD, when schema generation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_jsonld: Option<serde_json::Value>,
    /// Additional fields added by config-scoped hooks. A `BTreeMap` keeps
    /// serialization deterministic; the field is flattened so extras appear
    /// as top-level keys in the wire format.
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl SEOPayload {
    /// Converts the payload into a JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn to_dict(&self) -> Result<serde_json::Value, crate::error::EaseoError> {
        serde_json::to_value(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Rebuild a payload from its wire format. Unknown top-level keys are
    /// preserved in `extra`. Used by config-scoped hooks, which receive the
    /// payload as a dict and may add arbitrary keys.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when the value does not
    /// match the payload shape.
    pub fn from_dict(value: &serde_json::Value) -> Result<Self, crate::error::EaseoError> {
        serde_json::from_value(value.clone())
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Serializes the payload to a compact JSON string.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn to_json(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Serializes the payload to a pretty printed JSON string.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn to_json_pretty(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Renders only the Open Graph meta tags.
    pub fn render_opengraph(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "<meta property=\"og:type\" content=\"{}\">",
            escape_html(&self.og.og_type)
        ));
        if let Some(ref t) = self.og.title {
            lines.push(format!(
                "<meta property=\"og:title\" content=\"{}\">",
                escape_html(t)
            ));
        }
        if let Some(ref d) = self.og.description {
            lines.push(format!(
                "<meta property=\"og:description\" content=\"{}\">",
                escape_html(d)
            ));
        }
        if let Some(ref u) = self.og.url {
            lines.push(format!(
                "<meta property=\"og:url\" content=\"{}\">",
                escape_html(u)
            ));
        }
        if let Some(ref i) = self.og.image {
            lines.push(format!(
                "<meta property=\"og:image\" content=\"{}\">",
                escape_html(i)
            ));
        }
        if let Some(w) = self.og.image_width {
            lines.push(format!(
                "<meta property=\"og:image:width\" content=\"{}\">",
                w
            ));
        }
        if let Some(h) = self.og.image_height {
            lines.push(format!(
                "<meta property=\"og:image:height\" content=\"{}\">",
                h
            ));
        }
        if let Some(ref a) = self.og.image_alt {
            lines.push(format!(
                "<meta property=\"og:image:alt\" content=\"{}\">",
                escape_html(a)
            ));
        }
        if let Some(ref s) = self.og.site_name {
            lines.push(format!(
                "<meta property=\"og:site_name\" content=\"{}\">",
                escape_html(s)
            ));
        }
        if let Some(ref l) = self.og.locale {
            lines.push(format!(
                "<meta property=\"og:locale\" content=\"{}\">",
                escape_html(l)
            ));
        }
        if let Some(ref locs) = self.og.locale_alternate {
            for loc in locs {
                lines.push(format!(
                    "<meta property=\"og:locale:alternate\" content=\"{}\">",
                    escape_html(loc)
                ));
            }
        }
        if let Some(ref a) = self.og.audio {
            lines.push(format!(
                "<meta property=\"og:audio\" content=\"{}\">",
                escape_html(a)
            ));
        }
        if let Some(ref v) = self.og.video {
            lines.push(format!(
                "<meta property=\"og:video\" content=\"{}\">",
                escape_html(v)
            ));
        }
        lines.join("\n")
    }

    /// Renders only the Twitter Card meta tags.
    pub fn render_twitter(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "<meta name=\"twitter:card\" content=\"{}\">",
            escape_html(&self.twitter.card)
        ));
        if let Some(ref t) = self.twitter.title {
            lines.push(format!(
                "<meta name=\"twitter:title\" content=\"{}\">",
                escape_html(t)
            ));
        }
        if let Some(ref d) = self.twitter.description {
            lines.push(format!(
                "<meta name=\"twitter:description\" content=\"{}\">",
                escape_html(d)
            ));
        }
        if let Some(ref i) = self.twitter.image {
            lines.push(format!(
                "<meta name=\"twitter:image\" content=\"{}\">",
                escape_html(i)
            ));
        }
        if let Some(ref a) = self.twitter.image_alt {
            lines.push(format!(
                "<meta name=\"twitter:image:alt\" content=\"{}\">",
                escape_html(a)
            ));
        }
        if let Some(ref s) = self.twitter.site {
            lines.push(format!(
                "<meta name=\"twitter:site\" content=\"{}\">",
                escape_html(s)
            ));
        }
        if let Some(ref c) = self.twitter.creator {
            lines.push(format!(
                "<meta name=\"twitter:creator\" content=\"{}\">",
                escape_html(c)
            ));
        }
        lines.join("\n")
    }

    /// Renders the JSON-LD script tag, escaping `<` so the JSON stays
    /// script-safe. Returns an empty string when no schema is present.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn render_jsonld(&self) -> Result<String, crate::error::EaseoError> {
        match self.schema_jsonld {
            Some(ref schema) => {
                let json = serde_json::to_string_pretty(schema)
                    .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))?;
                let safe_json = json.replace("</", r#"<\/"#);
                Ok(format!(
                    "<script type=\"application/ld+json\">\n{}\n</script>",
                    safe_json
                ))
            }
            None => Ok(String::new()),
        }
    }

    /// Renders the full `<head>` snippet: title, description, canonical,
    /// robots, Open Graph, Twitter Cards, and JSON-LD.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when JSON-LD
    /// serialization fails.
    pub fn render_html(&self) -> Result<String, crate::error::EaseoError> {
        let mut lines = Vec::new();
        lines.push(format!("<title>{}</title>", escape_html(&self.title)));
        if !self.description.is_empty() {
            lines.push(format!(
                "<meta name=\"description\" content=\"{}\">",
                escape_html(&self.description)
            ));
        }
        lines.push(format!(
            "<link rel=\"canonical\" href=\"{}\">",
            escape_html(&self.canonical)
        ));
        lines.push(format!(
            "<meta name=\"robots\" content=\"{}\">",
            escape_html(&self.robots)
        ));
        let og = self.render_opengraph();
        if !og.is_empty() {
            lines.push(og);
        }
        let tw = self.render_twitter();
        if !tw.is_empty() {
            lines.push(tw);
        }
        let jl = self.render_jsonld()?;
        if !jl.is_empty() {
            lines.push(jl);
        }
        Ok(lines.join("\n") + "\n")
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn pick_string(values: &[Option<&str>]) -> Option<String> {
    for s in values.iter().flatten() {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn resolve_image_parts(
    image: Option<&SEOImage>,
) -> (Option<String>, Option<u32>, Option<u32>, Option<String>) {
    match image {
        None => (None, None, None, None),
        Some(img) => (
            Some(img.url.clone()),
            img.width,
            img.height,
            img.alt.clone(),
        ),
    }
}

fn entity_default_robots(entity: &SEOEntity, config: &SEOConfig) -> crate::entity::Robots {
    if entity.entity_type == EntityType::Search {
        return config.search_robots.clone();
    }
    match entity.status.as_deref() {
        Some(s) if s.eq_ignore_ascii_case("published") => crate::entity::Robots {
            index: true,
            follow: true,
            ..Default::default()
        },
        // Any explicit non-published status (draft, archived, private, ...)
        // must not be indexed.
        Some(_) => crate::entity::Robots {
            index: false,
            follow: true,
            ..Default::default()
        },
        None => config.default_robots.clone(),
    }
}

fn validate_canonical_override(url: &str) -> Result<String, EaseoError> {
    let value = url.trim();
    let parsed = url::Url::parse(value).map_err(|e| {
        EaseoError::InvalidUrl(format!(
            "canonical_url override must be an absolute URL: {}",
            e
        ))
    })?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(EaseoError::InvalidUrl(
            "canonical_url override must use http or https".to_string(),
        ));
    }
    Ok(value.to_string())
}

/// Builds a payload from an entity, route, config, optional overrides, and an
/// optional custom schema registry.
///
/// Most callers use the crate level [`crate::build_seo_payload`] or
/// [`crate::build_seo_payload_with_overrides`] instead.
///
/// # Errors
///
/// Returns [`EaseoError`] when the entity, route, configuration, or schema is
/// invalid.
pub fn build_seo_payload(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: Option<&SEOOverrides>,
    registry: Option<&crate::jsonld::registry::SchemaRegistry>,
) -> Result<SEOPayload, EaseoError> {
    let default_ov = SEOOverrides::default();
    let ov = overrides.unwrap_or(&default_ov);

    // Title
    let base_title = pick_string(&[
        ov.meta_title.as_deref(),
        entity.title.as_deref(),
        Some("Untitled"),
    ])
    .unwrap_or_else(|| "Untitled".to_string());

    let title = if let Some(ref tpl) = config.title_template {
        if ov.skip_title_template {
            base_title
        } else {
            tpl.replace("{title}", &base_title)
        }
    } else {
        base_title
    };

    // Description
    let body_snippet = entity
        .body_html
        .as_deref()
        .and_then(|h| build_description_snippet(Some(h), 160));
    let description = pick_string(&[
        ov.meta_description.as_deref(),
        entity.excerpt.as_deref(),
        body_snippet.as_deref(),
    ])
    .unwrap_or_default();

    // Canonical. canonical_url is trusted and only checked for absoluteness;
    // canonical_path goes through the full normalization pipeline.
    let canonical = if let Some(ref url) = ov.canonical_url {
        validate_canonical_override(url)?
    } else if let Some(ref path) = ov.canonical_path {
        normalize_public_url(path, config)?
    } else {
        normalize_public_url(route, config)?
    };

    // Robots
    let robots = if let Some(ref r) = ov.robots {
        r.serialize()
    } else {
        entity_default_robots(entity, config).serialize()
    };

    // OG image
    let og_image = ov
        .og_image
        .as_ref()
        .or(entity.featured_image.as_ref())
        .or(config.default_og_image.as_ref());
    let (og_img_url, og_img_w, og_img_h, og_img_alt) = resolve_image_parts(og_image);

    // Twitter image
    let twitter_image = ov.twitter_image.as_ref().or(og_image);
    let (tw_img_url, _tw_img_w, _tw_img_h, tw_img_alt) = resolve_image_parts(twitter_image);

    // OG title/description
    let og_title = pick_string(&[ov.og_title.as_deref(), Some(&title)]);
    let og_description = pick_string(&[ov.og_description.as_deref(), Some(&description)]);

    // Twitter title/description/card
    let twitter_title = pick_string(&[ov.twitter_title.as_deref(), og_title.as_deref()]);
    let twitter_description =
        pick_string(&[ov.twitter_description.as_deref(), og_description.as_deref()]);
    let twitter_card = pick_string(&[ov.twitter_card.as_deref(), Some(default_twitter_card())])
        .unwrap_or_else(|| default_twitter_card().to_string());

    let og = OGPayload {
        og_type: resolve_og_type(&entity.entity_type).to_string(),
        title: og_title,
        description: og_description,
        url: Some(canonical.clone()),
        image: og_img_url.clone(),
        image_width: og_img_w,
        image_height: og_img_h,
        image_alt: og_img_alt.clone(),
        site_name: config.site_name.clone(),
        locale: config.locale.clone(),
        locale_alternate: config.locale_alternate.clone(),
        audio: ov.og_audio.clone(),
        video: ov.og_video.clone(),
    };

    let twitter = TwitterPayload {
        card: twitter_card,
        title: twitter_title,
        description: twitter_description,
        image: tw_img_url,
        image_alt: tw_img_alt,
        site: config.twitter_site.clone(),
        creator: ov.twitter_creator.clone(),
    };

    // Schema
    let schema_jsonld = if ov.omit_schema {
        None
    } else if let Some(ref custom) = ov.schema_jsonld {
        Some(custom.clone())
    } else if config.auto_generate_schema {
        let ctx = crate::jsonld::SchemaContext {
            entity,
            config,
            canonical: &canonical,
            title: &title,
            description: Some(&description),
            og_image: og_img_url.as_deref(),
            registry,
        };
        crate::jsonld::build_schema(&ctx)?
    } else {
        None
    };

    // Breadcrumbs
    let mut schemas: Vec<serde_json::Value> = Vec::new();
    if let Some(ref s) = schema_jsonld {
        match s {
            serde_json::Value::Array(arr) => schemas.extend(arr.iter().cloned()),
            other => schemas.push(other.clone()),
        }
    }
    if let Some(ref breadcrumbs) = entity.breadcrumbs {
        schemas.push(crate::breadcrumbs::build_breadcrumb_list(
            breadcrumbs,
            config,
        )?);
    }

    let final_schema = if schemas.is_empty() {
        None
    } else if schemas.len() == 1 {
        schemas.into_iter().next()
    } else {
        Some(serde_json::Value::Array(schemas))
    };

    Ok(SEOPayload {
        title,
        description,
        canonical,
        robots,
        og,
        twitter,
        schema_jsonld: final_schema,
        extra: std::collections::BTreeMap::new(),
    })
}
