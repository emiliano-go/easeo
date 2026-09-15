use thiserror::Error;

#[derive(Error, Debug)]
pub enum EaseoError {
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("invalid entity: {0}")]
    InvalidEntity(String),

    #[error("invalid schema: {0}")]
    InvalidSchema(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("contract error: {0}")]
    ContractError(String),

    #[error("URL policy error: {0}")]
    URLPolicyError(String),
}
