//! Type definitions for get command

use std::collections::HashMap;

/// Configuration for field extraction
#[derive(Debug, Clone)]
pub struct ExtractConfig {
    /// Path to the TOML file
    pub file_path: String,
    /// Dot-separated path to the field (e.g., "package.name", "dependencies.serde", "authors[0]")
    pub field_path: String,
    /// Output format (None for raw value, Some("json") for JSON)
    pub output_format: Option<String>,
    /// Whether to strip quotes from string values
    pub strip_quotes: bool,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            file_path: "Cargo.toml".to_string(),
            field_path: "package.name".to_string(),
            output_format: None,
            strip_quotes: false,
        }
    }
}

/// Result of multiple field extraction
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// Extracted fields and their values
    pub fields: HashMap<String, String>,
    /// Path to the source TOML file
    pub source_file: String,
}

impl ExtractionResult {
    /// Create a new extraction result
    pub fn new(source_file: String) -> Self {
        Self {
            fields: HashMap::new(),
            source_file,
        }
    }

    /// Add a field to the result
    pub fn add_field(&mut self, field_path: String, value: String) {
        self.fields.insert(field_path, value);
    }

    /// Get a field value
    pub fn get(&self, field_path: &str) -> Option<&String> {
        self.fields.get(field_path)
    }

    /// Check if result is empty
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.fields)
    }

    /// Convert to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.fields)
    }
}