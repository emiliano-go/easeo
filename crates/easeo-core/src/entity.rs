use serde::{Deserialize, Serialize};

use crate::error::EaseoError;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EntityType {
    Home,
    Post,
    #[default]
    Page,
    Video,
    Taxonomy,
    Search,
    Other,
    Product,
    Organization,
    LocalBusiness,
    Faq,
}

impl EntityType {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, EaseoError> {
        match s {
            "home" => Ok(Self::Home),
            "post" => Ok(Self::Post),
            "page" => Ok(Self::Page),
            "video" => Ok(Self::Video),
            "taxonomy" => Ok(Self::Taxonomy),
            "search" => Ok(Self::Search),
            "other" => Ok(Self::Other),
            "product" => Ok(Self::Product),
            "organization" => Ok(Self::Organization),
            "local_business" => Ok(Self::LocalBusiness),
            "faq" => Ok(Self::Faq),
            _ => Err(EaseoError::InvalidEntity(format!(
                "entity_type must be one of home/post/page/video/taxonomy/search/other/product/organization/local_business/faq, got '{}'",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Post => "post",
            Self::Page => "page",
            Self::Video => "video",
            Self::Taxonomy => "taxonomy",
            Self::Search => "search",
            Self::Other => "other",
            Self::Product => "product",
            Self::Organization => "organization",
            Self::LocalBusiness => "local_business",
            Self::Faq => "faq",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOImage {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FAQItem {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robots {
    #[serde(default = "default_true")]
    pub index: bool,
    #[serde(default = "default_true")]
    pub follow: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_snippet: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_preview: Option<i32>,
}

fn default_true() -> bool {
    true
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            index: true,
            follow: true,
            max_snippet: None,
            max_image_preview: None,
            max_video_preview: None,
        }
    }
}

impl Robots {
    pub fn serialize(&self) -> String {
        let mut parts = vec![
            if self.index { "index" } else { "noindex" }.to_string(),
            if self.follow { "follow" } else { "nofollow" }.to_string(),
        ];
        if let Some(v) = self.max_snippet {
            parts.push(format!("max-snippet:{}", v));
        }
        if let Some(ref v) = self.max_image_preview {
            parts.push(format!("max-image-preview:{}", v));
        }
        if let Some(v) = self.max_video_preview {
            parts.push(format!("max-video-preview:{}", v));
        }
        parts.join(",")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOEntity {
    pub entity_type: EntityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<SEOImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumbs: Option<Vec<Breadcrumb>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_as: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq_items: Option<Vec<FAQItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Robots>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_image: Option<SEOImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_image: Option<SEOImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_jsonld: Option<serde_json::Value>,
    #[serde(default)]
    pub omit_schema: bool,
    #[serde(default)]
    pub skip_title_template: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_video: Option<String>,
}
