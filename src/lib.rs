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

pub mod extract_preset; // 5
pub use extract_preset::*;
