// src/set/xcli.rs
use clap::{Arg, Command};
use anyhow::{Result, Context};
// use crate::{SetConfig, set_field, set_field_and_save};
use crate::{SetConfig, set::core::{set_field, set_field_and_save}};

// use std::fs;

/// 定义 set 命令的 CLI 结构
pub fn cli() -> Command {
    Command::new("set")
        .about("Set values in TOML files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("TOML file path")
                .default_value("Cargo.toml")
                // .required(true),
        )
        .arg(
            Arg::new("field")
                .short('k')
                .long("field")
                .value_name("FIELD")
                .help("Dot-separated field path (e.g., package.version, dependencies.serde)")
                .required(true),
        )
        .arg(
            Arg::new("value")
                .short('v')
                .long("value")
                .value_name("VALUE")
                .help("Value to set for the field")
                .required(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Value type (string, integer, float, boolean, auto)")
                .default_value("auto"),
        )
        .arg(
            Arg::new("create-missing")
                .long("create-missing")
                .help("Create missing parent fields if they don't exist")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("in-place")
                .short('i')
                .long("in-place")
                .help("Modify the file in place")
                .action(clap::ArgAction::SetTrue),
        )
}

/// 实现 set 命令的处理逻辑
pub fn handle_set_command(matches: &clap::ArgMatches) -> Result<()> {
    // 解析参数
    let file_path = matches.get_one::<String>("file")
        .context("File path is required")?;
    let field_path = matches.get_one::<String>("field")
        .context("Field path is required")?;
    let value = matches.get_one::<String>("value")
        .context("Value is required")?;
    let value_type = matches.get_one::<String>("type")
        .context("Value type is required")?;
    let create_missing = matches.get_flag("create-missing");
    let in_place = matches.get_flag("in-place");

    // 处理值类型（自动推断或指定类型）
    let value_type = if value_type == "auto" {
        None
    } else {
        Some(value_type.as_str())
    };

    // 构建配置
    let config = SetConfig {
        file_path: file_path.to_string(),
        field_path: field_path.to_string(),
        value: value.to_string(),
        value_type: value_type.map(|s| s.to_string()),
        create_missing,
    };

    // 执行设置操作
    if in_place {
        // 原地修改文件
        set_field_and_save(&config)?;
        println!("✅ Field '{}' set to '{}' in {}", field_path, value, file_path);
    } else {
        // 输出修改后的内容（不修改原文件）
        let result = set_field(&config)?;
        println!("{}", result);
    }

    Ok(())
}