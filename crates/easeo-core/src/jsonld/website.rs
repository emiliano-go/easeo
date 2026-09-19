use super::{base_schema, SchemaContext};
use crate::error::EaseoError;

const SEARCH_TERM_PLACEHOLDER: &str = "{search_term_string}";

pub fn build_website(
    schema_type: &str,
    ctx: &SchemaContext,
) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema(schema_type, ctx)?;

    // A WebSite schema can advertise a site search action. It is only
    // emitted when the config provides a URL template, because a
    // SearchAction pointing at a non-existent endpoint is worse than none.
    if schema_type == "WebSite" {
        if let Some(ref template) = ctx.config.search_url_template {
            if template.contains(SEARCH_TERM_PLACEHOLDER) {
                schema["potentialAction"] = serde_json::json!({
                    "@type": "SearchAction",
                    "target": {
                        "@type": "EntryPoint",
                        "urlTemplate": template,
                    },
                    "query-input": "required name=search_term_string",
                });
            }
        }
    }

    Ok(schema)
}
