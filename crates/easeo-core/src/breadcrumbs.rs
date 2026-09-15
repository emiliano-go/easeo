use crate::config::SEOConfig;
use crate::entity::Breadcrumb;
use crate::url::normalize_public_url;

pub fn build_breadcrumb_list(
    breadcrumbs: &[Breadcrumb],
    config: &SEOConfig,
) -> Result<serde_json::Value, crate::error::EaseoError> {
    let items: Vec<serde_json::Value> = breadcrumbs
        .iter()
        .enumerate()
        .map(|(i, bc)| {
            let url = normalize_public_url(&bc.url, config)?;
            Ok(serde_json::json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": bc.name,
                "item": url,
            }))
        })
        .collect::<Result<Vec<_>, crate::error::EaseoError>>()?;

    Ok(serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": items,
    }))
}
