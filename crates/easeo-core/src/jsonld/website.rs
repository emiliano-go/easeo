use super::{base_schema, SchemaContext};
use crate::error::EaseoError;

pub fn build_website(
    schema_type: &str,
    ctx: &SchemaContext,
) -> Result<serde_json::Value, EaseoError> {
    base_schema(schema_type, ctx)
}
