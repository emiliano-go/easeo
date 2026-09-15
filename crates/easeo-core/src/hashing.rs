use sha2::{Sha256, Digest};
use crate::payload::SEOPayload;

pub fn hash_payload(payload: &SEOPayload) -> String {
    let json = serde_json::to_string(payload).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn etag_payload(payload: &SEOPayload) -> String {
    format!("\"{}\"", hash_payload(payload))
}
