
## 在本项目内开发时的使用方式
```powershell
# Extract package name from Cargo.toml
cargo run -- get --field package.name

# Extract from specific file
cargo run -- get --file config.toml --field database.host

# Extract multiple fields
cargo run -- get  --multiple package.name --multiple package.version

# JSON output
cargo run -- get  --field package.name --output json

# Strip quotes from string values
cargo run -- get  --field package.name --strip-quotes
cargo run -- get  --field package.version --strip-quotes
cargo run -- get  --field package.repository --strip-quotes
cargo run -- get --field "package.authors[0]" --strip-quotes
cargo run -- get --authors          # 所有作者
cargo run -- get --authors 0 --strip-quotes   # 第一个作者
cargo run -- get --keywords         # 所有关键词

cargo run -- get --field "bin[0].name" --strip-quotes

cargo run -- get --array "package.authors"
cargo run -- get --array-length "package.authors"
cargo run -- get --array-element "package.authors" --array-index 0

```

## 安装后的二进制使用方式

```bash
# Extract package name from Cargo.toml
toml_extract --field package.name

# Extract from specific file
toml_extract --file config.toml --field database.host

# Extract multiple fields
toml_extract --multiple package.name --multiple package.version

# JSON output
toml_extract --field package.name --output json

# Strip quotes from string values
toml_extract --field package.name --strip-quotes
```

```bash
# 基本字段提取
i_edit_toml get --field package.name
i_edit_toml get --field package.version --strip-quotes

# 数组访问
i_edit_toml get --field "authors[0]" --strip-quotes
i_edit_toml get --field "bin[1].name" --strip-quotes

# 便捷数组命令
i_edit_toml get --authors          # 所有作者
i_edit_toml get --authors 0        # 第一个作者
i_edit_toml get --keywords         # 所有关键词

# 数组操作
i_edit_toml get --array "package.authors"
i_edit_toml get --array-length "package.authors"
i_edit_toml get --array-element "package.authors" --array-index 0

# 多字段提取
i_edit_toml get --multiple package.name --multiple package.version

# JSON 输出
i_edit_toml get --dependencies --output json-pretty
```