use serde::{Deserialize, Serialize};

use crate::error::EaseoError;

/// The kind of content an entity represents.
///
/// The entity type drives schema selection, default robots directives, and
/// Open Graph type resolution.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EntityType {
    /// The site home page.
    Home,
    /// A blog post or article.
    Post,
    /// A generic page. This is the default.
    #[default]
    Page,
    /// A video page.
    Video,
    /// A taxonomy or archive listing, such as a category or tag.
    Taxonomy,
    /// A search results page.
    Search,
    /// Any other content type.
    Other,
    /// A product page.
    Product,
    /// An organization page.
    Organization,
    /// A local business page.
    LocalBusiness,
    /// A frequently asked questions page.
    Faq,
}

impl EntityType {
    /// Parses an entity type from its snake_case string form.
    ///
    /// # Errors
    ///
    /// Returns [`EaseoError::InvalidEntity`] when the string is not one of the
    /// supported entity types.
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

    /// Returns the snake_case string form of the entity type.
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

/// An author reference used in article schemas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOAuthor {
    /// Display name of the author.
    pub name: String,
    /// Optional profile URL for the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// An image reference used for Open Graph, Twitter Cards, and schemas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOImage {
    /// Absolute URL of the image.
    pub url: String,
    /// Image width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Image height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Alternative text describing the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

/// A single entry in a breadcrumb trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breadcrumb {
    /// Human readable label for the breadcrumb.
    pub name: String,
    /// URL the breadcrumb links to.
    pub url: String,
}

/// A single question and answer pair for FAQ schemas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FAQItem {
    /// The question text.
    pub question: String,
    /// The answer text.
    pub answer: String,
}

/// Robots directives for a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robots {
    /// Whether search engines may index the page. Defaults to `true`.
    #[serde(default = "default_true")]
    pub index: bool,
    /// Whether search engines may follow links on the page. Defaults to `true`.
    #[serde(default = "default_true")]
    pub follow: bool,
    /// Maximum number of characters to show in a snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_snippet: Option<i32>,
    /// Maximum image preview size, for example `"large"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_preview: Option<String>,
    /// Maximum video preview length in seconds.
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
    /// Serializes the directives into the comma separated meta robots value.
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

/// A content entity to generate SEO metadata for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOEntity {
    /// The kind of content this entity represents.
    pub entity_type: EntityType,
    /// URL slug for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Page title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    /// Full content as HTML. Used to derive a description when no excerpt is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    /// Publication status. Anything other than `"publish"` becomes noindex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Primary image for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<SEOImage>,
    /// Publication date as an ISO date or datetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    /// Last update date as an ISO date or datetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Display name of the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// Breadcrumb trail appended as a `BreadcrumbList` schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumbs: Option<Vec<Breadcrumb>>,
    /// Product stock keeping unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Product price as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// ISO currency code for the price, for example `"USD"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    /// Product availability, for example `"InStock"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    /// Additional URLs for the entity, used as `sameAs` in organization schemas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_as: Option<Vec<String>>,
    /// Postal address for local business schemas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Question and answer pairs for FAQ schemas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq_items: Option<Vec<FAQItem>>,
}

/// Per-call overrides that take precedence over the entity and config.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOOverrides {
    /// Overrides the resolved title, before the title template is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    /// Overrides the resolved description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    /// Overrides the resolved canonical URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    /// Overrides the resolved robots directives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Robots>,
    /// Overrides the Open Graph title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_title: Option<String>,
    /// Overrides the Open Graph description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_description: Option<String>,
    /// Overrides the Open Graph image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_image: Option<SEOImage>,
    /// Overrides the Twitter Card type, for example `"summary_large_image"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_card: Option<String>,
    /// Overrides the Twitter title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_title: Option<String>,
    /// Overrides the Twitter description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_description: Option<String>,
    /// Overrides the Twitter image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_image: Option<SEOImage>,
    /// Replaces the generated JSON-LD schema entirely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_jsonld: Option<serde_json::Value>,
    /// When `true`, no JSON-LD schema is emitted.
    #[serde(default)]
    pub omit_schema: bool,
    /// When `true`, the config title template is not applied.
    #[serde(default)]
    pub skip_title_template: bool,
    /// Overrides the Twitter creator handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_creator: Option<String>,
    /// Open Graph audio URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_audio: Option<String>,
    /// Open Graph video URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_video: Option<String>,
}
