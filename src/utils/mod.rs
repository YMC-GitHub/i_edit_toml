//! Utility functions for TOML operations.
//!
//! This module contains helper functions for parsing paths, values, and
//! formatting output.

mod path_parser;
mod value_parser;

// Re-export
pub use path_parser::{is_array_access, last_part, parent_path, parse_array_access, split_path};
pub use value_parser::{
    auto_parse_value, format_value, is_quoted_string, parse_value, strip_quotes, to_json_value,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parser() {
        assert!(is_array_access("array[0]"));
        assert!(!is_array_access("array"));

        let (name, index) = parse_array_access("array[0]").unwrap();
        assert_eq!(name, "array");
        assert_eq!(index, 0);
    }

    #[test]
    fn test_value_parser() {
        // 测试自动解析
        let int_value = parse_value("42", None).unwrap();
        assert!(int_value.is_integer());

        let float_value = parse_value("3.14", None).unwrap();
        assert!(float_value.is_float());

        let bool_value = parse_value("true", None).unwrap();
        assert!(bool_value.is_bool());

        let string_value = parse_value("hello", None).unwrap();
        assert!(string_value.is_str());
    }

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_quotes("\"hello\""), "hello");
        assert_eq!(strip_quotes("hello"), "hello");
        assert_eq!(strip_quotes("\"\""), "");
    }
}
