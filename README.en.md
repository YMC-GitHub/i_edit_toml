# toml_path

[![Crates.io](https://img.shields.io/crates/v/toml_path)](https://crates.io/crates/toml_path)
[![Documentation](https://docs.rs/toml_path/badge.svg)](https://docs.rs/toml_path)
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
cargo install toml_path
```

### Install from source

```bash
git clone https://github.com/YeMiancheng/toml_path
cd toml_path
cargo install --path .
```

```bash
# Install from GitHub
cargo install --git https://github.com/YeMiancheng/toml_path

# Specify branch
cargo install --git https://github.com/YeMiancheng/toml_path --branch main

# Specify tag
cargo install --git https://github.com/YeMiancheng/toml_path --tag v0.2.0
```

### Install from docker hub
```bash
#  from docker.io
# docker pull docker.io/yemiancheng/toml_path:latest
docker pull yemiancheng/toml_path:latest

#  from ghcr.io
# ghcr.io/<owner>/<repo>:latest
docker pull ghcr.io/ymc-github/toml_path:latest
```

## Usage

### As a Command Line Tool

#### Extract Fields (get command)

```bash
# Basic usage (extract package.name)
toml_path get -f Cargo.toml -k package.name

# Extract package version
toml_path get --package-version

# Extract all dependencies
toml_path get --dependencies

# Extract array elements
toml_path get --array package.authors
toml_path get --array-element package.authors --array-index 0

# Extract array length
toml_path get --array-length package.keywords

# Extract multiple fields
toml_path get -m package.name -m package.version -m package.authors

# Output in JSON format
toml_path get -k dependencies --output json-pretty
```

#### Set Fields (set command)

```bash
# Basic usage (set package.version)
toml_path set -f Cargo.toml -k package.version -v "0.3.0" --in-place

# Set array element
toml_path set -k package.authors[0] -v "New Author <author@example.com>" --in-place

# Create non-existent fields
toml_path set -k package.description -v "A new description" --create-missing --in-place

# Specify value type
toml_path set -k packageedition -v "2021" -t string --in-place
```

### As a Library

Add dependency to `Cargo.toml`:

```toml
[dependencies]
toml_path = "0.2"
```

Use in code:

```rust
use toml_path::{get, set, ExtractConfig, SetConfig};

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

Here's an example of using `toml_path` in a GitHub Actions workflow to read and modify `Cargo.toml`:

```yaml
name: Use toml_path in CI

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
          toolchain: 1.60.0  # Minimum Rust version required by toml_path

      - name: Install toml_path
        run: cargo install toml_path

      - name: Verify installation
        run: toml_path --version

      - name: Read Cargo.toml metadata
        id: read_toml
        run: |
          # Extract package name and version
          PACKAGE_NAME=$(toml_path get -f Cargo.toml -k package.name)
          PACKAGE_VERSION=$(toml_path get -f Cargo.toml -k package.version)
          # Extract Rust version requirement
          RUSTC_VERSION=$(toml_path get -f Cargo.toml -k package.rust-version)
          
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
          toml_path set -f Cargo.toml -k package.version \
            -v "${{ steps.read_toml.outputs.package_version }}+${{ github.sha }}" \
            --in-place
          
          # Add temporary build comment
          toml_path set -f Cargo.toml -k package.metadata.build_comment \
            -v "Built in CI: ${{ github.run_id }}" \
            --create-missing \
            --in-place

      - name: Verify modifications
        run: |
          echo "Modified version: $(toml_path get -f Cargo.toml -k package.version)"
          echo "Build comment: $(toml_path get -f Cargo.toml -k package.metadata.build_comment)"

      - name: Restore original file (optional)
        if: always()
        run: mv Cargo.toml.bak Cargo.toml
```

## License

This project is dual-licensed under:
- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

You may choose either license to use this software.