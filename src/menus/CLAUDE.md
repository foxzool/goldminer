[根目录](../../../CLAUDE.md) > [src](../../) > **menus**

## Menus 模块

> **模块职责**: 管理游戏中的所有菜单界面和状态转换，包括主菜单、设置、暂停菜单和制作人员界面。

## 入口与启动

模块通过 `plugin()` 函数注册到 Bevy 应用中：
- 初始化 `Menu` 状态枚举
- 注册所有子菜单插件：
  - `credits::plugin` - 制作人员菜单
  - `main::plugin` - 主菜单
  - `settings::plugin` - 设置菜单
  - `pause::plugin` - 暂停菜单

## 对外接口

### 状态枚举
```rust
pub enum Menu {
    None,      // 无菜单状态
    Main,      // 主菜单
    Credits,   // 制作人员
    Settings,  // 设置菜单
    Pause,     // 暂停菜单
}
```

### 主要导出
- `Menu` 状态枚举 - 菜单状态管理
- 各子菜单插件系统

## 关键依赖与配置

### 外部依赖
- `bevy` - 游戏引擎核心
- `crate::theme::widget` - UI 组件库
- `crate::asset_tracking::ResourceHandles` - 资源状态查询
- `crate::screens::Screen` - 屏幕状态管理

### 平台特性
- Web 平台隐藏退出选项
- 原生平台包含完整的退出功能

## 数据模型

### 状态管理
- `Menu` 枚举 - 菜单状态机
- 使用 Bevy States 进行状态转换
- 与 `Screen` 状态协同工作

### UI 组件
- 依赖 `theme::widget` 模块的按钮组件
- 使用 `DespawnOnExit` 进行状态清理
- 支持全局 Z-Index 层级管理

## 测试与质量

### 测试覆盖
**暂无测试文件**。建议添加：
- 菜单状态转换测试
- 按钮交互测试
- 跨平台行为测试

### 代码质量
- 状态驱动的架构设计
- 平台特定代码的条件编译
- 清晰的模块分离

## 常见问题 (FAQ)

**Q: 如何添加新的菜单项？**
A: 在相应的子菜单文件中使用 `widget::button()` 添加新按钮，并关联相应的处理函数。

**Q: 如何修改菜单的外观？**
A: 修改 `theme/palette.rs` 中的颜色定义或 `theme/widget.rs` 中的组件样式。

**Q: 如何处理菜单间的转换？**
A: 使用 `NextState<Menu>` 来触发状态转换，Bevy 会自动处理状态的进入和退出。

## 相关文件清单

| 文件 | 描述 | 重要性 |
|------|------|--------|
| `mod.rs` | 模块入口、状态定义、插件注册 | ⭐⭐⭐ |
| `main.rs` | 主菜单实现 | ⭐⭐⭐ |
| `pause.rs` | 暂停菜单实现 | ⭐⭐ |
| `settings.rs` | 设置菜单实现 | ⭐⭐ |
| `credits.rs` | 制作人员界面实现 | ⭐ |

## 变更记录 (Changelog)

**2025-11-18 15:03:39** - 初始化 menus 模块文档，分析菜单状态管理和UI交互