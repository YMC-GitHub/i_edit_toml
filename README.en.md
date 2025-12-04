# i_edit_toml

[![Crates.io](https://img.shields.io/crates/v/i_edit_toml)](https://crates.io/crates/i_edit_toml)
[![Documentation](https://docs.rs/i_edit_toml/badge.svg)](https://docs.rs/i_edit_toml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg)](https://www.rust-lang.org)

A lightweight, high-performance tool for Editing TOML based on field paths.

## Features

- Read and modify TOML files via intuitive field paths
- Support for nested structures and array operations
- Type-aware numeric processing
- Usable as a standalone CLI tool or Rust library
- Convenience operations for Cargo.toml (extract package name, version, dependencies, etc.)

## Installation

### Install from crates.io

```bash
cargo install i_edit_toml
```

### Install from source

```bash
git clone https://github.com/ymc-github/i_edit_toml
cd i_edit_toml
cargo install --path .
```

```bash
# Install from GitHub
cargo install --git https://github.com/ymc-github/i_edit_toml

# Specify branch
cargo install --git https://github.com/ymc-github/i_edit_toml --branch main

# Specify tag
cargo install --git https://github.com/ymc-github/i_edit_toml --tag v0.3.0
```

### Install from docker hub
```bash
#  from docker.io
# docker pull docker.io/ymc-github/i_edit_toml:latest
docker pull ymc-github/i_edit_toml:latest

#  from ghcr.io
# ghcr.io/<owner>/<repo>:latest
docker pull ghcr.io/ymc-github/i_edit_toml:latest
```

## Usage

### As a Command Line Tool

#### Extract Fields (get command)

```bash
# Basic usage (extract package.name)
i_edit_toml get -f Cargo.toml -k package.name

# Extract package version
i_edit_toml get --package-version

# Extract all dependencies
i_edit_toml get --dependencies

# Extract array elements
i_edit_toml get --array package.authors
i_edit_toml get --array-element package.authors --array-index 0

# Extract array length
i_edit_toml get --array-length package.keywords

# Extract multiple fields
i_edit_toml get -m package.name -m package.version -m package.authors

# Output in JSON format
i_edit_toml get -k dependencies --output json-pretty
```

#### Set Fields (set command)

```bash
# Basic usage (set package.version)
i_edit_toml set -f Cargo.toml -k package.version -v "0.3.0" --in-place

# Set array element
i_edit_toml set -k package.authors[0] -v "New Author <author@example.com>" --in-place

# Create non-existent fields
i_edit_toml set -k package.description -v "A new description" --create-missing --in-place

# Specify value type
i_edit_toml set -k package.edition -v "2021" -t string --in-place
```

### As a Library

Add dependency to `Cargo.toml`:

```toml
[dependencies]
i_edit_toml = "0.3"
```

Use in code:

```rust
use i_edit_toml::{get, set, ExtractConfig, SetConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Extract field
    let get_config = ExtractConfig {
        file_path: "Cargo.toml".to_string(),
        field_path: "package.version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    let version = get::extract_field(&get_config)?;
    println!("Current version: {}", version);

    // Set field
    let set_config = SetConfig {
        file_path: "Cargo.toml".to_string(),
        field_path: "package.version".to_string(),
        value: "0.3.0".to_string(),
        value_type: None,
        create_missing: false,
    };
    set::set_field_and_save(&set_config)?;
    println!("Version updated successfully");

    Ok(())
}
```

### GitHub Actions Integration

Here's an example of using `i_edit_toml` in a GitHub Actions workflow to read and modify `Cargo.toml`:

```yaml
name: Use i_edit_toml in CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  toml-operations:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.60.0  # Minimum Rust version required by i_edit_toml

      - name: Install i_edit_toml
        run: cargo install i_edit_toml

      - name: Verify installation
        run: i_edit_toml --version

      - name: Read Cargo.toml metadata
        id: read_toml
        run: |
          # Extract package name and version
          PACKAGE_NAME=$(i_edit_toml get -f Cargo.toml -k package.name)
          PACKAGE_VERSION=$(i_edit_toml get -f Cargo.toml -k package.version)
          # Extract Rust version requirement
          RUSTC_VERSION=$(i_edit_toml get -f Cargo.toml -k package.rust-version)
          
          # Export as GitHub Actions environment variables
          echo "package_name=$PACKAGE_NAME" >> $GITHUB_OUTPUT
          echo "package_version=$PACKAGE_VERSION" >> $GITHUB_OUTPUT
          echo "rustc_version=$RUSTC_VERSION" >> $GITHUB_OUTPUT

      - name: Display extracted info
        run: |
          echo "Package name: ${{ steps.read_toml.outputs.package_name }}"
          echo "Package version: ${{ steps.read_toml.outputs.package_version }}"
          echo "Rustc version: ${{ steps.read_toml.outputs.rustc_version }}"

      - name: Modify TOML file (example)
        run: |
          # Create backup of Cargo.toml
          cp Cargo.toml Cargo.toml.bak
          
          # Update version number (add build metadata)
          i_edit_toml set -f Cargo.toml -k package.version \
            -v "${{ steps.read_toml.outputs.package_version }}+${{ github.sha }}" \
            --in-place
          
          # Add temporary build comment
          i_edit_toml set -f Cargo.toml -k package.metadata.build_comment \
            -v "Built in CI: ${{ github.run_id }}" \
            --create-missing \
            --in-place

      - name: Verify modifications
        run: |
          echo "Modified version: $(i_edit_toml get -f Cargo.toml -k package.version)"
          echo "Build comment: $(i_edit_toml get -f Cargo.toml -k package.metadata.build_comment)"

      - name: Restore original file (optional)
        if: always()
        run: mv Cargo.toml.bak Cargo.toml
```

## License

This project is dual-licensed under:
- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

You may choose either license to use this software.