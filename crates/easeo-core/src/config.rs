use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::entity::{Robots, SEOImage};
use crate::error::EaseoError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrailingSlash {
    Always,
    Never,
    Preserve,
}

impl TrailingSlash {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, EaseoError> {
        match s {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "preserve" => Ok(Self::Preserve),
            _ => Err(EaseoError::URLPolicyError(format!(
                "trailing_slash must be one of: 'always', 'never', 'preserve', got '{}'",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct URLPolicy {
    #[serde(default = "default_true")]
    pub enforce_https: bool,
    #[serde(default = "default_true")]
    pub lowercase_paths: bool,
    #[serde(default = "default_trailing_slash")]
    pub trailing_slash: TrailingSlash,
    #[serde(default = "default_true")]
    pub collapse_duplicate_slashes: bool,
    #[serde(default = "default_true")]
    pub strip_tracking_params: bool,
    #[serde(default)]
    pub allowed_query_params: Vec<String>,
}

fn default_true() -> bool {
    true
}

fn default_trailing_slash() -> TrailingSlash {
    TrailingSlash::Never
}

impl Default for URLPolicy {
    fn default() -> Self {
        Self {
            enforce_https: true,
            lowercase_paths: true,
            trailing_slash: TrailingSlash::Never,
            collapse_duplicate_slashes: true,
            strip_tracking_params: true,
            allowed_query_params: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEOConfig {
    pub canonical_host: String,
    pub public_base_url: String,
    pub url_policy: URLPolicy,
    #[serde(default = "default_robots")]
    pub default_robots: Robots,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_og_image: Option<SEOImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(default = "default_title_template")]
    pub title_template: Option<String>,
    #[serde(default = "default_search_robots")]
    pub search_robots: Robots,
    #[serde(default = "default_schema_type_map")]
    pub schema_type_map: BTreeMap<String, Option<String>>,
    #[serde(default = "default_true")]
    pub auto_generate_schema: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_alternate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_site: Option<String>,
    #[serde(default)]
    pub emit_warnings: bool,
    /// Optional search URL template for the homepage `WebSite` schema. When
    /// set, a `SearchAction` is emitted with `{search_term_string}` as the
    /// placeholder, for example `https://example.com/search?q={search_term_string}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_url_template: Option<String>,
}

fn default_robots() -> Robots {
    Robots {
        index: true,
        follow: true,
        ..Default::default()
    }
}

fn default_title_template() -> Option<String> {
    Some("{title}".to_string())
}

fn default_search_robots() -> Robots {
    Robots {
        index: false,
        follow: true,
        ..Default::default()
    }
}

fn default_schema_type_map() -> BTreeMap<String, Option<String>> {
    let mut map = BTreeMap::new();
    map.insert("home".to_string(), Some("WebSite".to_string()));
    map.insert("post".to_string(), Some("Article".to_string()));
    map.insert("page".to_string(), Some("WebPage".to_string()));
    map.insert("video".to_string(), Some("VideoObject".to_string()));
    map.insert("taxonomy".to_string(), Some("CollectionPage".to_string()));
    map.insert("search".to_string(), Some("SearchResultsPage".to_string()));
    map.insert("product".to_string(), Some("Product".to_string()));
    map.insert("organization".to_string(), Some("Organization".to_string()));
    map.insert(
        "local_business".to_string(),
        Some("LocalBusiness".to_string()),
    );
    map.insert("faq".to_string(), Some("FAQPage".to_string()));
    map
}

impl SEOConfig {
    pub fn validate(&self) -> Result<(), EaseoError> {
        if self.canonical_host.is_empty() {
            return Err(EaseoError::InvalidConfiguration(
                "canonical_host must be a non-empty string".to_string(),
            ));
        }
        if self.canonical_host.contains("://")
            || self.canonical_host.contains('/')
            || self.canonical_host.contains('?')
            || self.canonical_host.contains('#')
        {
            return Err(EaseoError::InvalidConfiguration(
                "canonical_host must be host-only (no scheme/path/query)".to_string(),
            ));
        }
        if self.public_base_url.is_empty() {
            return Err(EaseoError::InvalidConfiguration(
                "public_base_url must be a non-empty string".to_string(),
            ));
        }
        if let Some(ref tpl) = self.title_template {
            if !tpl.contains("{title}") {
                return Err(EaseoError::InvalidConfiguration(
                    "title_template must include '{title}' placeholder".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl Default for SEOConfig {
    fn default() -> Self {
        Self {
            canonical_host: "localhost".to_string(),
            public_base_url: "http://localhost".to_string(),
            url_policy: URLPolicy::default(),
            default_robots: Robots::default(),
            default_og_image: None,
            site_name: None,
            title_template: Some("{title}".to_string()),
            search_robots: Robots {
                index: false,
                follow: true,
                ..Default::default()
            },
            schema_type_map: default_schema_type_map(),
            auto_generate_schema: true,
            publisher_name: None,
            publisher_logo: None,
            locale: None,
            locale_alternate: None,
            twitter_site: None,
            emit_warnings: false,
            search_url_template: None,
        }
    }
}
