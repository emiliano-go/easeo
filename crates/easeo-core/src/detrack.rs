use std::collections::BTreeMap;

pub const DEFAULT_PATTERNS: &[&str] = &[
    // UTM parameters
    "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content",
    "utm_id", "utm_cid", "utm_reader", "utm_name", "utm_social", "utm_social-type",
    // Social tracking
    "fbclid", "gclid", "gclsrc", "dclid",
    "msclkid", "twclid", "igclid", "ttclid", "li_fat_id",
    // Referral
    "ref", "source", "mc_cid", "mc_eid",
    // Analytics
    "_ga", "_gl", "_gac", "_gu", "_kx", "_hsenc", "_hsmi",
    "_openstat", "vero_id", "wickedid", "yclid",
    // Cache busting
    "cb", "rand", "timestamp", "t", "_t",
    // Session
    "sid", "phpsessid", "jsessionid", "asp.net_sessionid",
    // Redirect
    "next", "return", "redirect", "redirect_to", "redirect_url", "goto",
    // Affiliate / marketing
    "aff", "aff_id", "click_id", "tag", "keyword", "campaign",
    // Misc tracking
    "ncid", "zanpid", "zanphp", "msclkid_extra", "twclid_extra",
];

pub struct CleanResult {
    pub url: String,
    pub removed_params: BTreeMap<String, String>,
    pub cleaned_params: BTreeMap<String, String>,
}

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
            if is_tracking_param(key) {
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

pub fn clean_query(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }

    let pairs: Vec<String> = query
        .split('&')
        .filter(|pair| {
            let key = pair.split_once('=').map(|(k, _)| k).unwrap_or(pair);
            !is_tracking_param(key)
        })
        .map(|s| s.to_string())
        .collect();

    if pairs.is_empty() {
        String::new()
    } else {
        pairs.join("&")
    }
}

fn is_tracking_param(key: &str) -> bool {
    let lower = key.to_lowercase();
    DEFAULT_PATTERNS.iter().any(|&p| p == lower)
}

pub fn default_patterns() -> &'static [&'static str] {
    DEFAULT_PATTERNS
}
