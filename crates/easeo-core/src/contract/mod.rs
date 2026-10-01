/// Contract assertion helpers.
pub mod assertion;
/// Contract builder.
pub mod builder;
/// Contract model types.
pub mod model;

pub use assertion::resolve_references;
pub use builder::build_contract;
pub use model::*;

impl SEOContract {
    /// Serializes the contract to pretty printed JSON.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn to_json(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Converts the contract to a JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn to_dict(&self) -> Result<serde_json::Value, crate::error::EaseoError> {
        serde_json::to_value(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    /// Returns the SHA-256 hash of the serialized contract.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization fails.
    pub fn hash(&self) -> Result<String, crate::error::EaseoError> {
        use sha2::{Digest, Sha256};
        let json = self.to_json()?;
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Writes the contract as JSON to the given path.
    ///
    /// # Errors
    ///
    /// Returns [`crate::EaseoError::SerializationError`] when serialization or
    /// the file write fails.
    pub fn write(&self, path: &str) -> Result<(), crate::error::EaseoError> {
        let json = self.to_json()?;
        std::fs::write(path, json).map_err(|e| {
            crate::error::EaseoError::SerializationError(format!("failed to write contract: {}", e))
        })
    }
}
