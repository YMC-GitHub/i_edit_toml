//! Utility functions for get command

use anyhow::Result;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

use crate::error::TomlExtractError;

/// Resolve nested value from TOML structure using dot-separated path with array support
pub fn get_nested_value<'a>(
    value: &'a TomlValue,
    path: &str,
) -> Result<&'a TomlValue, TomlExtractError> {
    let mut current = value;

    for part in path.split('.') {
        // Handle array access syntax [index]
        if part.contains('[') && part.ends_with(']') {
            let bracket_start = part.find('[').ok_or_else(|| {
                TomlExtractError::InvalidArrayIndex(format!("Invalid array syntax: {}", part))
            })?;
            let array_name = &part[..bracket_start];
            let index_str = &part[bracket_start + 1..part.len() - 1];
            let index = index_str.parse::<usize>().map_err(|_| {
                TomlExtractError::InvalidArrayIndex(format!("Invalid array index: {}", index_str))
            })?;

            // Get array from current value
            current = current
                .get(array_name)
                .ok_or_else(|| TomlExtractError::FieldNotFound(array_name.to_string()))?;
            let array = current
                .as_array()
                .ok_or_else(|| TomlExtractError::NotAnArray(array_name.to_string()))?;

            // Check index bounds
            if index >= array.len() {
                return Err(TomlExtractError::ArrayIndexOutOfBounds {
                    path: array_name.to_string(),
                    index,
                    length: array.len(),
                });
            }

            current = &array[index];
        } else {
            // Regular field access
            current = current
                .get(part)
                .ok_or_else(|| TomlExtractError::FieldNotFound(part.to_string()))?;
        }
    }

    Ok(current)
}

/// Format TOML value for output based on specified format
pub fn format_output(value: &TomlValue, output_format: Option<&str>) -> Result<String> {
    match output_format {
        Some("json") => {
            let json_value = to_json_value(value)?;
            Ok(serde_json::to_string(&json_value)?)
        }
        Some("json-pretty") => {
            let json_value = to_json_value(value)?;
            Ok(serde_json::to_string_pretty(&json_value)?)
        }
        _ => Ok(value.to_string()),
    }
}

/// Convert TomlValue to serde_json::Value
pub fn to_json_value(toml_value: &TomlValue) -> Result<JsonValue> {
    match toml_value {
        TomlValue::String(s) => Ok(JsonValue::String(s.clone())),
        TomlValue::Integer(i) => Ok(JsonValue::Number((*i).into())),
        TomlValue::Float(f) => Ok(JsonValue::Number(
            serde_json::Number::from_f64(*f)
                .ok_or_else(|| anyhow::anyhow!("Cannot convert float {} to JSON number", f))?,
        )),
        TomlValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
        TomlValue::Array(arr) => {
            let mut json_arr = Vec::with_capacity(arr.len());
            for elem in arr {
                json_arr.push(to_json_value(elem)?);
            }
            Ok(JsonValue::Array(json_arr))
        }
        TomlValue::Table(table) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in table {
                json_obj.insert(key.clone(), to_json_value(val)?);
            }
            Ok(JsonValue::Object(json_obj))
        }
        TomlValue::Datetime(dt) => Ok(JsonValue::String(dt.to_string())),
    }
}

/// Strip surrounding quotes from a string if present
pub fn strip_quotes_internal(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Public API for stripping quotes
pub fn strip_quotes(s: &str) -> String {
    strip_quotes_internal(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value as TomlValue;

    #[test]
    fn test_get_nested_value() {
        let toml_str = r#"
            [package]
            name = "test"
            [dependencies.serde]
            version = "1.0"
            features = ["derive"]
            [array]
            items = [1, 2, 3]
        "#;
        let value: TomlValue = toml::from_str(toml_str).unwrap();

        // 测试普通字段
        assert_eq!(
            get_nested_value(&value, "package.name").unwrap(),
            &TomlValue::String("test".into())
        );

        // 测试嵌套字段
        assert_eq!(
            get_nested_value(&value, "dependencies.serde.version").unwrap(),
            &TomlValue::String("1.0".into())
        );

        // 测试数组
        assert_eq!(
            get_nested_value(&value, "array.items[1]").unwrap(),
            &TomlValue::Integer(2)
        );
    }

    #[test]
    fn test_strip_quotes_internal() {
        assert_eq!(strip_quotes_internal("\"hello\""), "hello");
        assert_eq!(strip_quotes_internal("'world'"), "world");
        assert_eq!(strip_quotes_internal("no_quotes"), "no_quotes");
    }

    #[test]
    fn test_to_json_value() {
        let toml_value = TomlValue::Integer(42);
        assert_eq!(
            to_json_value(&toml_value).unwrap(),
            serde_json::Value::Number(42.into())
        );

        let toml_value = TomlValue::String("test".into());
        assert_eq!(
            to_json_value(&toml_value).unwrap(),
            serde_json::Value::String("test".into())
        );
    }
}
