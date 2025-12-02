//! Core implementation for setting TOML fields

use anyhow::{Context, Result};
use std::fs;
use toml::{Value as TomlValue, Table};

use super::types::SetConfig;
use super::utils::{parse_value_with_type, split_field_path};
use crate::error::TomlExtractError;

/// Set a field in TOML file and return updated content
pub fn set_field(config: &SetConfig) -> Result<String> {
    // Read file content
    let content = fs::read_to_string(&config.file_path)
        .with_context(|| format!("Failed to read file: {}", config.file_path))?;

    // Parse TOML
    let mut toml_value: TomlValue = toml::from_str(&content)
        .map_err(|e| TomlExtractError::InvalidToml {
            file: config.file_path.clone(),
            error: e.to_string(),
        })?;

    // Split field path
    let parts = split_field_path(&config.field_path)?;

    // Set nested value
    set_nested_value(&mut toml_value, &parts, config.value.as_str(), config.value_type.as_deref(), config.create_missing)?;

    // Convert back to TOML string
    let updated_content = toml::to_string_pretty(&toml_value)?;
    Ok(updated_content)
}

/// Recursively set nested value in TOML structure
fn set_nested_value(
    current: &mut TomlValue,
    parts: &[String],
    value: &str,
    value_type: Option<&str>,
    create_missing: bool,
) -> Result<(), TomlExtractError> {
    if parts.is_empty() {
        return Err(TomlExtractError::FieldNotFound("Empty path".to_string()));
    }

    let (first, rest) = parts.split_first().unwrap();

    // Handle array syntax (e.g., "arr[0]")
    if first.contains('[') {
        let bracket_start = first.find('[').ok_or_else(|| {
            TomlExtractError::InvalidArrayIndex(format!("Invalid array syntax: {}", first))
        })?;
        let array_name = &first[..bracket_start];
        let index_part = &first[bracket_start + 1..first.len() - 1];
        let index: usize = index_part
            .parse()
            .map_err(|_| TomlExtractError::InvalidArrayIndex(index_part.to_string()))?;

        // Ensure parent is a table
        let current_table = current.as_table_mut().ok_or_else(|| {
            TomlExtractError::NotATable(format!("Parent of {} is not a table", array_name))
        })?;

        // Get or create array
        let array = current_table
            .entry(array_name)
            .or_insert_with(|| TomlValue::Array(Vec::new()))
            .as_array_mut()
            .ok_or_else(|| TomlExtractError::NotAnArray(array_name.to_string()))?;

        // Ensure array has enough elements if creating missing
        if create_missing {
            while array.len() <= index {
                array.push(TomlValue::String("".to_string()));
            }
        }

        if rest.is_empty() {
            // Set array element value
            let array_len = array.len(); // 先获取长度

            let parsed_value = parse_value_with_type(value, value_type)?;
            array.get_mut(index)
                .ok_or_else(|| TomlExtractError::ArrayIndexOutOfBounds {
                    path: array_name.to_string(),
                    index,
                    length: array_len,
                })?
                .clone_from(&parsed_value);
        } else {
            // Recurse into nested structure
            let array_len = array.len(); // 先获取长度
            let elem = array.get_mut(index).ok_or_else(|| {
                TomlExtractError::ArrayIndexOutOfBounds {
                    path: array_name.to_string(),
                    index,
                    length: array_len,
                }
            })?;
            set_nested_value(elem, rest, value, value_type, create_missing)?;
        }
    } else {
        // Handle regular fields
        if rest.is_empty() {
            // Set final field value
            let parsed_value = parse_value_with_type(value, value_type)?;
            if let TomlValue::Table(table) = current {
                table.insert(first.clone(), parsed_value);
            } else if create_missing {
                // Create parent table if missing and allowed
                *current = TomlValue::Table(Table::new());
                current.as_table_mut().unwrap()
                    .insert(first.clone(), parsed_value);
            } else {
                return Err(TomlExtractError::NotATable(format!(
                    "Cannot set field {} on non-table value",
                    first
                )));
            }
        } else {
            // Recurse into child fields
            let next = if let Some(table) = current.as_table_mut() {
                table.entry(first.clone()).or_insert_with(|| TomlValue::Table(Table::new()))
            } else if create_missing {
                *current = TomlValue::Table(Table::new());
                current.as_table_mut().unwrap()
                    .entry(first.clone())
                    .or_insert_with(|| TomlValue::Table(Table::new()))
            } else {
                return Err(TomlExtractError::NotATable(first.clone()));
            };
            
            set_nested_value(next, rest, value, value_type, create_missing)?;
        }
    }

    Ok(())
}

/// Set field and save changes to file
pub fn set_field_and_save(config: &SetConfig) -> Result<()> {
    let updated_content = set_field(config)?;
    fs::write(&config.file_path, updated_content)
        .with_context(|| format!("Failed to write to file: {}", config.file_path))?;
    Ok(())
}