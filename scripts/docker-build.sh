#!/bin/bash

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
CONFIG_FILE=".docker-build.conf"
if [ -f "$CONFIG_FILE" ]; then
    source "$CONFIG_FILE"
fi

IMAGE_NAME="${IMAGE_NAME:-myapp}"
USE_CHINA_MIRROR="${USE_CHINA_MIRROR:-true}"
ALPINE_MIRROR="${ALPINE_MIRROR:-mirrors.aliyun.com}"
RUST_MIRROR="${RUST_MIRROR:-ustc}"
PUSH="${PUSH:-false}"
TARGET="${TARGET:-all}"

# 获取项目信息
get_project_info() {
    if [ -f "Cargo.toml" ]; then
        CRATE_NAME=$(grep -E '^name\s*=' Cargo.toml | head -1 | sed -E 's/^name\s*=\s*"([^"]+)".*/\1/' 2>/dev/null || echo "app")
        VERSION=$(grep -E '^version\s*=' Cargo.toml | head -1 | sed -E 's/^version\s*=\s*"([^"]+)".*/\1/' 2>/dev/null || echo "0.0.0")
    else
        CRATE_NAME="app"
        VERSION="0.0.0"
    fi
    
    if [ -f "binary-name.txt" ]; then
        BINARY_NAME=$(cat binary-name.txt | xargs)
    else
        BINARY_NAME="$CRATE_NAME"
    fi
    
    GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    BUILD_DATE=$(date +"%Y%m%d")
    BUILD_TIME=$(date +"%H%M%S")
}

# 显示配置
show_config() {
    echo -e "${BLUE}=========================================${NC}"
    echo -e "${GREEN}🚀 Docker Build with China Mirror${NC}"
    echo -e "${BLUE}=========================================${NC}"
    echo -e "Image Name: ${YELLOW}$IMAGE_NAME${NC}"
    echo -e "Binary Name: ${YELLOW}$BINARY_NAME${NC}"
    echo -e "Version: ${YELLOW}$VERSION${NC}"
    echo -e "Git Commit: ${YELLOW}$GIT_COMMIT${NC}"
    echo -e "China Mirror: ${YELLOW}$USE_CHINA_MIRROR${NC}"
    if [ "$USE_CHINA_MIRROR" = "true" ]; then
        echo -e "Alpine Mirror: ${YELLOW}$ALPINE_MIRROR${NC}"
        echo -e "Rust Mirror: ${YELLOW}$RUST_MIRROR${NC}"
    fi
    echo -e "Push Images: ${YELLOW}$PUSH${NC}"
    echo -e "Build Target: ${YELLOW}$TARGET${NC}"
    echo -e "${BLUE}=========================================${NC}"
    echo ""
}

# 构建函数
build_target() {
    local target=$1
    local tag_prefix=$2
    
    echo -e "${GREEN}📦 Building $target...${NC}"
    
    # 基础构建命令
    local cmd="docker build --target $target"
    
    # 添加构建参数
    if [ "$USE_CHINA_MIRROR" = "true" ]; then
        cmd="$cmd --build-arg USE_CHINA_MIRROR=true"
        cmd="$cmd --build-arg ALPINE_MIRROR=$ALPINE_MIRROR"
        cmd="$cmd --build-arg RUST_MIRROR=$RUST_MIRROR"
    fi
    
    cmd="$cmd --build-arg BINARY_NAME=$BINARY_NAME"
    
    # 定义标签
    local tags=()
    
    case $target in
        development)
            tags=("dev-$GIT_COMMIT" "dev-$BUILD_DATE" "dev-latest" "development")
            if [ "$USE_CHINA_MIRROR" = "true" ]; then
                tags+=("dev-cn" "dev-china")
            fi
            ;;
        runtime-alpine)
            tags=("$VERSION-alpine" "alpine-$GIT_COMMIT" "alpine-latest" "alpine")
            if [ "$USE_CHINA_MIRROR" = "true" ]; then
                tags+=("alpine-cn" "alpine-china")
            fi
            ;;
        runtime)
            tags=("$VERSION" "$GIT_COMMIT" "latest" "prod" "minimal")
            if [ "$USE_CHINA_MIRROR" = "true" ]; then
                tags+=("prod-cn" "china" "cn-mirror")
            fi
            ;;
    esac
    
    # 添加标签前缀
    if [ -n "$tag_prefix" ]; then
        local prefixed_tags=()
        for tag in "${tags[@]}"; do
            prefixed_tags+=("$tag_prefix$tag")
        done
        tags=("${prefixed_tags[@]}")
    fi
    
    # 添加标签到命令
    for tag in "${tags[@]}"; do
        cmd="$cmd -t $IMAGE_NAME:$tag"
    done
    
    # 添加元数据
    cmd="$cmd --label \"app.name=$CRATE_NAME\""
    cmd="$cmd --label \"app.version=$VERSION\""
    cmd="$cmd --label \"git.commit=$GIT_COMMIT\""
    cmd="$cmd --label \"build.date=$BUILD_DATE.$BUILD_TIME\""
    cmd="$cmd --label \"china.mirror=$USE_CHINA_MIRROR\""
    
    echo -e "${YELLOW}Command:${NC} $cmd"
    echo ""
    
    # 执行构建
    if eval $cmd .; then
        echo -e "${GREEN}✅ $target built successfully!${NC}"
        
        # 推送镜像
        if [ "$PUSH" = "true" ]; then
            echo -e "${BLUE}📤 Pushing images...${NC}"
            for tag in "${tags[@]}"; do
                docker push "$IMAGE_NAME:$tag"
            done
        fi
        
        # 显示构建的镜像
        echo -e "${BLUE}📊 Built images:${NC}"
        for tag in "${tags[@]}"; do
            echo "  - $IMAGE_NAME:$tag"
        done
        echo ""
    else
        echo -e "${RED}❌ Failed to build $target${NC}"
        return 1
    fi
}

# 主函数
main() {
    get_project_info
    show_config
    
    # 根据TARGET参数构建
    case $TARGET in
        all)
            build_target "development"
            build_target "runtime-alpine"
            build_target "runtime"
            ;;
        dev|development)
            build_target "development"
            ;;
        alpine|runtime-alpine)
            build_target "runtime-alpine"
            ;;
        prod|production|runtime)
            build_target "runtime"
            ;;
        *)
            echo -e "${RED}❌ Unknown target: $TARGET${NC}"
            echo "Available targets: all, dev, alpine, prod"
            exit 1
            ;;
    esac
    
    echo -e "${GREEN}=========================================${NC}"
    echo -e "${GREEN}🎉 All builds completed successfully!${NC}"
    echo -e "${GREEN}=========================================${NC}"
}

# 解析参数
while [[ $# -gt 0 ]]; do
    case $1 in
        --name=*)
            IMAGE_NAME="${1#*=}"
            shift
            ;;
        --china=*)
            USE_CHINA_MIRROR="${1#*=}"
            shift
            ;;
        --alpine-mirror=*)
            ALPINE_MIRROR="${1#*=}"
            shift
            ;;
        --rust-mirror=*)
            RUST_MIRROR="${1#*=}"
            shift
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --target=*)
            TARGET="${1#*=}"
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --name=IMAGE_NAME      Set image name (default: myapp)"
            echo "  --china=true|false     Use China mirror (default: true)"
            echo "  --alpine-mirror=URL    Set Alpine mirror URL"
            echo "  --rust-mirror=NAME     Set Rust mirror (tuna|ustc)"
            echo "  --push                 Push images after build"
            echo "  --target=TARGET        Build target: all, dev, alpine, prod"
            echo "  --help                 Show this help"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# 运行主函数
main