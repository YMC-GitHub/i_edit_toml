//! TOML field extraction functionality
//!
//! Provides utilities for extracting values from TOML files using field paths,
//! with support for nested structures, arrays, and Cargo.toml-specific convenience methods.

pub mod core;
pub mod types;
pub mod utils;
pub mod xcli;

// pub use core::*;
// pub use types::*;
// pub use utils::strip_quotes;

pub use core::*;
pub use types::*;
pub use utils::*;
pub use xcli::*;