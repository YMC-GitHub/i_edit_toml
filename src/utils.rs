//! Utility functions for TOML processing

// use serde_json::Value as JsonValue;
use anyhow::{Context, Result};
use serde_json;
use toml::Value as TomlValue;

use crate::TomlExtractError;

/// Resolve nested value from TOML structure using dot-separated path with array support
pub fn get_nested_value<'a>(value: &'a TomlValue, path: &str) -> Result<&'a TomlValue, TomlExtractError> {
    let mut current = value;

    for part in path.split('.') {
        // Handle array access syntax [index]
        if part.contains('[') && part.ends_with(']') {
            let bracket_start = part.find('[').ok_or_else(|| {
                TomlExtractError::InvalidArrayIndex(format!("Invalid array syntax: {}", part))
            })?;
            let array_name = &part[..bracket_start];
            let index_part = &part[bracket_start + 1..part.len() - 1];

            // Get the array
            current = current
                .get(array_name)
                .ok_or_else(|| TomlExtractError::FieldNotFound(array_name.to_string()))?;

            let array = current
                .as_array()
                .ok_or_else(|| TomlExtractError::NotAnArray(array_name.to_string()))?;

            // Parse index
            let index: usize = index_part
                .parse()
                .map_err(|_| TomlExtractError::InvalidArrayIndex(index_part.to_string()))?;

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
pub fn format_output(value: &TomlValue, format: Option<&str>) -> Result<String> {
    match format {
        Some("json") => {
            let json_value = to_json_value(value);
            serde_json::to_string(&json_value).context("Failed to convert to JSON")
        }
        Some("json-pretty") => {
            let json_value = to_json_value(value);
            serde_json::to_string_pretty(&json_value).context("Failed to convert to JSON")
        }
        _ => Ok(value.to_string()),
    }
}

pub fn to_json_value(toml_value: &TomlValue) -> serde_json::Value {
    // 保持原实现不变
    match toml_value {
        TomlValue::String(s) => serde_json::Value::String(s.clone()),
        TomlValue::Integer(i) => serde_json::Value::Number((*i).into()),
        TomlValue::Float(f) => {
            if let Some(n) = serde_json::Number::from_f64(*f) {
                serde_json::Value::Number(n)
            } else {
                serde_json::Value::Null
            }
        }
        TomlValue::Boolean(b) => serde_json::Value::Bool(*b),
        TomlValue::Datetime(dt) => serde_json::Value::String(dt.to_string()),
        TomlValue::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(to_json_value).collect())
        }
        TomlValue::Table(table) => {
            let mut map = serde_json::Map::new();
            for (k, v) in table {
                map.insert(k.clone(), to_json_value(v));
            }
            serde_json::Value::Object(map)
        }
    }
}

/// Strip surrounding quotes from a string value
pub fn strip_quotes(value: &str) -> String {
    value.trim_matches('"').to_string()
}

/// Internal helper for stripping quotes (avoids naming conflict with public function)
pub fn strip_quotes_internal(value: &str) -> String {
    strip_quotes(value)
}