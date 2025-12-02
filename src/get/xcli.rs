//! CLI definitions and handling for get command

use anyhow::Result;
use clap::{Arg, Command};

use super::core::*;
use super::types::ExtractConfig;

/// Define the get command CLI structure
pub fn get_command() -> Command {
    Command::new("get")
        .about("Extract values from TOML files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("TOML file path")
                .default_value("Cargo.toml"),
        )
        .arg(
            Arg::new("field")
                .short('k')
                .long("field")
                .value_name("FIELD")
                .help("Dot-separated field path (e.g., package.name, authors[0], bin[1].name)")
                .required_unless_present_any([
                    "multiple", "package-name", "package-version", "dependencies",
                    "authors", "keywords", "categories", "array", "array-length", "array-element"
                ]),
        )
        .arg(
            Arg::new("multiple")
                .short('m')
                .long("multiple")
                .value_name("FIELDS")
                .action(clap::ArgAction::Append)
                .help("Extract multiple fields (can be used multiple times)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FORMAT")
                .help("Output format (raw, json, json-pretty)")
                .default_value("raw"),
        )
        .arg(
            Arg::new("strip-quotes")
                .long("strip-quotes")
                .help("Strip surrounding quotes from string values")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("package-name")
                .long("package-name")
                .help("Extract package name (convenience flag for package.name)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("package-version")
                .long("package-version")
                .help("Extract package version (convenience flag for package.version)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dependencies")
                .long("dependencies")
                .help("Extract all dependencies as JSON")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("authors")
                .long("authors")
                .value_name("INDEX")
                .help("Extract package authors (use --authors for all, --authors 0 for first author)")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("keywords")
                .long("keywords")
                .value_name("INDEX")
                .help("Extract package keywords (use --keywords for all, --keywords 0 for first keyword)")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("categories")
                .long("categories")
                .value_name("INDEX")
                .help("Extract package categories (use --categories for all, --categories 0 for first category)")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("array")
                .long("array")
                .value_name("ARRAY_PATH")
                .help("Extract entire array (e.g., package.authors, workspace.members)"),
        )
        .arg(
            Arg::new("array-length")
                .long("array-length")
                .value_name("ARRAY_PATH")
                .help("Get array length (e.g., package.authors, workspace.members)"),
        )
        .arg(
            Arg::new("array-element")
                .long("array-element")
                .value_name("ARRAY_PATH")
                .help("Extract specific array element")
                .requires("array-index"),
        )
        .arg(
            Arg::new("array-index")
                .long("array-index")
                .value_name("INDEX")
                .help("Array index for --array-element (0-based)"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress error messages for missing fields")
                .action(clap::ArgAction::SetTrue),
        )
}

/// Handle get command logic
pub fn handle_get_command(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = matches.get_one::<String>("file").unwrap();
    let output_format = matches.get_one::<String>("output").unwrap();
    let strip_quotes = matches.get_flag("strip-quotes");
    let quiet = matches.get_flag("quiet");

    // Handle array operations
    if let Some(array_path) = matches.get_one::<String>("array") {
        match extract_array(file_path, array_path, Some(output_format)) {
            Ok(result) => println!("{}", result),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if let Some(array_path) = matches.get_one::<String>("array-length") {
        match extract_array_length(file_path, array_path) {
            Ok(length) => println!("{}", length),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if let Some(array_path) = matches.get_one::<String>("array-element") {
        if let Some(index_str) = matches.get_one::<String>("array-index") {
            match index_str.parse::<usize>() {
                Ok(index) => {
                    match extract_array_element(file_path, array_path, index, strip_quotes) {
                        Ok(element) => println!("{}", element),
                        Err(e) if !quiet => eprintln!("Error: {}", e),
                        _ => {}
                    }
                }
                Err(_) if !quiet => eprintln!("Error: Invalid array index: {}", index_str),
                _ => {}
            }
        }
        return Ok(());
    }

    // Handle convenience flags for arrays
    if matches.contains_id("authors") {
        let index = matches
            .get_one::<String>("authors")
            .and_then(|s| s.parse::<usize>().ok());

        match get_package_authors(Some(file_path), index, strip_quotes) {
            Ok(authors) => println!("{}", authors),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if matches.contains_id("keywords") {
        let index = matches
            .get_one::<String>("keywords")
            .and_then(|s| s.parse::<usize>().ok());

        match get_package_keywords(Some(file_path), index, strip_quotes) {
            Ok(keywords) => println!("{}", keywords),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if matches.contains_id("categories") {
        let index = matches
            .get_one::<String>("categories")
            .and_then(|s| s.parse::<usize>().ok());

        match get_package_categories(Some(file_path), index, strip_quotes) {
            Ok(categories) => println!("{}", categories),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    // Handle convenience flags
    if matches.get_flag("package-name") {
        match get_package_name(Some(file_path)) {
            Ok(name) => println!("{}", name),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if matches.get_flag("package-version") {
        match get_package_version(Some(file_path)) {
            Ok(version) => println!("{}", version),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    if matches.get_flag("dependencies") {
        match get_dependencies(Some(file_path)) {
            Ok(deps) => {
                let json = serde_json::to_string_pretty(&deps)?;
                println!("{}", json);
            }
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
        return Ok(());
    }

    // Handle multiple fields extraction
    if let Some(field_paths) = matches.get_many::<String>("multiple") {
        let field_paths: Vec<String> = field_paths.cloned().collect();
        match extract_multiple_fields(file_path, &field_paths, strip_quotes) {
            Ok(result) => {
                if output_format == "json" {
                    println!("{}", result.to_json()?);
                } else if output_format == "json-pretty" {
                    println!("{}", result.to_json_pretty()?);
                } else {
                    for (field_path, value) in result.fields {
                        println!("{}: {}", field_path, value);
                    }
                }
            }
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
    } else if let Some(field_path) = matches.get_one::<String>("field") {
        // Single field extraction
        let config = ExtractConfig {
            file_path: file_path.to_string(),
            field_path: field_path.to_string(),
            output_format: Some(output_format.to_string()),
            strip_quotes,
        };

        match extract_field(&config) {
            Ok(result) => println!("{}", result),
            Err(e) if !quiet => eprintln!("Error: {}", e),
            _ => {}
        }
    }

    Ok(())
}
