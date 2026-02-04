# AGENTS.md (src/demo)

## 🎯 OVERVIEW
核心游戏玩法模块，实现钩子抓取、玩家动画、关卡生成及实体交互逻辑。

## 🏗️ STRUCTURE
- `mod.rs`: 插件入口，集成所有子模块插件。
- `hook.rs`: 核心逻辑。处理钩子摆动、发射、碰撞检测、回收及奖励结算。
- `player.rs`: 玩家视觉表现。管理玩家动画状态（Idle, Grab, GrabBack, UseDynamite）。
- `level.rs`: 关卡生命周期。负责关卡实体的生成与场景初始化。
- `entity.rs`: 实体工厂。定义黄金、石块、钻石等游戏物体的行为。
- `explosive.rs`: 爆炸系统。处理炸药特效及 TNT 连锁反应逻辑。

## 🔍 WHERE TO LOOK
- **修改钩子物理/速度**: 调整 `hook.rs` 中的 `HOOK_ROTATE_SPEED`, `HOOK_GRAB_SPEED` 等常量。
- **调整碰撞逻辑**: 参见 `hook.rs` 中的 `update_hook` 系统，搜索碰撞检测部分。
- **添加新物品类型**: 在 `entity.rs` 中定义生成逻辑，并在 `config/` 中配置相关属性。
- **玩家动画状态机**: 查阅 `player.rs` 中的 `PlayerAnimationState` 及相关系统。

## 📏 CONVENTIONS
- **状态驱动**: 所有系统应受 `Screen::Gameplay` 状态保护，并使用 `AppSystems::Update` 集。
- **自动清理**: 游戏内实体必须附加 `DespawnOnExit(Screen::Gameplay)`。
- **资源解耦**: 使用 `LoadResource` trait 加载模块私有资源（如 `HookAssets`）。
- **坐标同步**: 使用 `utils::love_to_bevy_coords` 将设计稿坐标映射到 Bevy 坐标系。

## ⚠️ ANTI-PATTERNS
- **状态混乱**: 禁止在 `hook.rs` 之外直接操控 `PlayerAnimation` 组件内部状态，应通过状态枚举更新。
- **硬编码计算**: 避免在 `update` 系统中直接写入魔数，应使用 `Hook` 组件的字段或模块常量。
- **忽略质量**: 结算逻辑（奖励加成）应集中在 `update_bonus_state`，不要散落在碰撞检测中。
- **手动销毁**: 避免手动遍历销毁实体，优先利用 `DespawnOnExit` 钩子或命令式 `despawn`。
