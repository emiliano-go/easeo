use crate::error::EaseoError;
use super::{SchemaContext, organization::build_organization};

pub fn build_local_business(ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = build_organization(ctx)?;
    schema["@type"] = serde_json::Value::String("LocalBusiness".to_string());

    if let Some(ref address) = ctx.entity.address {
        schema["address"] = serde_json::json!({
            "@type": "PostalAddress",
            "streetAddress": address,
        });
    }

    Ok(schema)
}
