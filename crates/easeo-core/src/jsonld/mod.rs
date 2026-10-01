/// Article and blog posting schemas.
pub mod article;
/// Breadcrumb list schemas.
pub mod breadcrumb;
/// FAQ page schemas.
pub mod faq;
/// LocalBusiness schemas.
pub mod local_business;
/// Organization schemas.
pub mod organization;
/// Product schemas.
pub mod product;
/// Custom schema registries.
pub mod registry;
/// VideoObject schemas.
pub mod video;
/// WebSite, WebPage, and related page schemas.
pub mod website;

use crate::config::SEOConfig;
use crate::entity::SEOEntity;
use crate::error::EaseoError;
use registry::SchemaRegistry;

/// Inputs available to JSON-LD schema builders.
pub struct SchemaContext<'a> {
    /// Entity the schema describes.
    pub entity: &'a SEOEntity,
    /// Site-wide configuration.
    pub config: &'a SEOConfig,
    /// Resolved canonical URL.
    pub canonical: &'a str,
    /// Resolved title.
    pub title: &'a str,
    /// Resolved description.
    pub description: Option<&'a str>,
    /// Resolved Open Graph image URL.
    pub og_image: Option<&'a str>,
    /// Optional custom schema registry.
    pub registry: Option<&'a SchemaRegistry>,
}

/// Builds the JSON-LD schema for the entity in the context.
///
/// Returns `None` when the entity type maps to no schema. Falls back to a
/// custom registry builder, then to a generic page schema, for unknown types.
///
/// # Errors
///
/// Returns [`EaseoError`] when schema construction fails.
pub fn build_schema(ctx: &SchemaContext) -> Result<Option<serde_json::Value>, EaseoError> {
    let schema_type = match ctx
        .config
        .schema_type_map
        .get(ctx.entity.entity_type.as_str())
    {
        Some(Some(t)) => t.clone(),
        Some(None) => return Ok(None),
        None => return Ok(None),
    };

    match schema_type.as_str() {
        "Article" | "BlogPosting" | "NewsArticle" => {
            Ok(Some(article::build_article(&schema_type, ctx)?))
        }
        "Product" => Ok(Some(product::build_product(ctx)?)),
        "Organization" => Ok(Some(organization::build_organization(ctx)?)),
        "LocalBusiness" => Ok(Some(local_business::build_local_business(ctx)?)),
        "FAQPage" => Ok(Some(faq::build_faq_page(ctx)?)),
        "WebPage" | "WebSite" | "CollectionPage" | "SearchResultsPage" => {
            Ok(Some(website::build_website(&schema_type, ctx)?))
        }
        "VideoObject" => Ok(Some(video::build_video(ctx)?)),
        _ => {
            if let Some(registry) = ctx.registry {
                if let Some(builder) = registry.get(&schema_type) {
                    return Ok(Some(builder(ctx)));
                }
            }
            Ok(Some(website::build_website(&schema_type, ctx)?))
        }
    }
}

pub(crate) fn base_schema(
    schema_type: &str,
    ctx: &SchemaContext,
) -> Result<serde_json::Value, EaseoError> {
    let mut schema = serde_json::json!({
        "@context": "https://schema.org",
        "@type": schema_type,
        "name": ctx.title,
        "url": ctx.canonical,
    });

    if let Some(desc) = ctx.description {
        schema["description"] = serde_json::Value::String(desc.to_string());
    }
    if let Some(img) = ctx.og_image {
        let img_url = if img.starts_with("http://") || img.starts_with("https://") {
            img.to_string()
        } else {
            format!(
                "{}{}",
                ctx.config.public_base_url.trim_end_matches('/'),
                img
            )
        };
        schema["image"] = serde_json::Value::String(img_url);
    }
    if let Some(ref pub_date) = ctx.entity.published_at {
        schema["datePublished"] = serde_json::Value::String(pub_date.clone());
    }
    if let Some(ref upd_date) = ctx.entity.updated_at {
        schema["dateModified"] = serde_json::Value::String(upd_date.clone());
    }
    if let Some(ref author) = ctx.entity.author_name {
        schema["author"] = serde_json::json!({
            "@type": "Person",
            "name": author,
        });
    }
    if let Some(ref name) = ctx.config.publisher_name {
        let mut publisher = serde_json::json!({
            "@type": "Organization",
            "name": name,
        });
        if let Some(ref logo) = ctx.config.publisher_logo {
            let logo_url = if logo.starts_with("http://") || logo.starts_with("https://") {
                logo.clone()
            } else {
                format!(
                    "{}{}",
                    ctx.config.public_base_url.trim_end_matches('/'),
                    logo
                )
            };
            publisher["logo"] = serde_json::Value::String(logo_url);
        }
        schema["publisher"] = publisher;
    }

    Ok(schema)
}
