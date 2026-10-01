use thiserror::Error;

/// Errors returned by `easeo-core`.
#[derive(Error, Debug)]
pub enum EaseoError {
    /// A URL is malformed.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// A configuration value failed validation.
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// An entity value failed validation.
    #[error("invalid entity: {0}")]
    InvalidEntity(String),

    /// JSON-LD construction failed.
    #[error("invalid schema: {0}")]
    InvalidSchema(String),

    /// A value could not be serialized or deserialized.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Contract generation failed.
    #[error("contract error: {0}")]
    ContractError(String),

    /// A URL policy value is invalid.
    #[error("URL policy error: {0}")]
    URLPolicyError(String),
}
