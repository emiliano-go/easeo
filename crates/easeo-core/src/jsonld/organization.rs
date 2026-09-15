use crate::error::EaseoError;
use super::{SchemaContext, base_schema};

pub fn build_organization(ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema("Organization", ctx)?;

    if let Some(ref same_as) = ctx.entity.same_as {
        schema["sameAs"] = serde_json::json!(same_as);
    }
    if let Some(ref logo) = ctx.config.publisher_logo {
        schema["logo"] = serde_json::Value::String(logo.clone());
    }

    Ok(schema)
}
