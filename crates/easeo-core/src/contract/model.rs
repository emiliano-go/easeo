use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Severity of a contract assertion failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractSeverity {
    /// The assertion must hold.
    Error,
    /// The assertion should hold.
    Warning,
    /// Informational assertion.
    Info,
}

/// Expectations applied to a page, a rule match, or contract defaults.
///
/// Every field is optional. Unset fields are not checked.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOExpectation {
    /// The field must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The field must be absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden: Option<bool>,
    /// The field must equal this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    /// The field must not equal this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<String>,
    /// The field must contain this substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    /// The field must match this regular expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<String>,
    /// The field must be one of these values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<String>>,
    /// Minimum string length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    /// Maximum string length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    /// Minimum number of items in a collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    /// Maximum number of items in a collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    /// Whether the page must be indexable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable: Option<bool>,
    /// Expected canonical behavior, for example `"self"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<String>,
    /// Expectations for the JSON-LD schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaExpectation>,
    /// Expectations for Open Graph metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_graph: Option<FieldExpectation>,
    /// Expectations for Twitter Card metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<FieldExpectation>,
    /// Expectations for the sitemap entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<FieldExpectation>,
    /// Expectations for hreflang annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<FieldExpectation>,
    /// Nested expectations for the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<SEOExpectation>>,
    /// Nested expectations for the description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<SEOExpectation>>,
}

/// Expectations for the JSON-LD schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaExpectation {
    /// Whether a schema must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Schema.org types the page must declare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// Expectations for a metadata block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldExpectation {
    /// Whether the block must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// A contract rule matched against route paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContractRule {
    /// Route pattern, for example `"/blog/*"`.
    pub r#match: String,
    /// Expectations applied when the pattern matches.
    pub expect: SEOExpectation,
    /// Optional severity for failures of this rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ContractSeverity>,
}

/// Input configuration for building a contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContractConfig {
    /// Canonical hostname without a scheme.
    pub canonical_host: String,
    /// URL scheme, defaults to `"https"`.
    #[serde(default = "default_scheme")]
    pub scheme: String,
    /// Default expectations applied to every route.
    #[serde(default)]
    pub defaults: SEOExpectation,
    /// Route specific rules.
    #[serde(default)]
    pub rules: Vec<SEOContractRule>,
    /// Per-route expectation overrides, keyed by route.
    #[serde(default)]
    pub exceptions: BTreeMap<String, SEOExpectation>,
}

fn default_scheme() -> String {
    "https".to_string()
}

/// A generated, machine-readable SEO contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContract {
    /// Version of the contract format itself.
    pub contract_version: String,
    /// Tool that generated the contract.
    pub generator: ContractGenerator,
    /// Site the contract applies to.
    pub site: ContractSite,
    /// Default expectations applied to every route.
    #[serde(default)]
    pub defaults: SEOExpectation,
    /// Route specific rules.
    #[serde(default)]
    pub rules: Vec<SEOContractRule>,
    /// Per-route expectation overrides.
    #[serde(default)]
    pub exceptions: BTreeMap<String, SEOExpectation>,
}

/// Identifies the tool that generated a contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractGenerator {
    /// Generator name.
    pub name: String,
    /// Generator version.
    pub version: String,
}

/// Site identity stored in a contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractSite {
    /// Canonical hostname.
    pub canonical_host: String,
    /// URL scheme.
    pub scheme: String,
}

impl Default for SEOContractConfig {
    fn default() -> Self {
        Self {
            canonical_host: "localhost".to_string(),
            scheme: "https".to_string(),
            defaults: SEOExpectation::default(),
            rules: Vec::new(),
            exceptions: std::collections::BTreeMap::new(),
        }
    }
}
