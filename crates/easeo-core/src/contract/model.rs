use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOExpectation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_graph: Option<FieldExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<FieldExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<FieldExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<FieldExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<SEOExpectation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<SEOExpectation>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaExpectation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldExpectation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContractRule {
    pub r#match: String,
    pub expect: SEOExpectation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ContractSeverity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContractConfig {
    pub canonical_host: String,
    #[serde(default = "default_scheme")]
    pub scheme: String,
    #[serde(default)]
    pub defaults: SEOExpectation,
    #[serde(default)]
    pub rules: Vec<SEOContractRule>,
    #[serde(default)]
    pub exceptions: BTreeMap<String, SEOExpectation>,
}

fn default_scheme() -> String {
    "https".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOContract {
    pub contract_version: String,
    pub generator: ContractGenerator,
    pub site: ContractSite,
    #[serde(default)]
    pub defaults: SEOExpectation,
    #[serde(default)]
    pub rules: Vec<SEOContractRule>,
    #[serde(default)]
    pub exceptions: BTreeMap<String, SEOExpectation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractGenerator {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractSite {
    pub canonical_host: String,
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
