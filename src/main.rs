//! CLI entry point for toml_code - a TOML field extraction and manipulation tool

use anyhow::{Context, Result};
use clap::Command;
use toml_code::{get::xcli::get_command, set::xcli::cli as set_command};

fn main() -> Result<()> {
    // Define main CLI structure
    let mut app = Command::new("toml_code")
        .version(env!("CARGO_PKG_VERSION"))
        .author("YeMiancheng <ymc.github@gmail.com>")
        .about("A lightweight, high-performance TOML field extraction and manipulation tool")
        .subcommand(get_command().name("get"))
        .subcommand(set_command().name("set"));

    // Parse CLI arguments
    let matches = app.clone().get_matches();

    // Dispatch to appropriate subcommand handler
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            toml_code::get::xcli::handle_get_command(sub_matches)
                .context("Failed to execute get command")?;
        }
        Some(("set", sub_matches)) => {
            toml_code::set::xcli::handle_set_command(sub_matches)
                .context("Failed to execute set command")?;
        }
        _ => {
            // Print help if no subcommand is provided
            println!("{}", app.render_help());
        }
    }

    Ok(())
}