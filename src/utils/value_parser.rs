use anyhow::{Context, Result};
use serde_json;
use toml::Value;

use crate::error::TomlError;

/// Parse string value to TOML value with optional type hint
pub fn parse_value(value: &str, value_type: Option<&str>) -> Result<Value> {
    match value_type {
        Some("string") => Ok(Value::String(value.to_string())),
        Some("integer") => {
            let int_val: i64 = value.parse().context("Failed to parse integer value")?;
            Ok(Value::Integer(int_val))
        }
        Some("float") => {
            let float_val: f64 = value.parse().context("Failed to parse float value")?;
            Ok(Value::Float(float_val))
        }
        Some("boolean") => {
            let bool_val = match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => {
                    return Err(TomlError::ValueConversion(format!(
                        "Invalid boolean value: {}",
                        value
                    ))
                    .into())
                }
            };
            Ok(Value::Boolean(bool_val))
        }
        Some("null") => Ok(Value::String("".to_string())),
        Some(unknown) => {
            Err(TomlError::ValueConversion(format!("Unknown value type: {}", unknown)).into())
        }
        None => auto_parse_value(value),
    }
}

/// Auto-detect and parse value type
pub fn auto_parse_value(value: &str) -> Result<Value> {
    // Try integer
    if let Ok(int_val) = value.parse::<i64>() {
        return Ok(Value::Integer(int_val));
    }

    // Try float
    if let Ok(float_val) = value.parse::<f64>() {
        return Ok(Value::Float(float_val));
    }

    // Try boolean
    if let Ok(bool_val) = value.parse::<bool>() {
        return Ok(Value::Boolean(bool_val));
    }

    // Default to string
    Ok(Value::String(value.to_string()))
}

/// Format TOML value to string based on format type
pub fn format_value(value: &Value, format: Option<&str>) -> Result<String> {
    match format {
        Some("json") => {
            let json_value = to_json_value(value);
            serde_json::to_string(&json_value).context("Failed to convert to JSON")
        }
        Some("json-pretty") => {
            let json_value = to_json_value(value);
            serde_json::to_string_pretty(&json_value).context("Failed to convert to JSON")
        }
        Some("toml") => toml::to_string(value).context("Failed to serialize TOML"),
        Some("toml-pretty") => toml::to_string_pretty(value).context("Failed to serialize TOML"),
        _ => Ok(value.to_string()),
    }
}

/// Convert TOML value to JSON value
pub fn to_json_value(toml_value: &Value) -> serde_json::Value {
    match toml_value {
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Integer(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Datetime(dt) => serde_json::Value::String(dt.to_string()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(to_json_value).collect()),
        Value::Table(table) => {
            let mut map = serde_json::Map::new();
            for (k, v) in table {
                map.insert(k.clone(), to_json_value(v));
            }
            serde_json::Value::Object(map)
        }
    }
}

/// Strip quotes from a string value
pub fn strip_quotes(value: &str) -> String {
    value.trim_matches('"').to_string()
}

/// Check if a value is a quoted string
pub fn is_quoted_string(value: &str) -> bool {
    value.starts_with('"') && value.ends_with('"')
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_parse_value_with_type() {
        // 字符串
        let value = parse_value("hello", Some("string")).unwrap();
        assert_eq!(value, Value::String("hello".to_string()));

        // 整数
        let value = parse_value("42", Some("integer")).unwrap();
        assert_eq!(value, Value::Integer(42));

        // 浮点数
        let value = parse_value("3.14", Some("float")).unwrap();
        assert_eq!(value, Value::Float(3.14));

        // 布尔值
        let value = parse_value("true", Some("boolean")).unwrap();
        assert_eq!(value, Value::Boolean(true));
        let value = parse_value("false", Some("boolean")).unwrap();
        assert_eq!(value, Value::Boolean(false));

        // 无效布尔值
        assert!(parse_value("maybe", Some("boolean")).is_err());

        // null
        let value = parse_value("", Some("null")).unwrap();
        assert_eq!(value, Value::String("".to_string()));

        // 未知类型
        assert!(parse_value("test", Some("unknown")).is_err());
    }

    #[test]
    fn test_auto_parse_value() {
        // 自动检测整数
        let value = auto_parse_value("123").unwrap();
        assert_eq!(value, Value::Integer(123));

        // 自动检测浮点数
        let value = auto_parse_value("123.45").unwrap();
        assert_eq!(value, Value::Float(123.45));

        // 自动检测布尔值
        let value = auto_parse_value("true").unwrap();
        assert_eq!(value, Value::Boolean(true));
        let value = auto_parse_value("false").unwrap();
        assert_eq!(value, Value::Boolean(false));

        // 自动检测字符串
        let value = auto_parse_value("hello world").unwrap();
        assert_eq!(value, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_format_value() {
        let value = Value::String("hello".to_string());

        // raw 格式
        let result = format_value(&value, None).unwrap();
        assert_eq!(result, "\"hello\"");

        // json 格式
        let result = format_value(&value, Some("json")).unwrap();
        assert_eq!(result, "\"hello\"");

        // json-pretty 格式
        let result = format_value(&value, Some("json-pretty")).unwrap();
        assert!(result.contains("\"hello\""));
    }

    #[test]
    fn test_to_json_value() {
        // 字符串
        let toml_value = Value::String("test".to_string());
        let json_value = to_json_value(&toml_value);
        assert_eq!(json_value, serde_json::json!("test"));

        // 整数
        let toml_value = Value::Integer(42);
        let json_value = to_json_value(&toml_value);
        assert_eq!(json_value, serde_json::json!(42));

        // 布尔值
        let toml_value = Value::Boolean(true);
        let json_value = to_json_value(&toml_value);
        assert_eq!(json_value, serde_json::json!(true));

        // 数组
        let toml_value = Value::Array(vec![Value::String("a".to_string()), Value::Integer(1)]);
        let json_value = to_json_value(&toml_value);
        assert_eq!(json_value, serde_json::json!(["a", 1]));
    }

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_quotes("\"hello\""), "hello");
        assert_eq!(strip_quotes("hello"), "hello");
        assert_eq!(strip_quotes("\"\""), "");
        assert_eq!(strip_quotes("'hello'"), "'hello'"); // 只移除双引号
    }

    #[test]
    fn test_is_quoted_string() {
        assert!(is_quoted_string("\"hello\""));
        assert!(!is_quoted_string("hello"));
        assert!(!is_quoted_string("'hello'"));
        assert!(is_quoted_string("\"\""));
    }
}
