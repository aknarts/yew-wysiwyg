//! Error types for yew-wysiwyg

use thiserror::Error;

/// Result type for yew-wysiwyg operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in yew-wysiwyg
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid widget configuration: {0}")]
    InvalidConfig(String),

    #[error("Theme error: {0}")]
    ThemeError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}
