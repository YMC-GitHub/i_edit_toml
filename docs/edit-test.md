使用 `test.toml` 文件进行测试的完整示例，涵盖 `get` 和 `set` 命令的主要功能：


### 1. 准备测试文件
创建 `test.toml` 并写入以下内容：
```toml
[package]
name = "test_app"
version = "1.0.0"
authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
edition = "2021"
publish = false

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1.0"

[dev-dependencies]
test-utils = "0.5.2"

[array_demo]
numbers = [10, 20, 30, 40]
nested = [
    { id = 1, name = "first" },
    { id = 2, name = "second" }
]
```


### 2. `get` 命令测试示例

#### 基础字段提取
```bash
# 获取 package.name（带引号）
cargo run -- get -f test.toml --field package.name

# 获取 version 并去引号
cargo run -- get -f test.toml --field package.version --strip-quotes

# 获取 publish 布尔值
cargo run -- get -f test.toml --field package.publish
```

#### 数组操作
```bash
# 获取所有作者
cargo run -- get -f test.toml --array package.authors

# 获取第二个作者（索引1）并去引号
cargo run -- get -f test.toml --array-element package.authors --array-index 1 --strip-quotes

# 获取 numbers 数组长度
cargo run -- get -f test.toml --array-length array_demo.numbers

# 获取 nested 数组的第一个元素
cargo run -- get -f test.toml --field "array_demo.nested[0]"
```

#### 依赖项提取
```bash
# 获取 serde 版本
cargo run -- get -f test.toml --field dependencies.serde.version --strip-quotes

# 获取所有依赖（JSON 格式）
cargo run -- get -f test.toml --dependencies -o json-pretty
```

#### 多字段提取
```bash
# 同时提取 name 和 version
cargo run -- get -f test.toml -m package.name -m package.version --strip-quotes
```


### 3. `set` 命令测试示例

#### 修改基础字段
```bash
# 修改版本号（输出修改后的内容，不保存到文件）
cargo run -- set -f test.toml --field package.version --value "1.1.0"

# 原地修改版本号（直接修改文件）
cargo run -- set -f test.toml --field package.version --value "1.1.0" -i
```

#### 修改数组元素
```bash
# 修改第一个作者
cargo run -- set -f test.toml --field "package.authors[0]" --value "Alice Smith <alice.smith@example.com>" -i

# 修改 nested 数组的第二个元素的 name
cargo run -- set -f test.toml --field "array_demo.nested[1].name" --value "second item" -i
```

#### 添加新字段
```bash
# 添加新的依赖（需要 --create-missing 创建不存在的字段）
cargo run -- set -f test.toml --field dependencies.anyhow --value "1.0" --create-missing -i

# 添加布尔值字段
cargo run -- set -f test.toml --field package.optional --value "true" --type boolean -i
```


### 4. 验证修改结果
使用 `get` 命令检查修改是否生效：
```bash
# 检查版本号是否已更新
cargo run -- get -f test.toml --field package.version --strip-quotes

# 检查新添加的依赖
cargo run -- get -f test.toml --field dependencies.anyhow --strip-quotes
```


通过以上测试，可以验证工具对 TOML 文件的读取、修改、数组操作、嵌套结构访问等核心功能。所有命令均针对 `test.toml` 操作，不会影响项目的 `Cargo.toml`。