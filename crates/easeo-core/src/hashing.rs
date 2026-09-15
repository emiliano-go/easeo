use crate::error::EaseoError;
use crate::payload::SEOPayload;
use sha2::{Digest, Sha256};

pub fn hash_payload(payload: &SEOPayload) -> Result<String, EaseoError> {
    let json = serde_json::to_string(payload)
        .map_err(|e| EaseoError::SerializationError(e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn etag_payload(payload: &SEOPayload) -> Result<String, EaseoError> {
    Ok(format!("\"{}\"", hash_payload(payload)?))
}
