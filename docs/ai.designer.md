
## 命令 | 创建 crate | 发布 crate | 使用 crate
- 如何将下列代码作为 crate，发布到crate.io，其他项目如何使用？作为命令行使用？作为cargo 命令？

- 在 Cargo.toml 中 package 键下的 name bin 键下的 name lib 键下的 name 是否应该统一？为什么有的 kebab-case 有的使用 snake_case?
-  kebab-case vs snake_case
- 统一使用 snake_case 会不会 更好

- 检查是否符合crate.io规范，包含keywords 个数（最多5个），categories ,文档是否规范
- 检查src下的源码是否符合crate.io规范

- 输出所需要修改文件的完整代码
- 控制台中的输出信息 使用英文

- 当前代码是否各主流操作系统(window,macos,linux)都可编译和运行，是否需要根据注释进行跨平台编译控制

## 命令 | 功能
- 如何访问数组的内容

- 我的想法：
    - 获取非数组的内容
        - toml_extract --field "package.name" --strip-quotes

    - 获取数组的第1个下标的内容
        - toml_extract --field "package.keywords[0]" --strip-quotes



    - 支持 "." 语法 和 ”[]“
        - 如果 ”[]“存在，如果里面的是数字，则表明数组的下标。
            - 比如 "package.keywords[0]"

        - 如果是非数字，则表示key,只是使用了类似数组的表达方式。
            - 比如 "package[name]" 等价于 "package.name"

    - 支持 --field-separator
        - 用于设置 field 字段的分隔符。
        - 默认为 "."
        - 方便有些特殊情景使用

    - 设置非数组的内容
        - toml_extract set --field "package.name" --value "my_package"
        - toml_extract set --field "package.version" --value "1.0.0"

    - 设置数组的第1个下标的内容
        - toml_extract set --field "package.keywords[0]" --value "toml"

    - 设置数组应包含哪些内容和应排除哪些内容
        - toml_extract stra/set --field "package.keywords" --include-values "toml" --exclude-values "rust,go" --value-separator ","

- 如果集成设置字段/数组取值 ？需要加入其他额外的crate吗

## 托管 | github | docker | crates.io
- github workflow 发布到 release
- 如何移除GitHub Workflow中的硬编码
- 保持原有输入参数，支持调试吗？在调试模式下不发布
- 从 Cargo.toml 读取项目名称的实现是不是不严谨 `BINARY_NAME=$(grep '^name =' Cargo.toml | head -1 | cut -d'"' -f2)`
- 使用rust实现一个从toml中获取指定字段取值的lib.rs 和 main.rs，发布到crate.io 和 github release, docker 等供自己和他人使用不是更好

## 安装 | 本地安装 | 临时安装 | 国内网络
- 如何临时使用官方crate.io ？ `cargo install code_packager --registry crates-io`
```powershell
cargo search code_packager --registry crates-io

# cargo install code_packager --version 0.1.0 --registry crates-io
# cargo install code_packager --version 0.1.0 --registry crates-io --index https://github.com/rust-lang/crates.io-index


# 从 GitHub 安装
cargo install --git https://github.com/ymc-github/code_packager

# 如果指定分支
cargo install --git https://github.com/ymc-github/code_packager --branch main

# 如果指定标签
cargo install --git https://github.com/ymc-github/code_packager --tag v0.1.0

# cargo install --path .
```

## 打包 | pre.ai
```powershell
# 添加额外文件
code_packager -i ./src -a "Cargo.toml" -a "README.md" -o src_output.md

# 忽略文件模式
code_packager -i ./src -a "Cargo.toml" -a "README.md"  --ignore "src/main*"  -o src_lib.md


code_packager -i ./src -a "Cargo.toml" -a "README.md" -o src_output.txt

code_packager -i ./src -a "Cargo.toml" -a "README.md"  --ignore "src/main*"  -o src_lib.txt

# Cargo.toml + src/lib.rs + src/main.rs
code_packager -i ./docs -a "Cargo.toml" -a "src/lib.rs" -a "src/main.rs"  --ignore "./docs/*" -o src_output.md


# Cargo.toml + src/lib.rs + src/main.rs
code_packager -i ./docs -a "Cargo.toml" -a "src/lib.rs" -a "src/main.rs"  --ignore "./docs/*.md" -o src_m0.md

code_packager -i ./docs -a "src/error.rs" -a "src/types.rs" -a "src/utils.rs" -a "src/extract*.rs"  --ignore "./docs/*" -o src_m1.md

code_packager -i ./docs -a "src/*_next.rs" --ignore "./docs/*" -o src_m2.md

code_packager -i ./docs -a "Cargo.toml" -a "src/get/*.rs" -a "src/set/*.rs"  --ignore "./docs/*.md" -o src_m3.md

# Cargo.toml + src/get/*.rs + src/set/*.rs -> src_m3.md
# src/lib.rs + src/main.rs + src/error.rs -> src_m4.md
code_packager -i ./docs -a "Cargo.toml" -a "src/get/*.rs" -a "src/set/*.rs"  --ignore "./docs/*.md" -o src_m3.md
code_packager -i ./docs -a "src/lib.rs" -a "src/main.rs" -a "src/error.rs"  --ignore "./docs/*.md" -o src_m4.md
code_packager -i ./docs -a "README.md"   --ignore "./docs/*.md" -o src_m5.md

# code_packager -i ./.github/workflows - --ignore "./.github/workflows/clean*" -o src_m6.md

code_packager -i ./docs -a "./.github/workflows/docker-publish*" -a "./.github/workflows/ghcr-publish*"   --ignore "./docs/*.md" -o src_m6.md


code_packager -i ./docs -a "./.github/workflows/*release*" --ignore "./docs/*.md" -o src_m7.md

```


## 拆分 | 1
- 是否需要提取 get 和 set 两部分？两者如果如果有公共代码的话，是否可以提取出来？或者有更合适的代码组织方式？
- trait ? struct ? enum ? pure function ?
- 通俗易懂，易读性强，可维护性高，
- 同时修复以下错误

## 拆分 | 2 
- 是否应该再提取下列 enum 为 单独的模块
    - TomlExtractError
- 是否应该再提取下列 struct 为单独的模块
    - ExtractConfig + ExtractionResult 
    - SetConfig 
    - TomlOperations
    - 其他 公共纯函数

- 名字规范： trait 文件名以 _trait 结尾， trait 名以 Trait 结尾 
    - 比如 toml_value_accessor_trait TomlValueAccessorTrait

- 是否应该提取纯函数或者trait
    - 为这些功能：
        - `part.contains('[') && part.ends_with(']')`
        - ```
                let bracket_start = part.find('[').unwrap();
                let array_name = &part[..bracket_start];
                let index_part = &part[bracket_start + 1..part.len() - 1];
          ```

## 拆分 |3
- 错误定义
    - 在 error.rs 文件中定义错误类型
- 类型定义
    - 在 types/get_config.rs 文件中定义类型  ExtractConfig, ExtractionResult
        - 同时使用 Get 替换 Extract 似乎更有语义化。即 GetConfig, GetResult
    - 在 types/set_config.rs 文件中定义类型  SetConfig
- 操作访问器
    - 在 traits/toml_value_accessor.rs 文件中定义 toml 访问器接口
- 操作模块
    - 在 operations/toml_operations.rs 文件中定义操作函数  TomlOperations
    - 在 operations/value_parser.rs    文件中定义 纯函数：解析、格式化等
        - 或者 value_parser 移动 到工具函数模块是否更合适？
- 工具函数
    - 在 utils/path_parser .rs 文件中定义路径解析工具

## 拆分 | 4
- CLI是不是可以先按命令拆分模块，再实现出缺失的功能


## 更新
- 根据最新代码更新main.rs
- 之前是否上传了main.rs文件，如果没有，下列是main.rs的主要内容。

- 更新: 统一使用 snake_case 更新库名，二进制名，命令行工具名。
    - 使用 snake_case 替换 kebab-case。
    - 使用 i_edit_toml 替换 toml_extract， 使其更具有语义化。
    - 比如库名 i_edit_toml ，命令行工具名 i_edit_toml 




## 创建 | 1
```sh
sh -c "touch src/error.rs"
# sh -c "touch src/types.rs"
sh -c "mkdir -p src/types"
sh -c "touch src/types/get_config.rs"
sh -c "touch src/types/set_config.rs"

# sh -c "touch src/traits.rs"
sh -c "mkdir -p src/traits"
sh -c "touch src/traits/toml_value_accessor.rs"
sh -c "touch src/traits/mod.rs"

# sh -c "touch src/utils.rs"
sh -c "mkdir -p src/utils"
sh -c "touch src/utils/path_parser.rs"
sh -c "touch src/utils/value_parser.rs"
sh -c "touch src/utils/mod.rs"

# sh -c "touch src/operations.rs"
sh -c "mkdir -p src/operations"
sh -c "touch src/operations/toml_operations.rs"
sh -c "touch src/operations/mod.rs"

# sh -c "touch src/main.rs"
sh -c "mkdir -p src/xcli"
sh -c "touch src/xcli/get_command.rs"
sh -c "touch src/xcli/set_command.rs"
sh -c "touch src/xcli/append_command.rs"
sh -c "touch src/xcli/remove_command.rs"
sh -c "touch src/xcli/common.rs"
sh -c "touch src/xcli/mod.rs"
```

## 测试 | 1
- 根据模块依赖关系，逐步测试是否通过
- 根据模块依赖关系，如何逐一进行单元测试

```powershell


# cargo test

cargo test --package i_edit_toml --lib -- error::tests

cargo test --package i_edit_toml --lib -- utils::path_parser::tests
cargo test --package i_edit_toml --lib -- utils::value_parser::tests
cargo test --package i_edit_toml --lib -- utils::tests

cargo test --package i_edit_toml --lib -- types::get_config::tests
cargo test --package i_edit_toml --lib -- types::set_config::tests
cargo test --package i_edit_toml --lib -- types::tests

# cargo test --package i_edit_toml --lib -- traits::toml_value_accessor::tests
cargo test --package i_edit_toml --lib -- traits::toml_value_accessor_impl::tests

cargo test --package i_edit_toml --lib -- traits::tests


cargo test --package i_edit_toml --lib -- --nocapture --test-threads=1
cargo test --package i_edit_toml --lib -- --nocapture --test-threads=1 --test get_command

# sh -c "cargo tree | grep i_edit_toml"

cargo test

# 运行特定模块测试
cargo test get::core::tests
cargo test get::utils::tests

cargo test set::core::tests

cargo test set::utils::tests

```

## 提交 | 1

```powershell
# code|format|lint|test|docgen|publish
git add src/error.rs
git commit -m "code: add error.rs"

git add src/utils
git commit -m "code: add path_parser and value_parser"

git add src/types
git commit -m "code: add get_config and set_config"

git add src/traits
git commit -m "code: add toml_value_accessor"

git add src/lib.rs;
git add src/main.rs;
git add Cargo.toml;
git commit -m "code: use i_edit_toml replace toml_extract"

# sh -c "cargo tree | grep i_edit_toml"

# impl not changes
git add src/error.rs
git commit -m "refactor: extract TomlExtractError to error moudle"
git add src/lib.rs
git commit -m "refactor: use error moudle"

git add src/types.rs;git add src/lib.rs;
git commit -m "refactor: extract ExtractConfig, ExtractionResult to types moudle"

git add src/utils.rs;git add src/lib.rs;
git commit -m "refactor: extract utils moudle"

git add src/extract.rs;git add src/lib.rs;
git commit -m "refactor: extract extract moudle"

git add src/extract_preset.rs;git add src/lib.rs;
git commit -m "refactor: extract extract_preset moudle"


git add src/get/*;
git commit -m "refactor: code get moudle"

git add src/set/*;
git commit -m "refactor: code set moudle"

git add src/error.rs;
git commit -m "chore: put for set module"

git add src/lib.rs;git add src/main.rs;
git commit -m "refactor: use set and get module"

git add src;
git commit -m "build: delete unused files"

git add src;
git add Cargo.toml;
git commit -m "refactor: use tomp_path as crate name"

git add Cargo.toml;git commit -m "docs: put crate description"

git add .dockerignore;
git add Dockerfile;
git commit -m "build: add dockerfile"

git add .gitignore
git commit -m "build: ignore .env file"

git add .dockerignore;
git add Dockerfile;
git commit -m "build: diable copy  binary-name.txt"

# 统一 scratch 和 alpine 运行时入口
git add .dockerignore;
git add Dockerfile;
git commit -m "build: Unify Scratch and Alpine runtime entrypoints"

git add Dockerfile.docker.client;
git commit -m "build: add docker client in docker"

git add scripts/*;  
git commit -m "chore: add scripts file"


git add .github/workflows/*;  
git commit -m "chore: add github workflow files"

git add README*;  
git commit -m "docs: add readme files"

git add src
git commit -m "test: add unite test code"

git add src
git commit -m "style: format code"

git add README*;  
git commit -m "docs: put readme files"

git add Dockerfile;
git commit -m "build: not copy Cargo.toml"

git add Dockerfile;
git commit -m "build: put binary name rule"

git add .github/workflows/platform-release.yml;  
git commit -m "build: add multi platform release workflow"

git add .github/workflows/platform-release.yml;
git add  ./scripts/build-release.sh;  
git commit -m "chore: add build-release.sh"

git add .github/workflows/platform-release.yml;
git add  ./scripts/build-release.sh;  
git commit -m "build: try to fix invalid workflow file"
# sh -c "rm scripts/*todel*"
```

## 不智 | 1
- 原有上传的lib.rs和main.rs是可以运行的！拆分之后缺功能和不运行了？！


- 如何使用 git 命令行 将 某指定修改文件 退回到 上次 commit 时的文件
```bash
# 文件未暂存（仅工作区修改，未执行 git add ）
# git checkout HEAD -- src/lib.rs
# git checkout HEAD -- src/main.rs

git restore src/lib.rs
git restore src/main.rs

# 文件已暂存（执行过 git add <文件> ）
# git reset HEAD src/lib.rs;git checkout -- src/lib.rs;

git restore --source=HEAD --staged --worktree src/lib.rs;
git restore --source=HEAD --staged --worktree src/main.rs;
```

## 重构 | 逐步
第一步：将工具函数提取到 utils 模块

第二步：将错误类型提取到 error 模块

第三步：将配置类型提取到 types 模块

第四步：将核心操作提取到 operations 模块

第五步：实现更高级的功能

但关键是：每次重构后都要确保能编译和运行！
- 不要一次性重写所有东西。先确保有一个能运行的基础版本，然后逐步、增量地改进。每次只修改一小部分，确保测试通过后再继续。

- 使用了 i_edit_toml 替换了 toml_extract !
- 按上述步骤 第一步重构就有问题！？
- 不是，仅是提取为模块就出错，就不能按原先的实现？！

- 不改变实现逻辑，只是把代码移动到不同的文件。直接按照原有实现来提取模块。
- 如何提取 TomlExtractError 到 src/error.rs?

- 不改变实现逻辑，只是把代码移动到不同的文件。直接按照原有实现来提取模块。
- 提取 ExtractConfig + ExtractionResult 到 哪个模块合适？ src/types.rs?
- 如何在 lib.rs 中重新导出以保持兼容性

- 提取 lib.rs 中的工具函数 到 哪个模块合适？ src/utils.rs?
- utils.rs 中 使用 `use crate::TomlExtractError;` 还是 `use super::TomlExtractError;` 更合适？
- 提取之前似乎是正常的！为什么提取之后出现了刚刚的错误提示？是修改了实现？是否可以提取但不修改实现！

- lib.rs 中 剩下的 操作函数 提取到 哪个/哪些模块合适？ src/extract.rs ?

- format_output + to_json_value + strip_quotes + strip_quotes_internal  + get_nested_value 不是已经提取到了 utils 模块了吗？


- extract_field + extract_multiple_fields + extract_array + extract_array_element + extract_array_length + （get_nested_value？） ->  src/extract.rs？
- format_output + to_json_value + strip_quotes + strip_quotes_internal ->  src/format.rs？

- get_package_name + get_package_version + get_dependencies、get_package_authors + get_package_keywords + get_package_categories ->  src/extract_preset.rs？

- 不改变实现逻辑，只是把代码移动到不同的文件。直接按照原有实现来提取模块。
- 参考 *_next.rs 的功能和实现，逐步添加模块实现 set 功能。

## 模特添加 | set 模块
- 按照模块加入 set 模块似乎更合适
    - 错误定义
        - 在 src/set/error.rs 文件中定义类型 
        - 还是统一在 src/error.rs 中定义类型 ？
    - 类型定义
        - 在 src/set/types.rs 文件中定义类型  SetConfig
    - 功能核心
        - 在 src/set/core.rs 文件中功能核心  set_field + set_nested_value + set_field_and_save
    - 工具函数
        - 在 src/set/utils.rs 文件中定义工具函数  strip_quotes + split_field_path
    - 命令行输入绑定与处理
        - 在 src/set/xcli.rs 文件中 定义 set 命令的 CLI 结构
        - 在 src/set/xcli.rs 文件中 实现 set 命令的处理逻辑  handle_set_command
        - 在 src/set/xcli.rs 文件中 关联 set 命令与处理函数

```powershell
sh -c "mkdir -p src/set"
sh -c "touch src/set/{mod,types,core,utils,xcli}.rs"
sh -c "rm src/set_*.rs";
sh -c "rm src/set.rs";

sh -c "rm -r src/_*";

```

## 模特添加 | cli 模块
- 按照模块加入 cli 模块似乎更合适
    - 关联命令与处理函数： 在 src/main.rs 中 使用 src/set/xcli.rs 文件中 定义的 关联命令与处理函数

- 补充 src/get/mod.rs
- 补充 src/lib.rs  和 src/main.rs


## 模特添加 | get 模块
- 参考 set 模块，根据 types + extract_preset + extract + utils + lib.rs 提取get模块
    - 类型定义
        - 在 src/get/types.rs 文件中定义类型
    - 功能核心
        - 在 src/get/core.rs 文件中功能核心
    - 工具函数
        - 在 src/get/utils.rs 文件中定义工具函数 
    - 命令行输入绑定与处理
        - 在 src/get/xcli.rs 文件中 定义 get 命令的 CLI 结构 
        - 在 src/get/xcli.rs 文件中 实现 get 命令的处理逻辑  handle_get_command

```powershell
sh -c "mkdir -p src/get"
sh -c "touch src/get/{mod,types,core,utils,xcli}.rs"

sh -c "rm -r src/_*.rs";

```

- 有必要重新导出 set 和 get 模块吗？

## 用法
- 完善当前用法
```bash
# 基础字段提取
cargo run -- get --field package.name                  # 获取包名（带引号）
cargo run -- get --field package.version --output json # 以JSON格式输出版本号
cargo run -- get -f Cargo.toml --field package.license # 指定文件路径

# 字符串去引号
cargo run -- get --field package.name --strip-quotes          # 包名去引号
cargo run -- get --field package.repository --strip-quotes    # 仓库地址去引号
cargo run -- get --field "package.authors[0]" --strip-quotes  # 数组元素去引号

# 数组操作
cargo run -- get --array "package.authors"                   # 获取完整作者数组
cargo run -- get --array-length "package.keywords"           # 获取关键词数组长度
cargo run -- get --array-element "package.categories" --array-index 1 # 获取第二个分类

# 便捷参数（针对Cargo.toml）
cargo run -- get --package-name                              # 直接获取包名
cargo run -- get --package-version                           # 直接获取版本号
cargo run -- get --dependencies                              # 获取所有依赖（JSON格式）
cargo run -- get --authors                                   # 获取所有作者
cargo run -- get --authors 0 --strip-quotes                  # 获取第一个作者（去引号）
cargo run -- get --keywords 2                                # 获取第三个关键词
cargo run -- get --categories                                # 获取所有分类

# 嵌套结构访问
cargo run -- get --field "bin[0].name"                       # 获取第一个二进制目标名称
cargo run -- get --field "lib.path" --strip-quotes           # 获取库文件路径

# 多字段提取
cargo run -- get --multiple package.name --multiple package.version # 提取多个字段
cargo run -- get -m package.license -m package.edition -o json-pretty # 格式化输出多个字段

# 静默模式（忽略错误）
cargo run -- get --field unknown.key --quiet                 # 忽略不存在的字段错误
cargo run -- get --array "invalid.array" --quiet             # 忽略数组不存在错误

# 设置操作示例（补充set命令用法）
cargo run -- set --field package.version --value "0.3.0"     # 输出修改后的内容（不保存）
cargo run -- set -f Cargo.toml --field package.description --value "New description" -i # 原地修改描述
cargo run -- set --field "dependencies.serde.version" --value "1.0.190" --create-missing -i # 创建不存在的字段
cargo run -- set --field package.keywords[3] --value "new-keyword" --type string -i # 修改数组元素
```
- 参考提供的 README.md 文件模板，为该 crate 书写 README.md
- 输出 英文 README.en.md

## 起名
- 为这个 crate 起个什么名字更合适呢？推荐几个名字。
- toml_edit vs i_edit_toml vs toml_extract vs toml_jq vs i_edit_toml
- 有更合适的名字吗
- tomlpath vs tomlfield vs tomlpathedit vs tomlfieldedit

## 构建 | 镜像
- 如何避免 硬编码 code_packager ，使其适合任意 binary repo
- 请不要修改镜像源等写法。

    - 添加 ARG BINARY_NAME 参数
        - 默认值为 code_packager
        - 可以在构建时覆盖
    - 自动从 Cargo.toml 提取名称
        - 如果未指定 BINARY_NAME，则从 Cargo.toml 的 name 字段提取
    - 使用变量替换硬编码
        - 所有 code_packager 出现的地方都替换为 ${BINARY_NAME}
    - 通过文件传递二进制名称
        - 使用 /binary-name.txt 在不同构建阶段间传递二进制名称

- 从 Cargo.toml 提取二进制名称 似乎不合适，因为当前获取方法可能获取到的名字不正确。建议从当前crate 中binary_name.txt中获取，如果存在。
- 如果不传入 BINARY_NAME 也不存在 binary-name.txt ，使用的时哪个名字

- 构建时使用中国镜像示例
- 该 dockerfile 本地开发时使用指南
- 该 dockerfile 本地构建时为不同的标签构建指南

```bash
docker build --build-arg BINARY_NAME=my_binary -t myapp .
# 示例1：完全使用中国镜像（Alpine + Cargo）
docker build --build-arg USE_CHINA_MIRROR=true --build-arg ALPINE_MIRROR=mirrors.aliyun.com --build-arg RUST_MIRROR=ustc --build-arg BINARY_NAME=my_app -t myapp:latest .
```

## 发布 | 镜像
- 下列两个 工作流 输出的镜像名称有哪些

## 构建 | 工作流
- 使用 matrix.exclude 替代动态过滤
    - 避免导致不必要的 runner 分配。

- 分离构建和打包逻辑
    - 将构建和打包分开，使逻辑更清晰
- 使用 cross 工具简化交叉编译
    - 对于 `runner.os == 'Linux' && matrix.arch != 'x86_64'`
- 添加构建状态汇总

- 更好的文件重命名逻辑
```yaml
- name: Normalize filenames
  run: |
    # 使用更系统的重命名方式
    rename_pattern() {
      local file=$1
      local os=$2
      local arch=$3
      local target=$4
      
      # 提取基础名称
      local base="${file%-*}"
      local ext="${file##*.}"
      
      # 创建标准化的名称
      case "$os" in
        windows*) echo "${base}-windows-${arch}.${ext}" ;;
        ubuntu*)  echo "${base}-linux-${arch}.${ext}" ;;
        macos*)   echo "${base}-darwin-${arch}.${ext}" ;;
        *)        echo "${file}" ;;
      esac
    }
```


- 这个语法不支持      
```yaml
    - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ join(matrix.targets, ',') }}
          ${{ matrix.arch != 'x86_64' && format('components: rust-std-{0}', matrix.targets[0]) || '' }}
```

- Build for all targets 太复杂了，各平台代码写在一起难以维护！能否分布根据平台进行！
- Prepare and normalize artifacts  各平台代码写在一起难以维护！
- Prepare unified release 各平台代码写在一起难以维护！
- 构建状态汇总太复杂了

- 似乎无完全支持platforms和architectures控制
- 似乎不支持 dry_run 模式 进行调试
- 一开始传入的矩阵方式不久很不错？！

- 是否应该使用 release 选项决定是否 release 比 debug_mode 更有语义化
- build_docker 阶段是否应该再 release 之后呢
- Docker 构建不依赖于构建好的二进制
- 因为 docker 构建的产物是可以发布到 ghcr 或者 docker.io 的，那么是不是需要 binary 构建好发布到github release 再发布 docker 产物呢 ? 还是 docker + linux + window + macos 并行构建
- 如果是  先构建二进制 ── create-release ── build-docker， 那么是否影响 只构建 docker 的情况
- docker_registry 是否应该 参考platforms，方便ghcr.io和docker.io同时支持或都不支持或支持一种

- build-docker 与linux,macos,window构建阶段没有关系，只是放在他们的构建和release步骤之后再运行而已，即时他们跳过也执行，当然失败则不执行。
- build-docker-standalone 是否多余。

- 下列几种手动触发的输出产物？
```bash
# gh workflow run next-release.yml -f release=false -f platforms="none" -f build_docker=true;
# gh workflow run next-release.yml -f release=false -f platforms="windows" -f build_docker=false;
# gh workflow run next-release.yml -f release=false -f platforms="macos" -f build_docker=false;
# gh workflow run next-release.yml -f release=false -f platforms="linux" -f build_docker=false;
```

```bash
# gh workflow run next-release.yml -f release=true -f platforms="none" -f build_docker=true;
# gh workflow run next-release.yml -f release=true -f platforms="windows" -f build_docker=false;
# gh workflow run next-release.yml -f release=true -f platforms="macos" -f build_docker=false;
# gh workflow run next-release.yml -f release=true -f platforms="linux" -f build_docker=false;
# gh workflow run next-release.yml -f release=true -f platforms="linux" -f build_docker=true;
# gh workflow run next-release.yml -f release=true -f platforms="linux,window,macos" -f build_docker=true;
```
- 当发送 tags 为 vx.y.z 是的输出产物？ `git tag v$ver;git push ghg;git push ghg --tags;`  `git tag v$ver;git push ghg;git push ghg v$ver;`

- 当发送 tags 为 vx.y.z 是的输出产物？ `git tag $ver ;git push ghg;git push ghg $ver;` `git tag 20251220 ;git push ghg;git push ghg 20251220;`

- tags 触发时 为什么不能使用inputs的参数值？如何支持使用inputs的参数值
- 构建 → 测试 → 准备发布文件（重命名 + 生成 checksum）→ 上传
- 构建 → 测试 → 准备发布文件（重命名 + 压缩二进制文件 +  生成 checksum）→ 上传
- 我所说的压缩二进制文件是说使用upx,strip等对二进制文件进行精简其大小但又不影响运行，而不是压缩成zip文件

- 参考 linux-release.yml + macos-release.yml + window-release.yml 的构建，更新 next-release.yml 中 build_linux,build_window,build_macos。 + 用upx,strip等对二进制文件进行精简其大小