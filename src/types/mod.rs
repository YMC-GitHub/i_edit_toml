//! Types used throughout the TOML extraction library.
//!
//! This module contains configuration and result types for getting and setting
//! values in TOML files.

mod get_config;
mod set_config;

// Re-export
pub use get_config::{GetConfig, GetResult};
pub use set_config::SetConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_default() {
        let config = GetConfig::default();
        assert_eq!(config.file_path, "Cargo.toml");
        assert_eq!(config.field_path, "package.name");
        assert!(config.output_format.is_none());
        assert!(!config.strip_quotes);
    }

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
    fn test_get_result() {
        let mut result = GetResult::new("test.toml".to_string());
        assert!(result.is_empty());

        result.add_field("package.name".to_string(), "test".to_string());
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("package.name"), Some(&"test".to_string()));
    }
}
