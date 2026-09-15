use crate::error::EaseoError;
use super::{SchemaContext, base_schema};

pub fn build_website(schema_type: &str, ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    base_schema(schema_type, ctx)
}
