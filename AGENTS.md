# Goldminer - AI 开发知识库

**Generated:** 2026-02-03
**Branch:** (current)
**Stack:** Rust + Bevy 0.18

## OVERVIEW

基于 Bevy 引擎的 2D 采矿游戏，采用 ECS 架构和状态机管理游戏流程。模块化插件系统，完整的 UI/音效/资源管理。

## STRUCTURE

```
goldminer/
├── src/
│   ├── main.rs              # 入口点
│   ├── config.rs            # 资源配置（图片、实体、关卡）
│   ├── constants.rs         # 颜色常量
│   ├── asset_tracking.rs    # 资源加载跟踪
│   ├── audio.rs             # 音效系统
│   ├── dev_tools.rs         # 开发工具
│   ├── demo/                # 游戏核心玩法
│   ├── menus/               # 菜单系统
│   ├── screens/             # 屏幕状态管理
│   └── theme/               # UI 主题与组件
├── assets/                  # 游戏资源
├── .github/workflows/       # CI/CD
└── Cargo.toml
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 修改游戏逻辑 | `src/demo/` | hook、player、level |
| 添加 UI 组件 | `src/theme/widget.rs` | 使用 `widget::button()` 等 |
| 屏幕状态 | `src/screens/mod.rs` | Screen 枚举定义 |
| 菜单系统 | `src/menus/mod.rs` | Menu 枚举定义 |
| 音效播放 | `src/audio.rs` | `music()` / `sound_effect()` |

## CONVENTIONS

### 状态管理
- 屏幕状态：`NextState<Screen>`
- 菜单状态：`NextState<Menu>`
- 系统必须加 `.run_if(in_state(...))` 过滤

### 资源清理
- UI 实体必须添加 `DespawnOnExit(Screen::X)` 组件
- 定时器/事件使用 `.on_despawn_free()` 自动清理

### 坐标系
- UI：Love2D 坐标系（中心 0,0）
- 转换：`src/utils.rs::love_to_bevy_coords()`

### 组件定义
```rust
#[derive(Component, Reflect)]
pub struct MyComponent { ... }
```

## ANTI-PATTERNS

- ❌ **不要** 在 `Update` 中运行系统而不加状态过滤
- ❌ **不要** 硬编码颜色，使用 `theme::palette::*` 常量
- ❌ **不要** 直接操作关卡数据，使用 `Resource` 或 `Event`
- ❌ **不要** 遗留未清理的 `Timer` 或单次事件

## COMMANDS

```bash
# 开发运行
cargo run

# Web 开发
bevy run web

# 发布构建
cargo build --release

# 代码检查
cargo fmt && cargo clippy
```

## CI/CD

- `.github/workflows/ci.yaml`: 格式检查、Clippy、Bevy Lints、测试
- `.github/workflows/release.yaml`: 多平台构建（Win/macOS/Linux/WASM）

## NOTES

- **Edition**: Rust 2024（实验性）
- **测试**: 当前无测试文件
- **保存数据**: `savedata.txt` 在项目根目录
- **WASM**: 使用 `AssetMetaCheck::Never` 避免构建错误
