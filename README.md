# toml_path

[![Crates.io](https://img.shields.io/crates/v/toml_path)](https://crates.io/crates/toml_path)
[![Documentation](https://docs.rs/toml_path/badge.svg)](https://docs.rs/toml_path)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg)](https://www.rust-lang.org)

一个轻量级、高性能的、基于字段路径的 TOML 编辑工具。

## 功能特性

- 通过直观的字段路径读取和修改 TOML 文件
- 支持嵌套结构和数组操作
- 类型感知的数值处理
- 可作为独立 CLI 工具或 Rust 库使用
- 提供 Cargo.toml 特定的便捷操作（如提取包名、版本、依赖等）

## 安装

### 从 crates.io 安装

```bash
cargo install toml_path
```

### 从源码安装

```bash
git clone https://github.com/YeMiancheng/toml_path
cd toml_path
cargo install --path .
```

```bash
# 从 GitHub 安装
cargo install --git https://github.com/YeMiancheng/toml_path

# 指定分支
cargo install --git https://github.com/YeMiancheng/toml_path --branch main

# 指定标签
cargo install --git https://github.com/YeMiancheng/toml_path --tag v0.2.0
```

### 从 docker hub 安装
```bash
#  from docker.io
# docker pull docker.io/yemiancheng/toml_path:latest
docker pull yemiancheng/toml_path:latest

#  from ghcr.io
# ghcr.io/<owner>/<repo>:latest
docker pull ghcr.io/ymc-github/toml_path:latest
```

## 使用方法

### 作为命令行工具

#### 提取字段（get 命令）

```bash
# 基本使用（提取 package.name）
toml_path get -f Cargo.toml -k package.name

# 提取包版本
toml_path get --package-version

# 提取所有依赖
toml_path get --dependencies

# 提取数组元素
toml_path get --array package.authors
toml_path get --array-element package.authors --array-index 0

# 提取数组长度
toml_path get --array-length package.keywords

# 提取多个字段
toml_path get -m package.name -m package.version -m package.authors

# 输出为 JSON 格式
toml_path get -k dependencies --output json-pretty
```

#### 设置字段（set 命令）

```bash
# 基本使用（设置 package.version）
toml_path set -f Cargo.toml -k package.version -v "0.3.0" --in-place

# 设置数组元素
toml_path set -k package.authors[0] -v "New Author <author@example.com>" --in-place

# 创建不存在的字段
toml_path set -k package.description -v "A new description" --create-missing --in-place

# 指定值类型
toml_path set -k package.edition -v "2021" -t string --in-place
```

### 作为库使用

添加依赖到 `Cargo.toml`：

```toml
[dependencies]
toml_path = "0.2"
```

在代码中使用：

```rust
use toml_path::{get, set, ExtractConfig, SetConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 提取字段
    let get_config = ExtractConfig {
        file_path: "Cargo.toml".to_string(),
        field_path: "package.version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    let version = get::extract_field(&get_config)?;
    println!("Current version: {}", version);

    // 设置字段
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

## 许可证

MIT OR Apache-2.0