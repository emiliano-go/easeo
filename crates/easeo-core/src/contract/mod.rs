pub mod assertion;
pub mod builder;
pub mod model;

pub use assertion::resolve_references;
pub use builder::build_contract;
pub use model::*;

impl SEOContract {
    pub fn to_json(&self) -> Result<String, crate::error::EaseoError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    pub fn to_dict(&self) -> Result<serde_json::Value, crate::error::EaseoError> {
        serde_json::to_value(self)
            .map_err(|e| crate::error::EaseoError::SerializationError(e.to_string()))
    }

    pub fn hash(&self) -> Result<String, crate::error::EaseoError> {
        use sha2::{Digest, Sha256};
        let json = self.to_json()?;
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn write(&self, path: &str) -> Result<(), crate::error::EaseoError> {
        let json = self.to_json()?;
        std::fs::write(path, json).map_err(|e| {
            crate::error::EaseoError::SerializationError(format!("failed to write contract: {}", e))
        })
    }
}
