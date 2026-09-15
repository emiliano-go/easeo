pub mod model;
pub mod assertion;
pub mod builder;

pub use model::*;
pub use assertion::resolve_references;
pub use builder::build_contract;

impl SEOContract {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_dict(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    pub fn hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let json = self.to_json();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn write(&self, path: &str) -> Result<(), crate::error::EaseoError> {
        let json = self.to_json();
        std::fs::write(path, json)
            .map_err(|e| crate::error::EaseoError::SerializationError(format!("failed to write contract: {}", e)))
    }
}
