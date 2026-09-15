use crate::error::EaseoError;
use super::{SchemaContext, base_schema};

pub fn build_article(schema_type: &str, ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema(schema_type, ctx)?;
    schema["mainEntityOfPage"] = serde_json::json!({
        "@id": ctx.canonical,
    });
    Ok(schema)
}
