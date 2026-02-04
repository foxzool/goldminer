# Menus 模块智能体指南

## OVERVIEW
管理游戏中的所有 UI 菜单界面及其状态转换逻辑。

## STRUCTURE
- `mod.rs`: 核心入口。定义了 `Menu` (界面级别) 和 `MenuSelect` (选项级别) 的状态枚举，并集中注册子模块插件。
- `main.rs`: 主菜单实现。处理标题界面的按钮布局 (Start Game, High Score) 以及键盘/手柄的选项切换与确认逻辑。
- `high_score.rs`: 最高分界面。从 `PersistentData` 读取并展示玩家的历史最高分及最高关卡记录。

## WHERE TO LOOK
- **新增菜单项**: 在 `mod.rs` 的 `Menu` 枚举中添加新状态，并在该目录下创建对应的 `.rs` 实现文件。
- **修改交互逻辑**: 检查 `main.rs` 或 `high_score.rs` 中的 `Update` 系统（如 `keyboard_input` 或 `go_back`）。
- **调整布局/位置**: 查找 `spawn_*_menu` 函数中使用的 `love_to_bevy_coords` 调用。

## CONVENTIONS
- **状态清理**: 必须为 UI 根实体及其子项添加 `DespawnOnExit(CurrentState)`，以保证退出菜单时自动销毁实体。
- **UI 构建**: 统一使用 `crate::theme::widget` 提供的辅助函数（如 `ui_root`）和 `children!` 宏。
- **坐标系**: 采用 Love2D 风格的坐标转换函数 `love_to_bevy_coords` 进行定位，以匹配原始设计。
- **层级管理**: 菜单根实体应设置 `GlobalZIndex(2)` 或更高，确保覆盖在游戏场景之上。

## ANTI-PATTERNS
- **硬编码颜色**: 严禁直接使用 `Color::rgb(...)`，应引用 `crate::constants` 中的颜色常量。
- **耦合业务逻辑**: 菜单系统仅负责切换状态（如 `NextState<Screen>`），不应直接修改玩家金钱或重置关卡。
- **重复输入检测**: 优先复用 `main.rs` 中的输入处理模式，避免在每个子页面编写完全独立且不一致的按键映射。
