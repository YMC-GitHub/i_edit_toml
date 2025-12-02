//! Core functionality for get command

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use toml::Value as TomlValue;

use super::types::{ExtractConfig, ExtractionResult};
use crate::error::TomlExtractError;
use crate::get::utils::{format_output, get_nested_value, strip_quotes_internal};

/// Extract a single field from a TOML file
///
/// # Errors
/// Returns `Err` if:
/// - The file doesn't exist or can't be read
/// - The field path doesn't exist
/// - The TOML syntax is invalid
/// - Array index is out of bounds
pub fn extract_field(config: &ExtractConfig) -> Result<String> {
    let content = fs::read_to_string(&config.file_path)
        .context(format!("Failed to read file: {}", config.file_path))?;

    let value: TomlValue = toml::from_str(&content)
        .context(format!("Invalid TOML syntax in: {}", config.file_path))?;

    let field_value = get_nested_value(&value, &config.field_path)
        .context(format!("Field not found: {}", config.field_path))?;

    let mut result = format_output(field_value, config.output_format.as_deref())?;

    if config.strip_quotes {
        result = strip_quotes_internal(&result);
    }

    Ok(result)
}

/// Extract multiple fields from a TOML file
///
/// # Arguments
/// * `file_path` - Path to the TOML file
/// * `field_paths` - List of field paths to extract
/// * `strip_quotes` - Whether to strip quotes from string values
///
/// # Returns
/// An `ExtractionResult` containing all extracted fields and their values
pub fn extract_multiple_fields(
    file_path: &str,
    field_paths: &[String],
    strip_quotes: bool,
) -> Result<ExtractionResult> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: TomlValue =
        toml::from_str(&content).context(format!("Invalid TOML syntax in: {}", file_path))?;

    let mut result = ExtractionResult::new(file_path.to_string());

    for field_path in field_paths {
        let field_value = get_nested_value(&value, field_path)
            .context(format!("Field not found: {}", field_path))?;

        let mut formatted_value = format_output(field_value, None)?;
        if strip_quotes {
            formatted_value = strip_quotes_internal(&formatted_value);
        }

        result.add_field(field_path.clone(), formatted_value);
    }

    Ok(result)
}

/// Extract an array from a TOML file
///
/// # Arguments
/// * `file_path` - Path to the TOML file
/// * `array_path` - Path to the array in the TOML structure
/// * `output_format` - Optional output format (None for raw, "json", or "json-pretty")
///
/// # Returns
/// The array as a formatted string
pub fn extract_array(
    file_path: &str,
    array_path: &str,
    output_format: Option<&str>,
) -> Result<String> {
    let config = ExtractConfig {
        file_path: file_path.to_string(),
        field_path: array_path.to_string(),
        output_format: output_format.map(|s| s.to_string()),
        strip_quotes: false,
    };
    extract_field(&config)
}

/// Extract array length from a TOML file
///
/// # Arguments
/// * `file_path` - Path to the TOML file
/// * `array_path` - Path to the array in the TOML structure
///
/// # Returns
/// The length of the array as a `usize`
pub fn extract_array_length(file_path: &str, array_path: &str) -> Result<usize> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: TomlValue =
        toml::from_str(&content).context(format!("Invalid TOML syntax in: {}", file_path))?;

    let array_value =
        get_nested_value(&value, array_path).context(format!("Array not found: {}", array_path))?;

    let array = array_value
        .as_array()
        .ok_or_else(|| TomlExtractError::NotAnArray(array_path.to_string()))?;

    Ok(array.len())
}

/// Extract a specific array element from a TOML file
///
/// # Arguments
/// * `file_path` - Path to the TOML file
/// * `array_path` - Path to the array in the TOML structure
/// * `index` - Index of the element to extract (0-based)
/// * `strip_quotes` - Whether to strip quotes from string values
///
/// # Returns
/// The array element as a string
pub fn extract_array_element(
    file_path: &str,
    array_path: &str,
    index: usize,
    strip_quotes: bool,
) -> Result<String> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: TomlValue =
        toml::from_str(&content).context(format!("Invalid TOML syntax in: {}", file_path))?;

    let array_value =
        get_nested_value(&value, array_path).context(format!("Array not found: {}", array_path))?;

    let array = array_value
        .as_array()
        .ok_or_else(|| TomlExtractError::NotAnArray(array_path.to_string()))?;

    if index >= array.len() {
        return Err(TomlExtractError::ArrayIndexOutOfBounds {
            path: array_path.to_string(),
            index,
            length: array.len(),
        }
        .into());
    }

    let element = &array[index];
    let mut result = format_output(element, None)?;

    if strip_quotes {
        result = strip_quotes_internal(&result);
    }

    Ok(result)
}

// Preset extraction functions

/// Get the package name from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
///
/// # Returns
/// The package name as a string
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

/// Get the package version from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
///
/// # Returns
/// The package version as a string
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

/// Get all dependencies from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
///
/// # Returns
/// A HashMap of dependency names to their versions
pub fn get_dependencies(file_path: Option<&str>) -> Result<HashMap<String, String>> {
    let path = file_path.unwrap_or("Cargo.toml");
    let content = fs::read_to_string(path).context("Failed to read Cargo.toml")?;

    let value: TomlValue = toml::from_str(&content).context("Invalid TOML syntax in Cargo.toml")?;

    let mut dependencies = HashMap::new();

    if let Some(deps) = value.get("dependencies") {
        if let Some(table) = deps.as_table() {
            for (name, value) in table {
                let version = match value {
                    TomlValue::String(s) => s.clone(),
                    TomlValue::Table(t) => {
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

/// Get package authors from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
/// * `index` - Optional index to get a specific author (returns all if None)
/// * `strip_quotes` - Whether to strip quotes from the result
///
/// # Returns
/// The authors (or specific author) as a string
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

/// Get package keywords from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
/// * `index` - Optional index to get a specific keyword (returns all if None)
/// * `strip_quotes` - Whether to strip quotes from the result
///
/// # Returns
/// The keywords (or specific keyword) as a string
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

/// Get package categories from a Cargo.toml file
///
/// # Arguments
/// * `file_path` - Optional path to Cargo.toml (uses "Cargo.toml" if None)
/// * `index` - Optional index to get a specific category (returns all if None)
/// * `strip_quotes` - Whether to strip quotes from the result
///
/// # Returns
/// The categories (or specific category) as a string
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
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_field_basic() {
        // 创建临时 TOML 文件
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[package]\nname = \"test\"").unwrap();
        let path = temp_file.path().to_str().unwrap();

        // 测试提取逻辑
        let config = ExtractConfig {
            file_path: path.to_string(),
            field_path: "package.name".to_string(),
            ..Default::default()
        };
        assert_eq!(extract_field(&config).unwrap(), "\"test\"");
    }

    #[test]
    fn test_extract_field_strip_quotes() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "version = \"1.0.0\"").unwrap();
        let path = temp_file.path().to_str().unwrap();

        let config = ExtractConfig {
            file_path: path.to_string(),
            field_path: "version".to_string(),
            strip_quotes: true,
            ..Default::default()
        };
        assert_eq!(extract_field(&config).unwrap(), "1.0.0");
    }

    #[test]
    fn test_extract_multiple_fields() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[package]\nname = \"test\"\nversion = \"1.0.0\"").unwrap();
        let path = temp_file.path().to_str().unwrap();

        let fields = vec!["package.name".to_string(), "package.version".to_string()];
        let result = extract_multiple_fields(path, &fields, false).unwrap();

        assert_eq!(result.get("package.name"), Some(&"\"test\"".to_string()));
        assert_eq!(
            result.get("package.version"),
            Some(&"\"1.0.0\"".to_string())
        );
    }

    #[test]
    fn test_extract_array_length() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "authors = [\"Alice\", \"Bob\"]").unwrap();
        let path = temp_file.path().to_str().unwrap();

        assert_eq!(extract_array_length(path, "authors").unwrap(), 2);
    }

    #[test]
    fn test_get_dependencies() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "[dependencies]\nserde = \"1.0\"\ntoml = {{ version = \"0.8\" }}"
        )
        .unwrap();
        let path = temp_file.path().to_str().unwrap();

        let deps = get_dependencies(Some(path)).unwrap();
        let expected = HashMap::from([
            ("serde".to_string(), "1.0".to_string()),
            ("toml".to_string(), "0.8".to_string()),
        ]);
        assert_eq!(deps, expected);
    }
}
