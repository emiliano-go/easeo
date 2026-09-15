use super::model::SEOExpectation;

/// Resolve references in expectation values.
/// Supports: $url, $canonical, $site.canonical_host, $entity.slug, $route.path
pub fn resolve_references(
    expectation: &SEOExpectation,
    url: &str,
    canonical: &str,
    site_host: &str,
    entity_slug: Option<&str>,
    route_path: &str,
) -> SEOExpectation {
    let resolve = |s: &str| -> String {
        match s {
            "$url" => url.to_string(),
            "$canonical" => canonical.to_string(),
            "$site.canonical_host" => site_host.to_string(),
            "$entity.slug" => entity_slug.unwrap_or("").to_string(),
            "$route.path" => route_path.to_string(),
            _ => s.to_string(),
        }
    };

    SEOExpectation {
        required: expectation.required,
        forbidden: expectation.forbidden,
        equals: expectation.equals.as_ref().map(|s| resolve(s)),
        not_equals: expectation.not_equals.as_ref().map(|s| resolve(s)),
        contains: expectation.contains.as_ref().map(|s| resolve(s)),
        matches: expectation.matches.as_ref().map(|s| resolve(s)),
        one_of: expectation
            .one_of
            .as_ref()
            .map(|v| v.iter().map(|s| resolve(s)).collect()),
        min_length: expectation.min_length,
        max_length: expectation.max_length,
        min_items: expectation.min_items,
        max_items: expectation.max_items,
        indexable: expectation.indexable,
        canonical: expectation.canonical.as_ref().map(|s| resolve(s)),
        schema: expectation.schema.clone(),
        open_graph: expectation.open_graph.clone(),
        twitter: expectation.twitter.clone(),
        sitemap: expectation.sitemap.clone(),
        hreflang: expectation.hreflang.clone(),
        title: expectation.title.as_ref().map(|t| {
            Box::new(resolve_references(
                t,
                url,
                canonical,
                site_host,
                entity_slug,
                route_path,
            ))
        }),
        description: expectation.description.as_ref().map(|d| {
            Box::new(resolve_references(
                d,
                url,
                canonical,
                site_host,
                entity_slug,
                route_path,
            ))
        }),
    }
}
