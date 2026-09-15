use super::{base_schema, SchemaContext};
use crate::error::EaseoError;

pub fn build_faq_page(ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema("FAQPage", ctx)?;

    if let Some(ref items) = ctx.entity.faq_items {
        let main_entity: Vec<serde_json::Value> = items
            .iter()
            .map(|item| {
                serde_json::json!({
                    "@type": "Question",
                    "name": item.question,
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": item.answer,
                    },
                })
            })
            .collect();
        schema["mainEntity"] = serde_json::Value::Array(main_entity);
    }

    Ok(schema)
}
