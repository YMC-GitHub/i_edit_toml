//! Configuration types for set operations

/// Configuration for field setting
#[derive(Debug, Clone)]
pub struct SetConfig {
    /// Path to the TOML file
    pub file_path: String,
    /// Dot-separated path to the field
    pub field_path: String,
    /// Value to set
    pub value: String,
    /// Value type (None for auto-detect)
    pub value_type: Option<String>,
    /// Whether to create missing parent fields
    pub create_missing: bool,
}

impl Default for SetConfig {
    fn default() -> Self {
        Self {
            file_path: "Cargo.toml".to_string(),
            field_path: "package.name".to_string(),
            value: "".to_string(),
            value_type: None,
            create_missing: false,
        }
    }
}