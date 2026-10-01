use crate::error::EaseoError;
use crate::payload::SEOPayload;
use sha2::{Digest, Sha256};

/// Returns the SHA-256 hash of the canonical JSON payload.
///
/// # Errors
///
/// Returns [`EaseoError`] when the payload cannot be serialized.
pub fn hash_payload(payload: &SEOPayload) -> Result<String, EaseoError> {
    let json = serde_json::to_string(payload)
        .map_err(|e| EaseoError::SerializationError(e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

/// Returns the payload hash as a quoted HTTP ETag value.
///
/// # Errors
///
/// Returns [`EaseoError`] when the payload cannot be serialized.
pub fn etag_payload(payload: &SEOPayload) -> Result<String, EaseoError> {
    Ok(format!("\"{}\"", hash_payload(payload)?))
}
