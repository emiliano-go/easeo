use crate::config::SEOConfig;
use crate::entity::{EntityType, SEOEntity, SEOImage, SEOOverrides};
use crate::error::EaseoError;
use crate::opengraph::resolve_og_type;
use crate::text::build_description_snippet;
use crate::twitter::default_twitter_card;
use crate::url::normalize_public_url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OGPayload {
    #[serde(rename = "type")]
    pub og_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_alternate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterPayload {
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOPayload {
    pub title: String,
    pub description: String,
    pub canonical: String,
    pub robots: String,
    pub og: OGPayload,
    pub twitter: TwitterPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_jsonld: Option<serde_json::Value>,
}

impl SEOPayload {
    pub fn to_dict(&self) -> Result<serde_json::Value, crate::error::EaseoError> {
        serde_json::to_value(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    pub fn to_json(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    pub fn to_json_pretty(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

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
    if entity
        .status
        .as_deref()
        .is_some_and(|s| s.eq_ignore_ascii_case("published"))
    {
        return crate::entity::Robots {
            index: true,
            follow: true,
            ..Default::default()
        };
    }
    config.default_robots.clone()
}

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

    // Canonical
    let canonical = if let Some(ref url) = ov.canonical_url {
        url.clone()
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
    })
}
