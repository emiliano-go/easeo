use crate::payload::SEOPayload;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SEOIssue {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub details: BTreeMap<String, serde_json::Value>,
}

pub fn validate(payload: &SEOPayload) -> Vec<SEOIssue> {
    let mut issues = Vec::new();

    if payload.title.is_empty() || payload.title == "Untitled" {
        issues.push(SEOIssue {
            rule_id: "EASEO101".to_string(),
            severity: Severity::Warning,
            message: "Title is missing or default".to_string(),
            url: Some(payload.canonical.clone()),
            details: BTreeMap::new(),
        });
    }

    if payload.title.len() > 60 {
        issues.push(SEOIssue {
            rule_id: "EASEO102".to_string(),
            severity: Severity::Warning,
            message: format!(
                "Title is {} characters (recommended max 60)",
                payload.title.len()
            ),
            url: Some(payload.canonical.clone()),
            details: {
                let mut d = BTreeMap::new();
                d.insert("length".to_string(), serde_json::json!(payload.title.len()));
                d
            },
        });
    }

    if payload.description.is_empty() {
        issues.push(SEOIssue {
            rule_id: "EASEO103".to_string(),
            severity: Severity::Warning,
            message: "Meta description is missing".to_string(),
            url: Some(payload.canonical.clone()),
            details: BTreeMap::new(),
        });
    } else if payload.description.len() > 160 {
        issues.push(SEOIssue {
            rule_id: "EASEO104".to_string(),
            severity: Severity::Warning,
            message: format!(
                "Meta description is {} characters (recommended max 160)",
                payload.description.len()
            ),
            url: Some(payload.canonical.clone()),
            details: {
                let mut d = BTreeMap::new();
                d.insert(
                    "length".to_string(),
                    serde_json::json!(payload.description.len()),
                );
                d
            },
        });
    }

    if !payload.canonical.starts_with("http://") && !payload.canonical.starts_with("https://") {
        issues.push(SEOIssue {
            rule_id: "EASEO105".to_string(),
            severity: Severity::Warning,
            message: "Canonical URL is not absolute".to_string(),
            url: Some(payload.canonical.clone()),
            details: BTreeMap::new(),
        });
    }

    // Check OG image is present
    if payload.og.image.is_none() {
        issues.push(SEOIssue {
            rule_id: "EASEO108".to_string(),
            severity: Severity::Warning,
            message: "OG image is missing; set default_og_image or an entity image".to_string(),
            url: Some(payload.canonical.clone()),
            details: BTreeMap::new(),
        });
    }

    // Check OG image is absolute if present
    if let Some(ref img) = payload.og.image {
        if !img.starts_with("http://") && !img.starts_with("https://") {
            issues.push(SEOIssue {
                rule_id: "EASEO106".to_string(),
                severity: Severity::Warning,
                message: "OG image URL is not absolute".to_string(),
                url: Some(payload.canonical.clone()),
                details: {
                    let mut d = BTreeMap::new();
                    d.insert("image".to_string(), serde_json::json!(img));
                    d
                },
            });
        }
    }

    // Check robots format is valid
    let robots = &payload.robots;
    let valid_parts: Vec<&str> = robots.split(',').map(|s| s.trim()).collect();
    let valid_directives = [
        "index",
        "noindex",
        "follow",
        "nofollow",
        "none",
        "noarchive",
        "nosnippet",
        "notranslate",
        "unavailable_after",
    ];
    for part in &valid_parts {
        let key = part.split(':').next().unwrap_or(part);
        if !valid_directives.contains(&key)
            && !key.starts_with("max-snippet")
            && !key.starts_with("max-image-preview")
            && !key.starts_with("max-video-preview")
        {
            issues.push(SEOIssue {
                rule_id: "EASEO107".to_string(),
                severity: Severity::Warning,
                message: format!("Invalid robots directive: '{}'", part),
                url: Some(payload.canonical.clone()),
                details: {
                    let mut d = BTreeMap::new();
                    d.insert("directive".to_string(), serde_json::json!(part));
                    d
                },
            });
        }
    }

    issues
}
