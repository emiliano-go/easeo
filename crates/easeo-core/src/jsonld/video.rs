use super::{base_schema, SchemaContext};
use crate::error::EaseoError;

/// Builds a `VideoObject` schema.
///
/// Adds the two properties Google requires for video rich results on top of
/// the shared page fields: `thumbnailUrl` (from the resolved image) and
/// `uploadDate` (from `published_at`). Video duration, content URL, and
/// embed URL are not modeled; use a [`SchemaRegistry`](crate::jsonld::registry::SchemaRegistry)
/// generator when those are needed.
///
/// # Errors
///
/// Returns [`EaseoError`] when the schema cannot be constructed.
pub fn build_video(ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema("VideoObject", ctx)?;

    if let Some(image) = schema.get("image").cloned() {
        schema["thumbnailUrl"] = image;
    }
    if let Some(ref published) = ctx.entity.published_at {
        schema["uploadDate"] = serde_json::Value::String(published.clone());
    }

    Ok(schema)
}
