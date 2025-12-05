## 其他项目的使用方式

### 方式一：作为命令行工具使用
```bash
# 安装
cargo install code_packager

# 使用
code_packager -i ./src -o packaged_code.txt
```

### 方式二：作为库依赖使用
在项目的 `Cargo.toml` 中添加：
```toml
[dependencies]
code_packager = "0.1"
```

使用示例：
```rust
use code_packager::{package_code, PackagerConfig};

fn main() {
    let config = PackagerConfig {
        input_dir: "src".to_string(),
        output_file: "packaged.txt".to_string(),
        extra_files: vec!["Cargo.toml".to_string()],
        ignore_patterns: vec!["target/*".to_string()],
    };
    
    package_code(&config).unwrap();
}
```

### 方式三：作为 Cargo 子命令
安装后，在项目目录中直接运行：
```bash
cargo code_packager
```


### 使用案例
```
# 基本使用
code_packager

# 指定输入输出
code_packager -i ./src -o src_output.md

# 添加额外文件
code_packager -i ./src -a "Cargo.toml" -a "README.md" -o src_output.md

# 忽略文件模式
code_packager -i ./src -a "Cargo.toml" -a "README.md"  --ignore "src/main*"  -o src_lib.md
```