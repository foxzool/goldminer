# GitHub 发布部署指南

基于 [bevy_cli](https://github.com/TheBevyFlock/bevy_cli) 模板的多平台发布工作流。

## 功能特性

- **多平台构建**: Windows, macOS (通用二进制), Linux, Web (WASM)
- **自动部署**: GitHub Releases, itch.io, GitHub Pages
- **发布前检查**: 自动运行格式检查、Clippy、测试
- **自动发布说明**: 生成包含下载链接和安装说明的 release notes

## 快速开始

### 1. 配置 Secrets

在 GitHub 仓库 Settings → Secrets and variables → Actions 中添加：

| Secret | 说明 | 必需 |
|--------|------|------|
| `BUTLER_CREDENTIALS` | itch.io API 密钥，从 [itch.io](https://itch.io/user/settings/api-keys) 获取 | 如需部署到 itch.io |

### 2. 触发发布

#### 方式 A: GitHub Web 界面（推荐）

1. 进入仓库的 Actions 页面
2. 选择 **Release** 工作流
3. 点击 **Run workflow**
4. 填写参数：
   - **Version**: `v1.2.3`（遵循语义化版本）
   - 选择要构建的平台
   - 选择部署目标
   - 点击 **Run workflow**

#### 方式 B: GitHub CLI

```bash
gh workflow run release.yaml -f version=v1.2.3 -f build_for_web=true -f deploy_to_itch=true
```

### 3. 参数说明

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `version` | 版本号，格式 `v1.2.3` | 必需 |
| `build_for_windows` | 构建 Windows 版本 | `true` |
| `build_for_macos` | 构建 macOS 版本 | `true` |
| `build_for_linux` | 构建 Linux 版本 | `true` |
| `build_for_web` | 构建 Web (WASM) 版本 | `true` |
| `upload_to_github` | 上传到 GitHub Releases | `true` |
| `deploy_to_itch` | 部署到 itch.io | `true` |
| `deploy_to_github_pages` | 部署 Web 到 GitHub Pages | `false` |
| `deny_warnings` | 构建时拒绝警告 | `false` |

## 配置说明

### 修改 itch.io 项目

编辑 `.github/workflows/release.yaml`：

```yaml
env:
  # 修改为你的 itch.io 用户名/项目名
  itch_page: yourname/yourproject
```

### 修改应用信息

```yaml
env:
  cargo_build_binary_name: goldminer  # 二进制文件名
  assets_path: assets                  # 资源目录
  app_id: foxzool.goldminer           # macOS Bundle ID
```

### GitHub Pages 配置

如需启用 GitHub Pages 部署：

1. 仓库 Settings → Pages → Source: GitHub Actions
2. 运行工作流时勾选 `deploy_to_github_pages`

## 发布流程

```
触发工作流
    ↓
预发布检查 (格式、Clippy、测试)
    ↓
获取版本号
    ↓
并行构建 (Windows/macOS/Linux/Web)
    ↓
打包产物
    ↓
上传到 GitHub Release
    ↓
部署到 itch.io (可选)
    ↓
部署到 GitHub Pages (可选)
```

## 构建产物

| 平台 | 文件名 | 格式 |
|------|--------|------|
| Windows | `goldminer-windows.zip` | ZIP |
| macOS | `goldminer-macos.dmg` | DMG |
| Linux | `goldminer-linux.zip` | ZIP |
| Web | `goldminer-web.zip` | ZIP |

## 故障排除

### WASM 构建失败

检查 `Cargo.toml` 是否有：
```toml
[target.wasm32-unknown-unknown.dependencies]
getrandom = { version = "0.3", features = ["wasm_js"] }
```

### itch.io 部署失败

1. 确认 `BUTLER_CREDENTIALS` secret 已配置
2. 确认 `itch_page` 格式正确：`用户名/项目名`
3. 检查 itch.io 项目是否已创建

### 缓存问题

如需清除缓存：
- 在仓库 Settings → Actions → Cache 中删除相关缓存
- 或设置 `use_github_cache: false`

## 参考

- [bevy_cli 文档](https://github.com/TheBevyFlock/bevy_cli)
- [bevy_new_2d 模板](https://github.com/TheBevyFlock/bevy_new_2d)
- [itch.io Butler 文档](https://itch.io/docs/butler/)
