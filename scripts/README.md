## crate-publish-pre
```bash
./scripts/crate-publish-pre.sh
```

## docker-build
### basic usage
```bash
# 1. 简单多标签构建
./scripts/docker-build.sh

# 2. 自定义镜像名
./scripts/docker-build.sh --name=zero/i_edit_toml --target=prod

./scripts/docker-build.sh --name=zero/i_edit_toml --target=prod --china=true --rust-mirror=ustc --alpine-mirror=mirrors.aliyun.com

./scripts/docker-build.sh --push --china=false
```

### in ci/cd
```yaml
# GitHub Actions示例
name: Build and Tag

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up Docker
      uses: docker/setup-buildx-action@v2
    
    - name: Build with multiple tags
      run: |
        chmod +x smart-tag-builder.sh
        ./scripts/docker-build.sh --china=false
    
    - name: Push to Registry
      if: github.ref == 'refs/heads/main'
      run: |
        docker login -u ${{ secrets.DOCKER_USERNAME }} -p ${{ secrets.DOCKER_PASSWORD }}
        ./scripts/docker-build.sh --push --china=false

```