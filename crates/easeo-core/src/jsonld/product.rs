use crate::error::EaseoError;
use super::{SchemaContext, base_schema};

pub fn build_product(ctx: &SchemaContext) -> Result<serde_json::Value, EaseoError> {
    let mut schema = base_schema("Product", ctx)?;

    if let Some(ref sku) = ctx.entity.sku {
        schema["sku"] = serde_json::Value::String(sku.clone());
    }

    if ctx.entity.price.is_some() || ctx.entity.price_currency.is_some() || ctx.entity.availability.is_some() {
        let mut offer = serde_json::json!({"@type": "Offer"});
        if let Some(ref price) = ctx.entity.price {
            offer["price"] = serde_json::Value::String(price.clone());
        }
        if let Some(ref currency) = ctx.entity.price_currency {
            offer["priceCurrency"] = serde_json::Value::String(currency.clone());
        }
        if let Some(ref avail) = ctx.entity.availability {
            offer["availability"] = serde_json::Value::String(format!("https://schema.org/{}", avail));
        }
        schema["offers"] = offer;
    }

    Ok(schema)
}
