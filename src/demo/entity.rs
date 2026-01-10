use crate::config::{EntityDescriptor, EntityType, LevelEntity};
use crate::demo::hook::Hook;
use crate::screens::Screen;
use bevy::prelude::*;

/// 注册实体巡逻系统插件
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            init_patrol_system,
            patrol_movement_system,
            entity_animation_system,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// 实体动画状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EntityAnimationState {
    #[default]
    Idle,
    Move,
}

/// 实体动画组件
#[derive(Component)]
pub struct EntityAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub state: EntityAnimationState,
    pub idle_frames: Vec<usize>,
    pub move_frames: Vec<usize>,
}

impl EntityAnimation {
    /// 创建新的实体动画组件
    /// frame_duration: 帧间隔秒数
    /// idle_frames: 待机状态的帧索引序列
    /// move_frames: 移动状态的帧索引序列
    pub fn new(frame_duration: f32, idle_frames: Vec<usize>, move_frames: Vec<usize>) -> Self {
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            current_frame: 0,
            state: EntityAnimationState::Move, // 默认初始状态为移动，与 patrol 逻辑一致
            idle_frames,
            move_frames,
        }
    }
}

/// 更新实体动画帧索引
fn entity_animation_system(
    time: Res<Time>,
    mut q_anim: Query<(&mut EntityAnimation, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in q_anim.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            let len = match anim.state {
                EntityAnimationState::Idle => anim.idle_frames.len(),
                EntityAnimationState::Move => anim.move_frames.len(),
            };

            if len > 0 {
                anim.current_frame = (anim.current_frame + 1) % len;
                let frame_idx = anim.current_frame;

                let tex_index = match anim.state {
                    EntityAnimationState::Idle => anim.idle_frames[frame_idx],
                    EntityAnimationState::Move => anim.move_frames[frame_idx],
                };

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = tex_index;
                }
            }
        }
    }
}

/// 巡逻状态组件，用于管理可移动实体的巡逻行为
#[derive(Component)]
pub struct PatrolState {
    /// 是否正在移动
    pub is_moving: bool,
    /// 移动方向，1.0 表示向右，-1.0 表示向左
    pub direction: f32,
    /// 目标位置的 x 坐标
    pub destination_x: f32,
    /// 闲置计时器
    pub idle_timer: Timer,
    /// 移动范围
    pub move_range: f32,
}

/// 初始化巡逻系统，为可移动实体添加巡逻状态组件
fn init_patrol_system(
    mut commands: Commands,
    mut q_added_entities: Query<
        (Entity, &LevelEntity, &EntityDescriptor, Option<&mut Sprite>),
        Added<LevelEntity>,
    >,
) {
    for (entity, level_entity, descriptor, mut sprite) in q_added_entities.iter_mut() {
        // 只为需要移动的实体添加巡逻状态
        if descriptor.entity_type == EntityType::MoveAround {
            // 获取移动范围，默认 135 像素
            let move_range = descriptor.move_range.unwrap_or(135.0);

            // 确定初始移动方向，从配置读取或默认向右 (1.0)
            let direction = match level_entity.dir {
                Some(crate::config::Direction::Left) => -1.0,
                _ => 1.0,
            };

            // 设置初始朝向（图片默认朝左）
            if let Some(ref mut s) = sprite {
                s.flip_x = direction > 0.0;
            }

            // 将 LÖVE 坐标转换为 Bevy 坐标，获取当前 x 位置
            let current_x =
                crate::utils::love_to_bevy_coords(level_entity.pos.x, level_entity.pos.y).x;
            // 计算目标位置
            let destination_x = current_x + (direction * move_range);

            // 为实体添加巡逻状态组件
            commands.entity(entity).insert(PatrolState {
                is_moving: true,
                direction,
                destination_x,
                idle_timer: Timer::from_seconds(1.0, TimerMode::Once),
                move_range,
            });
        }
    }
}

/// 处理实体的巡逻移动逻辑
fn patrol_movement_system(
    time: Res<Time>,
    mut q_patrol: Query<(
        Entity,
        &mut Transform,
        &mut PatrolState,
        &EntityDescriptor,
        &mut Sprite,
        Option<&mut EntityAnimation>,
    )>,
    q_hooks: Query<&Hook>,
) {
    for (entity, mut transform, mut state, descriptor, mut sprite, mut animation) in
        q_patrol.iter_mut()
    {
        // 如果实体被钩子抓取，跳过巡逻更新
        let mut is_grabbed = false;
        for hook in q_hooks.iter() {
            if hook.grabed_entity == Some(entity) {
                is_grabbed = true;
                break;
            }
        }
        if is_grabbed {
            continue;
        }

        if state.is_moving {
            // 确保动画状态为移动
            if let Some(anim) = &mut animation
                && anim.state != EntityAnimationState::Move {
                    anim.state = EntityAnimationState::Move;
                    anim.current_frame = 0;
                    // 立即更新图集索引
                    if !anim.move_frames.is_empty()
                        && let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = anim.move_frames[0];
                        }
                }

            // 计算移动速度：配置中的速度通常是每帧像素数，需要乘以 60 转换为每秒像素数
            let speed = descriptor.speed.unwrap_or(1.0) * 60.0;

            // 计算移动增量并更新位置
            let delta = speed * state.direction * time.delta_secs();
            transform.translation.x += delta;

            // 检查是否到达目标位置
            // 向右移动时检查是否超过目标 x，向左移动时检查是否小于目标 x
            let reached = if state.direction > 0.0 {
                transform.translation.x >= state.destination_x
            } else {
                transform.translation.x <= state.destination_x
            };

            if reached {
                // 到达目标位置，固定到精确位置
                transform.translation.x = state.destination_x;
                state.is_moving = false;
                state.idle_timer.reset();
                // 反转方向
                state.direction *= -1.0;
                // 计算下一个目标位置
                state.destination_x =
                    transform.translation.x + (state.direction * state.move_range);
            }

            // 更新精灵朝向（图片默认朝左）
            sprite.flip_x = state.direction > 0.0;
        } else {
            // 确保动画状态为待机
            if let Some(anim) = &mut animation
                && anim.state != EntityAnimationState::Idle {
                    anim.state = EntityAnimationState::Idle;
                    anim.current_frame = 0;
                    // 立即更新图集索引
                    if !anim.idle_frames.is_empty()
                        && let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = anim.idle_frames[0];
                        }
                }

            // 闲置状态，更新计时器
            state.idle_timer.tick(time.delta());
            // 闲置结束后开始移动
            if state.idle_timer.is_finished() {
                state.is_moving = true;
            }
        }
    }
}
