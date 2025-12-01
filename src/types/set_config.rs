/// Configuration for setting values in TOML
#[derive(Debug, Clone)]
pub struct SetConfig {
    /// Path to the TOML file
    pub file_path: String,
    /// Dot-separated path to the field
    pub field_path: String,
    /// Value to set
    pub value: String,
    /// Value type (auto-detected if None)
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

impl SetConfig {
    /// Create a new SetConfig with auto type detection
    pub fn new(file_path: String, field_path: String, value: String) -> Self {
        Self {
            file_path,
            field_path,
            value,
            value_type: None,
            create_missing: false,
        }
    }

    /// Set value type explicitly
    pub fn with_type(mut self, value_type: &str) -> Self {
        self.value_type = Some(value_type.to_string());
        self
    }

    /// Enable creating missing fields
    pub fn with_create_missing(mut self, create_missing: bool) -> Self {
        self.create_missing = create_missing;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_config_default() {
        let config = SetConfig::default();
        assert_eq!(config.file_path, "Cargo.toml");
        assert_eq!(config.field_path, "package.name");
        assert_eq!(config.value, "");
        assert!(config.value_type.is_none());
        assert!(!config.create_missing);
    }

    #[test]
    fn test_set_config_builder() {
        let config = SetConfig::new(
            "test.toml".to_string(),
            "package.name".to_string(),
            "new-name".to_string(),
        );
        
        assert_eq!(config.file_path, "test.toml");
        assert_eq!(config.field_path, "package.name");
        assert_eq!(config.value, "new-name");
        assert!(config.value_type.is_none());
        assert!(!config.create_missing);
        
        // 测试链式调用
        let config = config
            .with_type("string")
            .with_create_missing(true);
            
        assert_eq!(config.value_type, Some("string".to_string()));
        assert!(config.create_missing);
    }
}
