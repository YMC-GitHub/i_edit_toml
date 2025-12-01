use thiserror::Error;

/// Error types for TOML operations
#[derive(Error, Debug)]
pub enum TomlError {
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

    #[error("Invalid value type: expected {expected}, got {actual}")]
    InvalidValueType { expected: String, actual: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Value conversion error: {0}")]
    ValueConversion(String),

    #[error("Path parsing error: {0}")]
    PathParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = TomlError::FileNotFound("test.toml".to_string());
        assert_eq!(error.to_string(), "File not found: test.toml");

        let error = TomlError::InvalidToml {
            file: "test.toml".to_string(),
            error: "invalid syntax".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid TOML syntax in test.toml: invalid syntax");

        let error = TomlError::FieldNotFound("package.name".to_string());
        assert_eq!(error.to_string(), "Field not found: package.name");

        let error = TomlError::ArrayIndexOutOfBounds {
            path: "authors".to_string(),
            index: 5,
            length: 2,
        };
        assert_eq!(error.to_string(), "Array index out of bounds: authors[5], array length: 2");

        let error = TomlError::PathParseError("test".to_string());
        assert_eq!(error.to_string(), "Path parsing error: test");
    }

    #[test]
    fn test_error_from_io() {
        use std::io;
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let toml_error: TomlError = io_error.into();
        assert!(toml_error.to_string().contains("IO error"));
    }
}