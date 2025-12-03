#!/bin/bash
# 多平台构建和发布脚本

# 全局变量
BINARY_NAME=""
REPO_NAME=""
ARTIFACT_DIR=""
RELEASE_DIR=""

# 设置构建矩阵
# 参数: platforms architectures current_os current_arch
setup_build_matrix() {
    local platforms="$1"
    local architectures="$2"
    local current_os="$3"
    local current_arch="$4"
    
    echo "Requested platforms: $platforms"
    echo "Requested architectures: $architectures"
    
    # 提取基础平台名称
    local os_name
    if [[ "$current_os" == "windows-latest" ]]; then
        os_name="windows"
    elif [[ "$current_os" == "ubuntu-latest" ]]; then
        os_name="linux"
    elif [[ "$current_os" == "macos-latest" ]]; then
        os_name="macos"
    else
        os_name="$current_os"
    fi
    
    # 检查是否需要构建
    if [[ ",$platforms," == *",$os_name,"* ]] || [[ "$platforms" == "all" ]] || [[ -z "$platforms" ]]; then
        if [[ ",$architectures," == *",$current_arch,"* ]] || [[ "$architectures" == "all" ]] || [[ -z "$architectures" ]]; then
            echo "✅ Building for $current_os-$current_arch"
            echo "build_this=true"
            return 0
        else
            echo "❌ Skipping $current_arch (not in requested architectures)"
            echo "build_this=false"
            return 1
        fi
    else
        echo "❌ Skipping $os_name (not in requested platforms)"
        echo "build_this=false"
        return 1
    fi
}

# 安装交叉编译工具
# 参数: arch
install_cross_compile_tools() {
    local arch="$1"
    
    echo "Installing cross-compilation tools for $arch..."
    sudo apt-get update -y
    
    case "$arch" in
        aarch64)
            sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
            ;;
        armv7)
            sudo apt-get install -y gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf
            ;;
        *)
            echo "No specific cross-compilation tools needed for $arch"
            ;;
    esac
}

# 构建所有目标
# 参数: targets runner_os arch binary_name file_extension
build_targets() {
    local targets_str="$1"
    local runner_os="$2"
    local arch="$3"
    local binary_name="$4"
    local file_extension="$5"
    
    echo "🏗️ Building for $runner_os ($arch)"
    echo "Targets: $targets_str"
    
    # 将逗号分隔的 targets 转换为数组
    IFS=',' read -ra TARGET_ARRAY <<< "$targets_str"
    
    for target in "${TARGET_ARRAY[@]}"; do
        echo "Building for target: $target"
        
        # 设置交叉编译环境变量
        case "$target" in
            aarch64-unknown-linux-gnu)
                export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
                ;;
            armv7-unknown-linux-gnueabihf)
                export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
                ;;
        esac
        
        # 构建项目
        cargo build --release --target "$target"
        
        echo "📁 Build output for $target:"
        if [[ "$runner_os" == "Windows" ]]; then
            find "target/$target/release/" -name "*.exe" -type f | while read -r file; do
                ls -la "$file"
            done
        else
            find "target/$target/release/" -maxdepth 1 -type f -executable | while read -r file; do
                ls -la "$file"
            done
        fi
    done
}

# 准备构建产物
# 参数: targets runner_os binary_name file_extension arch os_name
prepare_artifacts() {
    local targets_str="$1"
    local runner_os="$2"
    local binary_name="$3"
    local file_extension="$4"
    local arch="$5"
    local os_name="$6"
    
    ARTIFACT_DIR="release-artifacts-${os_name}-${arch}"
    mkdir -p "$ARTIFACT_DIR"
    
    # 将逗号分隔的 targets 转换为数组
    IFS=',' read -ra TARGET_ARRAY <<< "$targets_str"
    
    for target in "${TARGET_ARRAY[@]}"; do
        # 查找可执行文件
        local executables
        if [[ "$runner_os" == "Windows" ]]; then
            executables=$(find "target/$target/release/" -name "*.exe" -type f)
        else
            # 在非 Windows 系统上，查找可执行文件但排除库文件
            executables=$(find "target/$target/release/" -maxdepth 1 -type f -perm -111 ! -name "*.so" ! -name "*.dylib" ! -name "*.a" ! -name "*.rlib")
        fi
        
        for exe in $executables; do
            local filename=$(basename "$exe")
            local new_name
            
            # 重命名文件以包含平台和架构信息
            if [[ "$filename" == "${binary_name}${file_extension}" ]]; then
                new_name="${binary_name}-${target//-/_}${file_extension}"
            else
                local basename="${filename%${file_extension}}"
                new_name="${basename}-${target//-/_}${file_extension}"
            fi
            
            cp "$exe" "$ARTIFACT_DIR/$new_name"
            echo "✅ Prepared: $new_name"
            
            # 计算文件大小
            if command -v stat &> /dev/null; then
                local size
                if stat -f%z "$exe" 2>/dev/null; then
                    size=$(stat -f%z "$exe")
                else
                    size=$(stat -c%s "$exe")
                fi
                local size_mb=$(echo "scale=2; $size / 1024 / 1024" | bc)
                echo "  Size: ${size_mb} MB"
            fi
        done
    done
    
    echo "artifact_dir=$ARTIFACT_DIR"
}

# 准备发布文件
# 参数: artifacts_path
prepare_release_files() {
    local artifacts_path="$1"
    
    echo "📦 Preparing multi-platform release files..."
    RELEASE_DIR="final-release"
    mkdir -p "$RELEASE_DIR"
    
    # 复制所有二进制文件到发布目录
    find "$artifacts_path" -type f -exec cp {} "$RELEASE_DIR/" \;
    
    # 重命名文件使其更友好
    cd "$RELEASE_DIR" || exit 1
    for file in *; do
        # 替换复杂的命名
        local new_name=$(echo "$file" | sed \
            -e 's/windows-latest_x86_64/windows-x64/' \
            -e 's/ubuntu-latest_x86_64/linux-x64/' \
            -e 's/macos-latest_x86_64/macos-x64/' \
            -e 's/ubuntu-latest_aarch64/linux-arm64/' \
            -e 's/ubuntu-latest_armv7/linux-armv7/' \
            -e 's/x86_64_pc_windows_msvc/msvc/' \
            -e 's/x86_64_pc_windows_gnu/gnu/' \
            -e 's/x86_64_unknown_linux_gnu/gnu/' \
            -e 's/x86_64_unknown_linux_musl/musl/' \
            -e 's/x86_64_apple_darwin//' \
            -e 's/aarch64_unknown_linux_gnu/gnu/' \
            -e 's/aarch64_unknown_linux_musl/musl/' \
            -e 's/armv7_unknown_linux_gnueabihf/gnueabihf/' \
            -e 's/__/_/g' \
            -e 's/_\././')
        
        if [[ "$file" != "$new_name" ]]; then
            mv "$file" "$new_name"
            echo "📝 Renamed: $file -> $new_name"
        fi
    done
    
    # 显示文件列表
    echo "📄 Release files:"
    ls -la | while read -r line; do
        echo "  $line"
    done
    
    echo "release_dir=$RELEASE_DIR"
}

# 创建校验和文件
# 参数: release_dir
create_checksums() {
    local release_dir="$1"
    
    echo "🔐 Creating checksums..."
    cd "$release_dir" || exit 1
    
    # 为每个文件创建校验和
    for file in *; do
        if [[ -f "$file" && ! "$file" =~ \.(sha256|sha512)$ ]]; then
            echo "Creating checksums for: $file"
            
            # SHA256
            sha256sum "$file" | awk '{print $1 " *" $2}' > "${file}.sha256"
            
            # SHA512
            sha512sum "$file" | awk '{print $1 " *" $2}' > "${file}.sha512"
            
            echo "✅ Created checksums for: $file"
        fi
    done
    
    # 创建合并的校验和文件
    sha256sum * | grep -v '\.sha' > SHA256SUMS
    sha512sum * | grep -v '\.sha' > SHA512SUMS
}

# 生成发布说明
# 参数: repo_name tag_name release_dir
generate_release_notes() {
    local repo_name="$1"
    local tag_name="$2"
    local release_dir="$3"
    
    echo "📝 Generating release notes..."
    
    cat > release-notes.md << EOF
# $repo_name v$tag_name

## 📦 Binaries

This release includes binaries for multiple platforms:

EOF
    
    cd "$release_dir" || exit 1
    
    # 按平台分组显示文件
    echo "### Windows" >> ../release-notes.md
    ls *windows* 2>/dev/null | grep -v '\.sha' | while read -r file; do
        if [[ -f "$file" ]]; then
            local size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file")
            local size_mb=$(echo "scale=2; $size / 1024 / 1024" | bc 2>/dev/null || echo "$size")
            echo "- \`$file\` (${size_mb} MB)" >> ../release-notes.md
        fi
    done
    
    echo "" >> ../release-notes.md
    echo "### Linux" >> ../release-notes.md
    ls *linux* 2>/dev/null | grep -v '\.sha' | while read -r file; do
        if [[ -f "$file" ]]; then
            local size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file")
            local size_mb=$(echo "scale=2; $size / 1024 / 1024" | bc 2>/dev/null || echo "$size")
            echo "- \`$file\` (${size_mb} MB)" >> ../release-notes.md
        fi
    done
    
    echo "" >> ../release-notes.md
    echo "### macOS" >> ../release-notes.md
    ls *macos* 2>/dev/null | grep -v '\.sha' | while read -r file; do
        if [[ -f "$file" ]]; then
            local size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file")
            local size_mb=$(echo "scale=2; $size / 1024 / 1024" | bc 2>/dev/null || echo "$size")
            echo "- \`$file\` (${size_mb} MB)" >> ../release-notes.md
        fi
    done
    
    cat >> ../release-notes.md << EOF

## 🔐 Verification

You can verify the integrity of the downloads using the provided checksums:

\`\`\`bash
# Verify SHA256
sha256sum -c SHA256SUMS

# Verify SHA512  
sha512sum -c SHA512SUMS
\`\`\`

## 🚀 Installation

Choose the appropriate binary for your platform and architecture.

## 📄 Changelog

See the git history for detailed changes.
EOF
    
    echo "release_notes=release-notes.md"
}

# 主函数 - 构建流程
main_build() {
    local platforms="$1"
    local architectures="$2"
    local os_name="$3"
    local arch="$4"
    local targets="$5"
    local binary_name="$6"
    local file_extension="$7"
    
    echo "🚀 Starting build process..."
    
    # 设置构建矩阵
    local matrix_output
    matrix_output=$(setup_build_matrix "$platforms" "$architectures" "$os_name" "$arch")
    echo "$matrix_output"
    
    # 检查是否需要构建
    if ! echo "$matrix_output" | grep -q "build_this=true"; then
        echo "Skipping build for $os_name-$arch"
        return 1
    fi
    
    # 安装交叉编译工具
    if [[ "$os_name" == "ubuntu-latest" ]] && [[ "$arch" != "x86_64" ]]; then
        install_cross_compile_tools "$arch"
    fi
    
    # 构建目标
    build_targets "$targets" "$(uname -s)" "$arch" "$binary_name" "$file_extension"
    
    # 准备构建产物
    local artifacts_output
    artifacts_output=$(prepare_artifacts "$targets" "$(uname -s)" "$binary_name" "$file_extension" "$arch" "$os_name")
    echo "$artifacts_output"
    
    # 提取 artifact_dir
    ARTIFACT_DIR=$(echo "$artifacts_output" | grep -E "^artifact_dir=" | cut -d= -f2)
    echo "Artifacts directory: $ARTIFACT_DIR"
}

# 主函数 - 发布流程
main_release() {
    local repo_name="$1"
    local tag_name="$2"
    local artifacts_path="$3"
    
    echo "🚀 Starting release process..."
    
    # 准备发布文件
    local release_output
    release_output=$(prepare_release_files "$artifacts_path")
    echo "$release_output"
    
    # 提取 release_dir
    RELEASE_DIR=$(echo "$release_output" | grep -E "^release_dir=" | cut -d= -f2)
    echo "Release directory: $RELEASE_DIR"
    
    # 创建校验和
    create_checksums "$RELEASE_DIR"
    
    # 生成发布说明
    local notes_output
    notes_output=$(generate_release_notes "$repo_name" "$tag_name" "$RELEASE_DIR")
    echo "$notes_output"
    
    # 提取 release_notes 文件路径
    local release_notes=$(echo "$notes_output" | grep -E "^release_notes=" | cut -d= -f2)
    echo "Release notes: $release_notes"
}

# 显示帮助信息
show_help() {
    cat << EOF
多平台构建和发布脚本

用法:
  $0 build [OPTIONS]      执行构建流程
  $0 release [OPTIONS]    执行发布流程

构建选项:
  --platforms=PLATFORMS     要构建的平台 (逗号分隔，默认: windows,linux,macos)
  --architectures=ARCHS     要构建的架构 (逗号分隔，默认: x86_64)
  --os-name=OS_NAME         操作系统名称 (windows-latest, ubuntu-latest, macos-latest)
  --arch=ARCH               架构 (x86_64, aarch64, armv7)
  --targets=TARGETS         Rust 目标平台 (逗号分隔)
  --binary-name=NAME        二进制文件名
  --file-extension=EXT      文件扩展名

发布选项:
  --repo-name=NAME          仓库名称
  --tag-name=TAG            标签名称
  --artifacts-path=PATH     构建产物路径

示例:
  # 执行构建
  $0 build \\
    --platforms=windows,linux \\
    --architectures=x86_64 \\
    --os-name=ubuntu-latest \\
    --arch=x86_64 \\
    --targets=x86_64-unknown-linux-gnu \\
    --binary-name=myapp \\
    --file-extension=""
  
  # 执行发布
  $0 release \\
    --repo-name=myapp \\
    --tag-name=v1.0.0 \\
    --artifacts-path=./all-artifacts
EOF
}

# 解析命令行参数
parse_arguments() {
    local command="$1"
    shift
    
    case "$command" in
        build)
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --platforms=*)
                        PLATFORMS="${1#*=}"
                        shift
                        ;;
                    --architectures=*)
                        ARCHITECTURES="${1#*=}"
                        shift
                        ;;
                    --os-name=*)
                        OS_NAME="${1#*=}"
                        shift
                        ;;
                    --arch=*)
                        ARCH="${1#*=}"
                        shift
                        ;;
                    --targets=*)
                        TARGETS="${1#*=}"
                        shift
                        ;;
                    --binary-name=*)
                        BINARY_NAME="${1#*=}"
                        shift
                        ;;
                    --file-extension=*)
                        FILE_EXTENSION="${1#*=}"
                        shift
                        ;;
                    *)
                        echo "未知参数: $1"
                        show_help
                        exit 1
                        ;;
                esac
            done
            ;;
        release)
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --repo-name=*)
                        REPO_NAME="${1#*=}"
                        shift
                        ;;
                    --tag-name=*)
                        TAG_NAME="${1#*=}"
                        shift
                        ;;
                    --artifacts-path=*)
                        ARTIFACTS_PATH="${1#*=}"
                        shift
                        ;;
                    *)
                        echo "未知参数: $1"
                        show_help
                        exit 1
                        ;;
                esac
            done
            ;;
        *)
            echo "未知命令: $command"
            show_help
            exit 1
            ;;
    esac
}

# 脚本入口点
main() {
    if [[ $# -lt 1 ]]; then
        show_help
        exit 1
    fi
    
    local command="$1"
    shift
    
    parse_arguments "$command" "$@"
    
    case "$command" in
        build)
            # 设置默认值
            PLATFORMS=${PLATFORMS:-"windows,linux,macos"}
            ARCHITECTURES=${ARCHITECTURES:-"x86_64"}
            FILE_EXTENSION=${FILE_EXTENSION:-""}
            
            main_build "$PLATFORMS" "$ARCHITECTURES" "$OS_NAME" "$ARCH" \
                "$TARGETS" "$BINARY_NAME" "$FILE_EXTENSION"
            ;;
        release)
            main_release "$REPO_NAME" "$TAG_NAME" "$ARTIFACTS_PATH"
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            echo "未知命令: $command"
            show_help
            exit 1
            ;;
    esac
}

# 如果脚本被直接执行，调用 main 函数
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi