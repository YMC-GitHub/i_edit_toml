use thiserror::Error;

/// Error types for TOML extraction operations
#[derive(Error, Debug)]
pub enum TomlExtractError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid TOML syntax in {file}: {error}")]
    InvalidToml { file: String, error: String },

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Array index out of bounds: {path}[{index}], array length: {length}")]
    ArrayIndexOutOfBounds {
        path: String,
        index: usize,
        length: usize,
    },

    #[error("Not an array: {0}")]
    NotAnArray(String),

    #[error("Invalid array index: {0}")]
    InvalidArrayIndex(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}
