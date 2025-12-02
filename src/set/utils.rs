//! Utility functions for set operations

use crate::error::TomlExtractError;
use toml::Value as TomlValue;

/// Split field path into segments (handles array syntax like "arr[0]")
pub fn split_field_path(field_path: &str) -> Result<Vec<String>, TomlExtractError> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_array = false;

    for c in field_path.chars() {
        match c {
            '.' if !in_array => {
                if current.is_empty() {
                    return Err(TomlExtractError::InvalidFieldPath(
                        "Empty path segment".to_string(),
                    ));
                }
                parts.push(current);
                current = String::new();
            }
            '[' => {
                in_array = true;
                current.push(c);
            }
            ']' => {
                in_array = false;
                current.push(c);
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    } else {
        return Err(TomlExtractError::InvalidFieldPath(
            "Field path cannot end with a dot".to_string(),
        ));
    }

    Ok(parts)
}

/// Parse value with optional type hint
pub fn parse_value_with_type(value: &str, value_type: Option<&str>) -> Result<TomlValue, TomlExtractError> {
    match value_type {
        Some("string") => Ok(TomlValue::String(value.to_string())),
        Some("integer") => value
            .parse::<i64>()
            .map(TomlValue::Integer)
            .map_err(|_| TomlExtractError::InvalidValueType(format!("{} is not a valid integer", value))),
        Some("float") => value
            .parse::<f64>()
            .map(TomlValue::Float)
            .map_err(|_| TomlExtractError::InvalidValueType(format!("{} is not a valid float", value))),
        Some("boolean") => match value.to_lowercase().as_str() {
            "true" => Ok(TomlValue::Boolean(true)),
            "false" => Ok(TomlValue::Boolean(false)),
            _ => Err(TomlExtractError::InvalidValueType(format!(
                "{} is not a valid boolean",
                value
            ))),
        },
        _ => {
            // Auto-detect type
            if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
                Ok(TomlValue::Boolean(value.eq_ignore_ascii_case("true")))
            } else if let Ok(num) = value.parse::<i64>() {
                Ok(TomlValue::Integer(num))
            } else if let Ok(num) = value.parse::<f64>() {
                Ok(TomlValue::Float(num))
            } else {
                Ok(TomlValue::String(value.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_field_path() {
        assert_eq!(
            split_field_path("package.name").unwrap(),
            vec!["package".to_string(), "name".to_string()]
        );
        
        assert_eq!(
            split_field_path("dependencies.serde.features[0]").unwrap(),
            vec![
                "dependencies".to_string(),
                "serde".to_string(),
                "features[0]".to_string()
            ]
        );
    }

    #[test]
    fn test_parse_value_with_type() {
        assert!(matches!(parse_value_with_type("42", Some("integer")).unwrap(), TomlValue::Integer(42)));
        assert!(matches!(parse_value_with_type("3.14", Some("float")).unwrap(), TomlValue::Float(3.14)));
        assert!(matches!(parse_value_with_type("true", Some("boolean")).unwrap(), TomlValue::Boolean(true)));
        assert!(matches!(parse_value_with_type("text", Some("string")).unwrap(), TomlValue::String(s) if s == "text"));
    }

    #[test]
    fn test_parse_value_auto() {
        assert!(matches!(parse_value_with_type("100", None).unwrap(), TomlValue::Integer(100)));
        assert!(matches!(parse_value_with_type("false", None).unwrap(), TomlValue::Boolean(false)));
        assert!(matches!(parse_value_with_type("hello", None).unwrap(), TomlValue::String(s) if s == "hello"));
    }
}