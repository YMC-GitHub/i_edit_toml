use anyhow::Result;
use serde_json;
use std::collections::HashMap;

/// Configuration for getting values from TOML
#[derive(Debug, Clone)]
pub struct GetConfig {
    /// Path to the TOML file
    pub file_path: String,
    /// Dot-separated path to the field
    pub field_path: String,
    /// Output format (None for raw value, Some("json") for JSON)
    pub output_format: Option<String>,
    /// Whether to strip quotes from string values
    pub strip_quotes: bool,
}

impl Default for GetConfig {
    fn default() -> Self {
        Self {
            file_path: "Cargo.toml".to_string(),
            field_path: "package.name".to_string(),
            output_format: None,
            strip_quotes: false,
        }
    }
}

/// Result of getting multiple fields from TOML
#[derive(Debug, Clone)]
pub struct GetResult {
    /// Extracted fields and their values
    pub fields: HashMap<String, String>,
    /// Path to the source TOML file
    pub source_file: String,
}

impl GetResult {
    /// Create a new get result
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

    /// Get the number of fields
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self.fields).map_err(|e| e.into())
    }

    /// Convert to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.fields).map_err(|e| e.into())
    }

    /// Get an iterator over the fields
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.fields.iter()
    }
}

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
    fn test_get_result() {
        let mut result = GetResult::new("test.toml".to_string());
        
        assert!(result.is_empty());
        assert_eq!(result.len(), 0);
        assert_eq!(result.source_file, "test.toml");
        
        // 添加字段
        result.add_field("package.name".to_string(), "test".to_string());
        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("package.name"), Some(&"test".to_string()));
        
        // 添加第二个字段
        result.add_field("package.version".to_string(), "0.1.0".to_string());
        assert_eq!(result.len(), 2);
        
        // 测试迭代器
        let mut fields: Vec<_> = result.iter().collect();
        fields.sort_by_key(|(k, _)| *k);
        assert_eq!(fields.len(), 2);
        
        // 测试 JSON 序列化
        let json = result.to_json().unwrap();
        assert!(json.contains("test"));
        let json_pretty = result.to_json_pretty().unwrap();
        assert!(json_pretty.contains("test"));
    }
}