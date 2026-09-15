use crate::config::{SEOConfig, TrailingSlash, URLPolicy};
use crate::error::EaseoError;

pub fn normalize_path(path: &str, policy: &URLPolicy) -> Result<String, EaseoError> {
    let mut value = path.trim().to_string();
    if value.is_empty() {
        value = "/".to_string();
    }
    if !value.starts_with('/') {
        value = format!("/{}", value);
    }
    if policy.collapse_duplicate_slashes {
        value = collapse_duplicate_slashes(&value);
    }
    if policy.lowercase_paths {
        value = value.to_lowercase();
    }
    value = apply_trailing_slash(&value, &policy.trailing_slash);
    Ok(value)
}

pub fn normalize_public_url(url_or_path: &str, config: &SEOConfig) -> Result<String, EaseoError> {
    let value = url_or_path.trim().to_string();
    if value.is_empty() {
        return Err(EaseoError::InvalidUrl(
            "url_or_path must be a non-empty string".to_string(),
        ));
    }

    let parsed_input = url::Url::parse(&value).ok();
    let parsed_base = url::Url::parse(&config.public_base_url)
        .map_err(|e| EaseoError::InvalidConfiguration(format!("invalid public_base_url: {}", e)))?;

    let (path, query) = match parsed_input {
        Some(ref input) => {
            if input.scheme().is_empty() && input.host_str().is_none() {
                let parts: Vec<&str> = value.splitn(2, '?').collect();
                (
                    parts[0].to_string(),
                    parts.get(1).unwrap_or(&"").to_string(),
                )
            } else {
                (
                    input.path().to_string(),
                    input.query().unwrap_or("").to_string(),
                )
            }
        }
        None => {
            let parts: Vec<&str> = value.splitn(2, '?').collect();
            (
                parts[0].to_string(),
                parts.get(1).unwrap_or(&"").to_string(),
            )
        }
    };

    let base_path = parsed_base.path().trim_end_matches('/');
    let mut route = path.clone();
    if !base_path.is_empty() {
        let already_has_base = if route == base_path {
            true
        } else if let Some(rest) = route.strip_prefix(base_path) {
            rest.starts_with('/')
        } else {
            false
        };
        if !already_has_base {
            if route.starts_with('/') {
                route = format!("{}{}", base_path, route);
            } else {
                route = format!("{}/{}", base_path, route);
            }
        }
    }

    let normalized_path = normalize_path(&route, &config.url_policy)?;
    let normalized_query = filter_query(&query, &config.url_policy);

    let scheme = if config.url_policy.enforce_https {
        "https"
    } else {
        parsed_base.scheme()
    };

    let mut result = format!("{}://{}", scheme, config.canonical_host);
    result.push_str(&normalized_path);
    if !normalized_query.is_empty() {
        result.push('?');
        result.push_str(&normalized_query);
    }
    Ok(result)
}

pub(crate) fn collapse_duplicate_slashes(path: &str) -> String {
    let mut out = Vec::new();
    let mut prev_slash = false;
    for ch in path.chars() {
        if ch == '/' {
            if prev_slash {
                continue;
            }
            prev_slash = true;
            out.push(ch);
        } else {
            prev_slash = false;
            out.push(ch);
        }
    }
    out.into_iter().collect()
}

pub(crate) fn apply_trailing_slash(path: &str, mode: &TrailingSlash) -> String {
    match mode {
        TrailingSlash::Preserve => path.to_string(),
        TrailingSlash::Always => {
            if path == "/" || path.ends_with('/') {
                path.to_string()
            } else {
                format!("{}/", path)
            }
        }
        TrailingSlash::Never => {
            if path == "/" {
                path.to_string()
            } else {
                path.trim_end_matches('/').to_string()
            }
        }
    }
}

pub(crate) fn filter_query(query: &str, policy: &URLPolicy) -> String {
    if query.is_empty() {
        return String::new();
    }

    let mut pairs: Vec<(String, String)> = query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            if key.is_empty() {
                return None;
            }
            let value = parts.next().unwrap_or("").to_string();
            Some((key, value))
        })
        .collect();

    if policy.strip_tracking_params {
        pairs.retain(|(k, _)| !crate::detrack::is_tracking_param(k));
    }

    if !policy.allowed_query_params.is_empty() {
        let allowlist: std::collections::HashSet<&str> = policy
            .allowed_query_params
            .iter()
            .map(|s| s.as_str())
            .collect();
        pairs.retain(|(k, _)| allowlist.contains(k.as_str()));
    }

    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}
