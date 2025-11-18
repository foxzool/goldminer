[根目录](../../../CLAUDE.md) > [src](../../) > **demo**

## Demo 模块

> **模块职责**: 游戏演示功能，包含玩家控制、动画、移动系统和关卡生成。这些模块主要用于演示目的，可以替换为实际的游戏逻辑。

## 入口与启动

模块通过 `plugin()` 函数注册到 Bevy 应用中，包含以下子插件：
- `animation::plugin` - 玩家动画系统
- `level::plugin` - 关卡生成和管理
- `movement::plugin` - 移动控制系统
- `player::plugin` - 玩家角色系统

## 对外接口

### 主要导出
- `pub mod level` - 关卡系统公开接口
- `pub mod player` - 玩家系统公开接口

### 核心函数
- `level::spawn_level()` - 关卡生成函数
- `player::player()` - 玩家实体生成函数

## 关键依赖与配置

### 外部依赖
- `bevy` - 游戏引擎核心
- `crate::asset_tracking` - 资源加载系统
- `crate::audio` - 音效系统
- `crate::AppSystems` - 应用系统集

### 配置文件
无特定配置文件，依赖资源通过 `LoadResource` trait 管理。

## 数据模型

### 核心组件
- `Player` - 玩家标记组件
- `PlayerAnimation` - 玩家动画控制
- `MovementController` - 移动控制器
- `ScreenWrap` - 屏幕边界处理

### 资源结构
- `PlayerAssets` - 玩家资源集合（图像、音效）
- `LevelAssets` - 关卡资源集合（音乐等）

## 测试与质量

### 测试覆盖
**暂无测试文件**。建议添加：
- 玩家移动逻辑测试
- 动画状态转换测试
- 资源加载测试

### 代码质量
- 遵循 Rust 最佳实践
- 使用 Bevy ECS 架构
- 模块化设计清晰

## 常见问题 (FAQ)

**Q: 如何修改玩家移动速度？**
A: 在调用 `player()` 函数时调整 `max_speed` 参数。

**Q: 如何添加新的玩家动画？**
A: 修改 `PlayerAnimation` 组件，更新纹理图集布局和动画逻辑。

**Q: 如何更换玩家角色？**
A: 替换 `PlayerAssets` 中的 `ducky` 图像资源和相关动画设置。

## 相关文件清单

| 文件 | 描述 | 重要性 |
|------|------|--------|
| `mod.rs` | 模块入口和插件注册 | ⭐⭐⭐ |
| `player.rs` | 玩家角色系统 | ⭐⭐⭐ |
| `level.rs` | 关卡生成系统 | ⭐⭐⭐ |
| `movement.rs` | 移动控制系统 | ⭐⭐ |
| `animation.rs` | 动画系统 | ⭐⭐ |

## 变更记录 (Changelog)

**2025-11-18 15:03:39** - 初始化 demo 模块文档，分析模块结构和接口