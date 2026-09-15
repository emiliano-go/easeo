use crate::config::SEOConfig;
use crate::error::EaseoError;
use crate::url::normalize_public_url;

pub fn build_canonical(route: &str, config: &SEOConfig) -> Result<String, EaseoError> {
    normalize_public_url(route, config)
}
