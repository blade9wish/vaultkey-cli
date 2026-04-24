use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Failed to read config file '{0}': {1}")]
    ConfigRead(String, #[source] std::io::Error),

    #[error("Failed to parse config TOML: {0}")]
    ConfigParse(String),

    #[error("Config validation error: {0}")]
    Validation(String),
}
