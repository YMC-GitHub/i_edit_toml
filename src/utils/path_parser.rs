use crate::error::TomlError;

/// Parse a TOML path part to check if it's array access
pub fn is_array_access(part: &str) -> bool {
    part.contains('[') && part.ends_with(']')
}

/// Parse array access syntax (e.g., "array[0]") into (array_name, index)
pub fn parse_array_access(part: &str) -> Result<(&str, usize), TomlError> {
    if !is_array_access(part) {
        return Err(TomlError::PathParseError(format!(
            "Not an array access syntax: {}",
            part
        )));
    }

    let bracket_start = part
        .find('[')
        .ok_or_else(|| TomlError::PathParseError("Missing '[' in array access".to_string()))?;

    let array_name = &part[..bracket_start];

    if array_name.is_empty() {
        return Err(TomlError::PathParseError("Empty array name".to_string()));
    }

    let index_part = &part[bracket_start + 1..part.len() - 1];

    let index = index_part
        .parse::<usize>()
        .map_err(|_| TomlError::InvalidArrayIndex(index_part.to_string()))?;

    Ok((array_name, index))
}

/// Split a TOML path into parts (e.g., "package.name[0]" -> ["package", "name[0]"])
pub fn split_path(path: &str) -> Vec<&str> {
    path.split('.').collect()
}

/// Get parent path (e.g., "package.name[0]" -> "package")
pub fn parent_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = split_path(path);
    if parts.len() > 1 {
        Some(parts[..parts.len() - 1].join("."))
    } else {
        None
    }
}

/// Get last part of path (e.g., "package.name[0]" -> "name[0]")
pub fn last_part(path: &str) -> Option<&str> {
    split_path(path).last().copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::error::TomlError;

    #[test]
    fn test_is_array_access() {
        assert!(is_array_access("array[0]"));
        assert!(is_array_access("array[123]"));
        assert!(!is_array_access("array"));
        assert!(!is_array_access("array["));
        assert!(!is_array_access("array]"));
    }

    #[test]
    fn test_parse_array_access() {
        let (name, index) = parse_array_access("array[0]").unwrap();
        assert_eq!(name, "array");
        assert_eq!(index, 0);

        let (name, index) = parse_array_access("test[42]").unwrap();
        assert_eq!(name, "test");
        assert_eq!(index, 42);

        // 错误情况
        assert!(parse_array_access("array").is_err());
        assert!(parse_array_access("[0]").is_err()); // 空数组名
        assert!(parse_array_access("array[abc]").is_err()); // 非数字索引
        assert!(parse_array_access("array[]").is_err()); // 空索引
    }

    #[test]
    fn test_split_path() {
        assert_eq!(split_path("package.name"), vec!["package", "name"]);
        assert_eq!(
            split_path("package.authors[0]"),
            vec!["package", "authors[0]"]
        );
        assert_eq!(split_path(""), vec![""]);
    }

    #[test]
    fn test_parent_path() {
        assert_eq!(parent_path("package.name"), Some("package".to_string()));
        assert_eq!(
            parent_path("package.authors[0]"),
            Some("package".to_string())
        );
        assert_eq!(parent_path("name"), None);
        assert_eq!(parent_path(""), None);
    }

    #[test]
    fn test_last_part() {
        assert_eq!(last_part("package.name"), Some("name"));
        assert_eq!(last_part("package.authors[0]"), Some("authors[0]"));
        assert_eq!(last_part(""), Some(""));
    }
}
