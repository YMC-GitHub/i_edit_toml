use thiserror::Error;

/// Error types for TOML extraction and manipulation operations.
#[derive(Error, Debug)]
pub enum TomlExtractError {
    /// The specified file was not found.
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// The TOML file contains invalid syntax.
    #[error("Invalid TOML syntax in {file}: {error}")]
    InvalidToml {
        /// Path to the invalid TOML file.
        file: String,
        /// Description of the syntax error.
        error: String,
    },

    /// The specified field path does not exist in the TOML file.
    #[error("Field not found: {0}")]
    FieldNotFound(String),

    /// An array index is out of bounds.
    #[error("Array index out of bounds: {path}[{index}], array length: {length}")]
    ArrayIndexOutOfBounds {
        /// Path to the array in the TOML file.
        path: String,
        /// The index that was accessed.
        index: usize,
        /// The actual length of the array.
        length: usize,
    },

    /// The specified value is not an array.
    #[error("Not an array: {0}")]
    NotAnArray(String),

    /// The array index is invalid (e.g., non-numeric).
    #[error("Invalid array index: {0}")]
    InvalidArrayIndex(String),

    /// An I/O error occurred while reading/writing the file.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// An error occurred during JSON serialization/deserialization.
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// The specified value is not a TOML table (dictionary).
    #[error("Not a table: {0}")]
    NotATable(String),

    /// The field path is invalid (e.g., empty segments, invalid syntax).
    #[error("Invalid field path: {0}")]
    InvalidFieldPath(String),

    /// The value type is invalid for the requested operation.
    #[error("Invalid value type: {0}")]
    InvalidValueType(String),
}