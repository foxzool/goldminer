[根目录](../../../CLAUDE.md) > [src](../../) > **theme**

## Theme 模块

> **模块职责**: 提供可重用的UI组件、主题色彩方案和交互效果，统一游戏的视觉风格和用户体验。

## 入口与启动

模块通过 `plugin()` 函数注册到 Bevy 应用中：
- 注册 `interaction::plugin` - 交互效果系统
- 提供预lude模块用于方便导入

## 对外接口

### 主要导出
- `prelude` 模块 - 常用导入的集合
- `widget` 模块 - UI组件库
- `palette` 模块 - 颜色方案
- `interaction` 模块 - 交互效果

### Prellude 导出
```rust
pub use super::{
    interaction::InteractionPalette,
    palette as ui_palette,
    widget
};
```

## 关键依赖与配置

### 外部依赖
- `bevy` - 游戏引擎核心
- 无其他模块依赖，作为UI基础设施模块

### 设计目标
- 提供一致的UI外观
- 支持可访问性需求
- 响应式交互反馈
- 易于扩展的主题系统

## 数据模型

### 颜色方案 (`palette.rs`)
- `LABEL_TEXT` - 标签文本颜色 (#ddd369)
- `HEADER_TEXT` - 标题文本颜色 (#fcfbcc)
- `BUTTON_TEXT` - 按钮文本颜色 (#ececec)
- `BUTTON_BACKGROUND` - 按钮背景色 (#4666bf)
- `BUTTON_HOVERED_BACKGROUND` - 按钮悬停色 (#6299d1)
- `BUTTON_PRESSED_BACKGROUND` - 按钮按下色 (#3d4999)

### 组件系统
- `InteractionPalette` - 交互状态的颜色管理
- 各种UI组件的样式和行为

## 测试与质量

### 测试覆盖
**暂无测试文件**。建议添加：
- 颜色对比度测试（可访问性）
- 组件渲染测试
- 交互状态转换测试
- 响应式布局测试

### 代码质量
- `#[allow(dead_code)]` 避免未使用工具的警告
- 模块化设计清晰
- 良好的文档和注释

## 常见问题 (FAQ)

**Q: 如何修改游戏的整体色彩方案？**
A: 编辑 `palette.rs` 中的颜色常量，所有使用这些颜色的UI组件会自动更新。

**Q: 如何创建自定义的UI组件？**
A: 在 `widget.rs` 中添加新的组件函数，复用现有的样式和交互模式。

**Q: 如何添加新的交互效果？**
A: 扩展 `interaction.rs` 中的系统，添加新的状态检测和视觉反馈。

## 相关文件清单

| 文件 | 描述 | 重要性 |
|------|------|--------|
| `mod.rs` | 模块入口、插件注册、prelude | ⭐⭐⭐ |
| `palette.rs` | 颜色方案定义 | ⭐⭐⭐ |
| `widget.rs` | UI组件库 | ⭐⭐⭐ |
| `interaction.rs` | 交互效果系统 | ⭐⭐ |

## 变更记录 (Changelog)

**2025-11-18 15:03:39** - 初始化 theme 模块文档，分析UI组件系统和主题设计