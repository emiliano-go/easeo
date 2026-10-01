use std::collections::BTreeMap;

/// Tracking query parameters removed by default.
///
/// The list is deliberately conservative: only vendor-prefixed click ids,
/// UTM parameters, and unambiguous session identifiers are removed. Generic
/// names that applications may use for real content (`ref`, `source`, `tag`,
/// `keyword`, `campaign`, `redirect`, `next`, `timestamp`, and similar) are
/// *not* stripped by default. Add them per site with
/// [`crate::config::URLPolicy::extra_tracking_params`].
pub const DEFAULT_PATTERNS: &[&str] = &[
    // UTM parameters
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "utm_id",
    "utm_cid",
    "utm_reader",
    "utm_name",
    "utm_social",
    "utm_social-type",
    // Social tracking
    "fbclid",
    "gclid",
    "gclsrc",
    "dclid",
    "msclkid",
    "twclid",
    "igclid",
    "ttclid",
    "li_fat_id",
    // Mailchimp
    "mc_cid",
    "mc_eid",
    // Analytics
    "_ga",
    "_gl",
    "_gac",
    "_gu",
    "_kx",
    "_hsenc",
    "_hsmi",
    "_openstat",
    "vero_id",
    "wickedid",
    "yclid",
    // Session
    "phpsessid",
    "jsessionid",
    "asp.net_sessionid",
    // Misc tracking
    "ncid",
    "zanpid",
    "zanphp",
];

/// Result of cleaning a URL's tracking parameters.
pub struct CleanResult {
    /// URL with tracking parameters removed.
    pub url: String,
    /// Parameters that were removed, keyed by parameter name.
    pub removed_params: BTreeMap<String, String>,
    /// Parameters that were kept, keyed by parameter name.
    pub cleaned_params: BTreeMap<String, String>,
}

/// Removes tracking parameters from a URL and reports what changed.
pub fn clean_url(url: &str) -> CleanResult {
    let (base, query) = match url.split_once('?') {
        Some((b, q)) => (b.to_string(), q.to_string()),
        None => (url.to_string(), String::new()),
    };

    let cleaned_query = clean_query(&query);
    let mut removed = BTreeMap::new();
    let mut cleaned = BTreeMap::new();

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if is_tracking_param(&decoded_key(key)) {
                removed.insert(key.to_string(), value.to_string());
            } else {
                cleaned.insert(key.to_string(), value.to_string());
            }
        }
    }

    let result_url = if cleaned_query.is_empty() {
        base
    } else {
        format!("{}?{}", base, cleaned_query)
    };

    CleanResult {
        url: result_url,
        removed_params: removed,
        cleaned_params: cleaned,
    }
}

/// Removes tracking parameters from a query string.
pub fn clean_query(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }

    let pairs: Vec<String> = query
        .split('&')
        .filter(|pair| {
            let key = pair.split_once('=').map(|(k, _)| k).unwrap_or(pair);
            !is_tracking_param(&decoded_key(key))
        })
        .map(|s| s.to_string())
        .collect();

    if pairs.is_empty() {
        String::new()
    } else {
        pairs.join("&")
    }
}

/// Decodes a query parameter key for matching. The original bytes are kept in
/// output; decoding only feeds tracking checks.
pub(crate) fn decoded_key(raw_key: &str) -> String {
    url::form_urlencoded::parse(raw_key.as_bytes())
        .next()
        .map(|(key, _)| key.into_owned())
        .unwrap_or_else(|| raw_key.to_string())
}

fn is_default_tracking_param(key: &str) -> bool {
    DEFAULT_PATTERNS.iter().any(|p| p.eq_ignore_ascii_case(key))
}

pub(crate) fn is_tracking_param(key: &str) -> bool {
    is_default_tracking_param(key)
}

/// Returns `true` when `key` matches the built-in tracking list or one of the
/// caller supplied extra parameter names.
pub(crate) fn is_tracking_param_with(key: &str, extra: &[String]) -> bool {
    is_default_tracking_param(key) || extra.iter().any(|p| p.eq_ignore_ascii_case(key))
}

/// Returns the default tracking parameter patterns.
pub fn default_patterns() -> &'static [&'static str] {
    DEFAULT_PATTERNS
}
