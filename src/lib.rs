//! A lightweight, high-performance TOML field extraction library.
//!
//! This crate provides functionality to extract specific fields from TOML files
//! with support for nested keys, arrays, and multiple output formats.


pub mod error; // 1
pub use error::TomlExtractError;

pub mod types; // 2
pub use types::{ExtractConfig, ExtractionResult};

pub mod utils;  // 3
pub use utils::strip_quotes;  // 保留公共API的strip_quotes

// #[allow(unused_imports)]
// use utils::{format_output, get_nested_value, strip_quotes_internal, to_json_value,};

pub mod extract; // 4
pub use extract::*;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

// remove code for extracting to error module
// ...
// remove code for extracting to types module
// ...
// remove code for extracting to utils module
// ...


/// Get package name from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
///
/// # Returns
/// Package name as string
///
/// # Examples
/// ```
/// use toml_code::get_package_name;
///
/// let package_name = get_package_name(None).unwrap();
/// println!("Package name: {}", package_name);
/// ```
pub fn get_package_name(file_path: Option<&str>) -> Result<String> {
    let path = file_path.unwrap_or("Cargo.toml");
    let config = ExtractConfig {
        file_path: path.to_string(),
        field_path: "package.name".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    extract_field(&config)
}

/// Get package version from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
///
/// # Returns
/// Package version as string
pub fn get_package_version(file_path: Option<&str>) -> Result<String> {
    let path = file_path.unwrap_or("Cargo.toml");
    let config = ExtractConfig {
        file_path: path.to_string(),
        field_path: "package.version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    extract_field(&config)
}

/// Get all dependencies from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
///
/// # Returns
/// HashMap containing dependency names and versions
pub fn get_dependencies(file_path: Option<&str>) -> Result<HashMap<String, String>> {
    let path = file_path.unwrap_or("Cargo.toml");
    let content = fs::read_to_string(path).context("Failed to read Cargo.toml")?;

    let value: toml::Value =
        toml::from_str(&content).context("Invalid TOML syntax in Cargo.toml")?;

    let mut dependencies = HashMap::new();

    if let Some(deps) = value.get("dependencies") {
        if let Some(table) = deps.as_table() {
            for (name, value) in table {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => {
                        if let Some(version) = t.get("version") {
                            version.as_str().unwrap_or("").to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    _ => value.to_string(),
                };
                dependencies.insert(name.clone(), version);
            }
        }
    }

    Ok(dependencies)
}

/// Get package authors from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
/// * `index` - Optional array index to get specific author
/// * `strip_quotes` - Whether to strip quotes from string values
///
/// # Returns
/// Authors array or specific author as string
pub fn get_package_authors(
    file_path: Option<&str>,
    index: Option<usize>,
    strip_quotes: bool,
) -> Result<String> {
    let path = file_path.unwrap_or("Cargo.toml");

    if let Some(idx) = index {
        extract_array_element(path, "package.authors", idx, strip_quotes)
    } else {
        extract_array(path, "package.authors", None)
    }
}

/// Get package keywords from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
/// * `index` - Optional array index to get specific keyword
/// * `strip_quotes` - Whether to strip quotes from string values
///
/// # Returns
/// Keywords array or specific keyword as string
pub fn get_package_keywords(
    file_path: Option<&str>,
    index: Option<usize>,
    strip_quotes: bool,
) -> Result<String> {
    let path = file_path.unwrap_or("Cargo.toml");

    if let Some(idx) = index {
        extract_array_element(path, "package.keywords", idx, strip_quotes)
    } else {
        extract_array(path, "package.keywords", None)
    }
}

/// Get package categories from Cargo.toml (convenience function)
///
/// # Arguments
/// * `file_path` - Path to Cargo.toml (defaults to "Cargo.toml")
/// * `index` - Optional array index to get specific category
/// * `strip_quotes` - Whether to strip quotes from string values
///
/// # Returns
/// Categories array or specific category as string
pub fn get_package_categories(
    file_path: Option<&str>,
    index: Option<usize>,
    strip_quotes: bool,
) -> Result<String> {
    let path = file_path.unwrap_or("Cargo.toml");

    if let Some(idx) = index {
        extract_array_element(path, "package.categories", idx, strip_quotes)
    } else {
        extract_array(path, "package.categories", None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ... 保留之前的测试 ...

    #[test]
    fn test_array_element_extraction() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [package]
            name = "test-package"
            authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
            keywords = ["rust", "toml", "cli"]
        "#;
        let toml_file = temp_dir.path().join("test.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试数组元素访问
        let result =
            extract_array_element(&toml_file.to_string_lossy(), "package.authors", 0, true)?;
        assert_eq!(result, "Alice <alice@example.com>");

        let result =
            extract_array_element(&toml_file.to_string_lossy(), "package.authors", 1, true)?;
        assert_eq!(result, "Bob <bob@example.com>");

        // 测试数组越界
        let result =
            extract_array_element(&toml_file.to_string_lossy(), "package.authors", 2, true);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_array_extraction() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [package]
            name = "test-package"
            authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
        "#;
        let toml_file = temp_dir.path().join("test.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试整个数组提取
        let result = extract_array(&toml_file.to_string_lossy(), "package.authors", None)?;
        assert!(result.contains("Alice <alice@example.com>"));
        assert!(result.contains("Bob <bob@example.com>"));

        Ok(())
    }

    #[test]
    fn test_array_length() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [package]
            name = "test-package"
            authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
            keywords = ["rust"]
        "#;
        let toml_file = temp_dir.path().join("test.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试数组长度
        let length = extract_array_length(&toml_file.to_string_lossy(), "package.authors")?;
        assert_eq!(length, 2);

        let length = extract_array_length(&toml_file.to_string_lossy(), "package.keywords")?;
        assert_eq!(length, 1);

        Ok(())
    }

    #[test]
    fn test_nested_array_access() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [package]
            name = "test-package"

            [[bin]]
            name = "first"
            path = "src/main.rs"

            [[bin]]
            name = "second" 
            path = "src/bin/second.rs"
        "#;
        let toml_file = temp_dir.path().join("test.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试嵌套数组访问
        let config = ExtractConfig {
            file_path: toml_file.to_string_lossy().to_string(),
            field_path: "bin[0].name".to_string(),
            output_format: None,
            strip_quotes: true,
        };
        let result = extract_field(&config)?;
        assert_eq!(result, "first");

        let config = ExtractConfig {
            file_path: toml_file.to_string_lossy().to_string(),
            field_path: "bin[1].name".to_string(),
            output_format: None,
            strip_quotes: true,
        };
        let result = extract_field(&config)?;
        assert_eq!(result, "second");

        Ok(())
    }

    #[test]
    fn test_convenience_array_functions() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [package]
            name = "test-package"
            authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
            keywords = ["rust", "toml"]
            categories = ["development-tools"]
        "#;
        let toml_file = temp_dir.path().join("Cargo.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试便捷函数
        let author = get_package_authors(Some(&toml_file.to_string_lossy()), Some(0), true)?;
        assert_eq!(author, "Alice <alice@example.com>");

        let keyword = get_package_keywords(Some(&toml_file.to_string_lossy()), Some(1), true)?;
        assert_eq!(keyword, "toml");

        let categories = get_package_categories(Some(&toml_file.to_string_lossy()), None, false)?;
        assert!(categories.contains("development-tools"));

        Ok(())
    }

    #[test]
    fn test_complex_array_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let toml_content = r#"
            [workspace]
            members = ["crates/a", "crates/b"]

            [dependencies]
            serde = { version = "1.0", features = ["derive"] }
        "#;
        let toml_file = temp_dir.path().join("test.toml");
        fs::write(&toml_file, toml_content)?;

        // 测试复杂路径中的数组访问
        let result =
            extract_array_element(&toml_file.to_string_lossy(), "workspace.members", 0, true)?;
        assert_eq!(result, "crates/a");

        let result =
            extract_array_element(&toml_file.to_string_lossy(), "workspace.members", 1, true)?;
        assert_eq!(result, "crates/b");

        Ok(())
    }
}
