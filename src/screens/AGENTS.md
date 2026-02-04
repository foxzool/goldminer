# Screens Agent Guide

## OVERVIEW
负责管理游戏的所有顶级屏幕状态（States）定义、场景切换逻辑及各屏幕专属的 UI 初始化。

## STRUCTURE
- `mod.rs`: 定义 `Screen` 状态枚举、全局退出逻辑及各屏幕插件注册。
- `gameplay.rs`: 核心游戏屏幕逻辑，处理游戏进行时的暂停覆盖层。
- `loading.rs`: 资源异步加载界面，监控加载进度并自动转换状态。
- `splash.rs`: 启动动画实现，包含渐入渐出效果及定时跳转。
- `title.rs`: 主标题界面，处理菜单项的选择与导航。
- `shop.rs` / `game_over.rs`: 循环流程中的商店界面与游戏结束结算界面。
- `persistent.rs`: 负责持久化数据（如最高分）的加载与保存。
- `stats.rs`: 跟踪当前关卡和全局的游戏统计数据。

## WHERE TO LOOK
- **状态定义**: 查看 `mod.rs` 中的 `Screen` 枚举及其插件注册顺序。
- **场景切换**: 搜索 `NextState<Screen>` 的引用以定位跳转触发点。
- **UI 挂载**: 寻找 `spawn_..._screen` 函数，通常与 `OnEnter` 钩子绑定。

## CONVENTIONS
- **插件化**: 每个子模块必须导出一个 `plugin(app: &mut App)` 函数。
- **自动清理**: 挂载 UI 实体时务必添加 `DespawnOnExit(Screen::...)` 组件，确保屏幕切换时资源正确销毁。
- **状态约束**: `Update` 阶段的屏幕专属系统必须添加 `.run_if(in_state(Screen::...))` 运行条件。
- **UI 风格**: 使用 `theme` 模块提供的 `widget` 工具函数（如 `ui_root`）构建界面。

## ANTI-PATTERNS
- **状态泄漏**: 在 `Update` 中运行系统却未加状态过滤，导致逻辑在非预期屏幕生效。
- **逻辑耦合**: 在屏幕模块内直接修改关卡数据，应通过 `Resource` 或 `Event` 进行解耦。
- **清理不全**: 仅清理了实体但遗忘了残留的 `Resource` 或单次 `Timer`（参考 `SplashTimer` 的移除逻辑）。
