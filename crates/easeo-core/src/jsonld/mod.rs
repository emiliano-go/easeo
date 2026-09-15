pub mod article;
pub mod product;
pub mod organization;
pub mod local_business;
pub mod faq;
pub mod website;
pub mod breadcrumb;
pub mod registry;

use crate::entity::SEOEntity;
use crate::config::SEOConfig;
use crate::error::EaseoError;
use registry::SchemaRegistry;

pub struct SchemaContext<'a> {
    pub entity: &'a SEOEntity,
    pub config: &'a SEOConfig,
    pub canonical: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub og_image: Option<&'a str>,
    pub registry: Option<&'a SchemaRegistry>,
}

pub fn build_schema(ctx: &SchemaContext) -> Result<Option<serde_json::Value>, EaseoError> {
    let schema_type = match ctx.config.schema_type_map.get(ctx.entity.entity_type.as_str()) {
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
        "VideoObject" => Ok(Some(website::build_website(&schema_type, ctx)?)),
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
        schema["image"] = serde_json::Value::String(img.to_string());
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
            publisher["logo"] = serde_json::Value::String(logo.clone());
        }
        schema["publisher"] = publisher;
    }

    Ok(schema)
}
