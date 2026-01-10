# Goldminer

基于 Bevy 游戏引擎开发的 2D 采矿游戏，具有完整的游戏循环、UI 系统和音效支持。

## 游戏玩法

### 基础操作

| 按键 | 功能 |
|------|------|
| ↓ / J / K | 发射钩子 |
| ↑ / U / I | 使用炸药（钩子回缩且抓到物体时） |
| Space | 跳过关卡（达到目标分数后） |

### 菜单导航

| 按键 | 功能 |
|------|------|
| ↑ / ↓ | 上下移动选项 |
| Enter / J / K | 确认选择 |

### 商店界面

| 按键 | 功能 |
|------|------|
| ← / → | 左右切换商品 |
| Enter / J / K | 购买当前商品 |
| Space | 退出商店 |

### 通用功能

| 按键 | 功能 |
|------|------|
| P | 暂停游戏 |
| Esc | 返回/取消 |
| ~ | 切换开发工具（仅开发模式） |

## 运行游戏

### 开发环境要求
- Rust 1.70+ (edition 2024)
- Bevy 0.17

### 构建和运行

```bash
# 开发模式运行（本地）
cargo run

# Web 开发模式
bevy run web

# 发布构建
cargo build --release
```

## 开发特性

- 动态链接优化编译时间
- 资源热重载
- 内置开发工具和调试功能
- 性能分析支持 (Tracy)

## 项目结构

- `src/main.rs` - 应用入口和插件配置
- `src/screens/` - 游戏屏幕（标题、游戏、商店等）
- `src/menus/` - 菜单系统
- `src/demo/` - 游戏核心玩法（钩子、玩家、关卡等）
- `src/theme/` - UI 主题和组件
- `src/audio.rs` - 音效系统
- `src/asset_tracking.rs` - 资源加载和依赖跟踪
- `src/dev_tools.rs` - 开发调试工具

## 技术栈

- **游戏引擎**: Bevy 0.17
- **架构模式**: ECS (Entity Component System)
- **状态管理**: Bevy States 系统
- **平台支持**: 原生平台和 Web 平台 (WASM)

## 许可证

本项目使用 [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) 模板创建。
