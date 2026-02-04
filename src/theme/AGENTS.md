# Theme 模块开发指南

## OVERVIEW
提供可重用的 UI 组件、主题色彩方案和交互效果，统一游戏的视觉风格。

## STRUCTURE
- `palette.rs`: 核心颜色常量定义，作为 UI 视觉的单一事实来源。
- `widget.rs`: 提供 `ui_root`, `header`, `button` 等 UI 构建块函数。
- `interaction.rs`: 封装 `InteractionPalette` 组件及自动处理悬停/点击颜色的系统。
- `mod.rs`: 导出 `prelude` 并注册插件。

## WHERE TO LOOK
- **调整全局配色**: 直接修改 `palette.rs` 中的颜色常量。
- **自定义组件样式**: 在 `widget.rs` 中定义新的构建函数，返回 `impl Bundle`。
- **扩展交互逻辑**: 在 `interaction.rs` 中增加对新交互状态的支持。

## CONVENTIONS
- **组件组合**: 优先使用 `widget.rs` 中的函数构建 UI，避免在业务代码中重复定义 `Node` 布局。
- **色彩引用**: 所有 UI 颜色必须引用 `palette.rs` 中的常量，严禁硬编码 `Color::srgb`。
- **模块导出**: 常用组件和类型应通过 `prelude` 统一对外暴露。

## ANTI-PATTERNS
- **样式泄露**: 在 `src/screens` 或 `src/menus` 中手动配置具体的像素值或颜色。
- **重复逻辑**: 为每个按钮手动编写 `Interaction` 改变颜色的系统（应使用 `InteractionPalette`）。
- **硬编码文本属性**: 直接设置 `TextFont` 大小而不通过 `theme` 中的语义化函数。
