
## 发布到 crates.io

### 准备工作
1. 创建 crates.io 账号
2. 获取 API token：
   ```bash
   cargo login <your-api-token>
   ```

### 发布流程
```bash
# pack,format,lint,test,docgen,publish
# sh ./scripts/pre.ai.sh -i src -o src_code.md -a Cargo.toml

cargo fmt;cargo fmt -- --check > src_fmt_log.md 2>&1
cargo clippy -- -D warnings > src_lint_log.md 2>&1
cargo clippy --all-features -- -D warnings > src_lint_log.md 2>&1

cargo test --all-features > src_test_log.md 2>&1
# cargo test --features "windows-interop,messages"  > src_test_log.md 2>&1
# cargo test --features "windows-interop,messages,bit-ops,coordinates"

cargo doc --no-deps --all-features > src_docgen_log.md 2>&1
cargo package --allow-dirty > src_pack_log.md 2>&1

# cargo package --list --allow-dirty

cargo publish --dry-run --registry crates-io --allow-dirty > src_publish_log.md 2>&1

# ...
# run workflow
```
