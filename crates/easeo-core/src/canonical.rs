use crate::url::normalize_public_url;
use crate::config::SEOConfig;
use crate::error::EaseoError;

pub fn build_canonical(route: &str, config: &SEOConfig) -> Result<String, EaseoError> {
    normalize_public_url(route, config)
}
