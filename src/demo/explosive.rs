//! TNT 爆炸机制模块
//!
//! 实现地图 TNT 实体的爆炸效果：
//! - 钩子碰撞 TNT 时触发爆炸
//! - 爆炸产生范围伤害，销毁周围实体
//! - TNT 可引爆其他 TNT (连锁反应)

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::AppSystems;
use crate::audio::{AudioAssets, sound_effect};
use crate::config::{EntityDescriptor, ImageAssets};
use crate::screens::Screen;

/// 爆炸半径 (与 Lua 版 biggerExplosiveFX 对齐)
const EXPLOSION_RADIUS: f32 = 35.0 / 2.0;

/// 爆炸特效动画帧时长 (秒)
const EXPLOSION_FRAME_DURATION: f32 = 0.06;

/// 爆炸特效总帧数
const EXPLOSION_FRAME_COUNT: usize = 8;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            explosion_trigger_system,
            explosion_fx_system,
            explosion_damage_system,
        )
            .chain()
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// TNT 爆炸状态组件
#[derive(Component, Default)]
pub struct ExplosiveState {
    /// 是否正在爆炸
    pub is_exploding: bool,
    /// 是否已触发过范围伤害 (防止重复触发)
    pub damage_dealt: bool,
}

/// 爆炸特效组件
#[derive(Component)]
pub struct ExplosionFX {
    /// 动画计时器
    pub timer: Timer,
    /// 当前帧索引
    pub current_frame: usize,
    /// 爆炸中心位置 (用于范围伤害检测)
    pub center: Vec2,
}

impl ExplosionFX {
    pub fn new(center: Vec2) -> Self {
        Self {
            timer: Timer::from_seconds(EXPLOSION_FRAME_DURATION, TimerMode::Repeating),
            current_frame: 0,
            center,
        }
    }
}

/// 爆炸触发系统：当 TNT 的 is_exploding 被设为 true 时，生成爆炸特效
fn explosion_trigger_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    image_assets: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut q_explosives: Query<(Entity, &mut ExplosiveState, &GlobalTransform), With<ExplosiveState>>,
) {
    for (_entity, mut state, transform) in q_explosives.iter_mut() {
        if state.is_exploding && !state.damage_dealt {
            let center = transform.translation().truncate();

            // 播放爆炸音效
            if let Some(audio) = audio_assets.get_audio("Explosive") {
                commands.spawn(sound_effect(audio));
            }

            // 生成爆炸特效实体
            if let Some(fx_image) = image_assets.get_image("BiggerExplosiveFX") {
                // bigger_explosive_fx_sheet.png: 4x2 帧，每帧 35x35
                let layout = TextureAtlasLayout::from_grid(UVec2::new(35, 35), 4, 2, None, None);
                let atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((
                    Name::new("ExplosionFX"),
                    ExplosionFX::new(center),
                    Sprite::from_atlas_image(
                        fx_image,
                        TextureAtlas {
                            layout: atlas_layout,
                            index: 0,
                        },
                    ),
                    Transform::from_translation(center.extend(10.0)), // 高 z-index 确保在顶层
                    Anchor::CENTER,
                    DespawnOnExit(Screen::Gameplay),
                ));
            }

            // 标记已触发，防止重复生成特效
            state.damage_dealt = true;
        }
    }
}

/// 爆炸特效动画系统：更新帧索引，动画结束后销毁特效实体
fn explosion_fx_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_fx: Query<(Entity, &mut ExplosionFX, &mut Sprite)>,
) {
    for (entity, mut fx, mut sprite) in q_fx.iter_mut() {
        fx.timer.tick(time.delta());

        if fx.timer.just_finished() {
            fx.current_frame += 1;

            if fx.current_frame >= EXPLOSION_FRAME_COUNT {
                // 动画结束，销毁特效实体
                commands.entity(entity).despawn();
            } else {
                // 更新图集索引
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = fx.current_frame;
                }
            }
        }
    }
}

/// 爆炸范围伤害系统：对范围内实体造成伤害
fn explosion_damage_system(
    mut commands: Commands,
    q_fx: Query<&ExplosionFX>,
    mut q_explosives: Query<(Entity, &mut ExplosiveState, &GlobalTransform)>,
    q_entities: Query<
        (Entity, &GlobalTransform, &EntityDescriptor),
        (With<crate::config::LevelEntity>, Without<ExplosiveState>),
    >,
) {
    // 收集所有活跃的爆炸中心
    let explosion_centers: Vec<Vec2> = q_fx.iter().map(|fx| fx.center).collect();

    if explosion_centers.is_empty() {
        return;
    }

    // 检测范围内的普通实体
    for (entity, transform, descriptor) in q_entities.iter() {
        let entity_pos = transform.translation().truncate();
        let entity_radius = descriptor.collision_radius.unwrap_or(6.0);

        for center in &explosion_centers {
            if center.distance(entity_pos) < (EXPLOSION_RADIUS + entity_radius) {
                // 销毁普通实体
                commands.entity(entity).despawn();
                break;
            }
        }
    }

    // 检测范围内的其他 TNT (连锁反应)
    let explosive_positions: Vec<(Entity, Vec2)> = q_explosives
        .iter()
        .map(|(e, _, t)| (e, t.translation().truncate()))
        .collect();

    for (entity, mut state, _) in q_explosives.iter_mut() {
        if state.is_exploding {
            continue; // 已在爆炸中的 TNT 跳过
        }

        let entity_pos = explosive_positions
            .iter()
            .find(|(e, _)| *e == entity)
            .map(|(_, pos)| *pos)
            .unwrap_or(Vec2::ZERO);

        for center in &explosion_centers {
            if center.distance(entity_pos) < (EXPLOSION_RADIUS + 6.0) {
                // 触发连锁爆炸
                state.is_exploding = true;
                break;
            }
        }
    }
}
