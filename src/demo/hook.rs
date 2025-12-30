use crate::asset_tracking::LoadResource;
use crate::audio::{AudioAssets, sound_effect};
use crate::config::EntityDescriptor;
use crate::demo::player::{PlayerAnimation, PlayerAnimationState, PlayerResource};
use crate::screens::Screen;
use crate::utils::love_to_bevy_coords;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HookAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hook);
    app.add_systems(
        Update,
        (handle_hook_input, update_hook, update_bonus_state)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

// --- 配置常量 ---
const HOOK_MIN_ANGLE: f32 = -75.0; // 最小角度 (-75度)
const HOOK_MAX_ANGLE: f32 = 75.0; // 最大角度 (75度)
const HOOK_ROTATE_SPEED: f32 = 65.0; // 旋转速度 (度/秒)
const HOOK_MAX_LENGTH: f32 = 460.0; // 最大伸出长度
const HOOK_GRAB_SPEED: f32 = 100.0; // 抓取速度 (像素/秒)
const HOOK_COLLISION_RADIUS: f32 = 6.0; // 钩子碰撞半径 (匹配 Lua)
const HOOK_COLLISION_OFFSET: f32 = 13.0; // 碰撞圆心偏移 (匹配 Lua)
const BONUS_DISPLAY_DURATION: f32 = 1.0; // 奖励显示时长 (秒)

// --- 动画帧索引 ---
const HOOK_ANIM_IDLE: usize = 0;
const HOOK_ANIM_GRAB_NORMAL: usize = 1;
const HOOK_ANIM_GRAB_MINI: usize = 2;

#[derive(Component)]
pub struct Hook {
    pub length: f32, // 改为 f32 以提高精度
    pub angle: f32,

    pub rotate_right: bool,
    pub is_grabing: bool,
    pub is_backing: bool,
    pub is_showing_bonus: bool, // 新增：是否正在显示奖励
    pub grabed_entity: Option<Entity>,

    pub bonus_timer: f32, // 改为 f32 以支持浮点计时
}

impl Default for Hook {
    fn default() -> Self {
        Self {
            length: 0.0,
            angle: HOOK_MAX_ANGLE,
            rotate_right: true,
            is_grabing: false,
            is_backing: false,
            is_showing_bonus: false,
            grabed_entity: None,
            bonus_timer: 0.0,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct HookAssets {
    #[dependency]
    hook: Handle<Image>,
}

impl FromWorld for HookAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hook: assets.load("images/hook_sheet.png"),
        }
    }
}

fn spawn_hook(
    mut commands: Commands,
    hook_assets: Res<HookAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(13, 15), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let base_pos = love_to_bevy_coords(158.0, 30.0);

    commands.spawn((
        Name::new("hook"),
        Hook::default(),
        Sprite::from_atlas_image(
            hook_assets.hook.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: HOOK_ANIM_IDLE,
            },
        ),
        Transform::from_translation(base_pos.extend(0.0)),
        Anchor::TOP_CENTER,
    ));
}

fn handle_hook_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut q_hook: Query<&mut Hook>,
) {
    if let Some(mut hook) = q_hook.iter_mut().next() {
        if input.just_pressed(KeyCode::Space)
            && !hook.is_grabing
            && !hook.is_backing
            && !hook.is_showing_bonus
        {
            hook.is_grabing = true;
            // 播放抓取开始音效
            if let Some(audio) = audio_assets.get_audio("GrabStart") {
                commands.spawn(sound_effect(audio));
            }
        }
    }
}

fn update_hook(
    time: Res<Time>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    player: Res<PlayerResource>,
    audio_assets: Res<AudioAssets>,
    mut query: Query<(&mut Hook, &mut Transform, &mut Sprite)>,
    q_entities: Query<
        (Entity, &GlobalTransform),
        (With<crate::config::LevelEntity>, Without<Hook>),
    >,
    q_descriptors: Query<&EntityDescriptor>,
    mut q_player_anim: Query<&mut PlayerAnimation>,
) {
    let rope_color = Color::srgb(66.0 / 255.0, 66.0 / 255.0, 66.0 / 255.0);
    let base_pos = love_to_bevy_coords(158.0, 30.0);

    for (mut hook, mut transform, mut sprite) in &mut query {
        // 奖励显示状态下跳过其他更新
        if hook.is_showing_bonus {
            continue;
        }

        // 绘制绳索
        if hook.is_grabing || hook.is_backing {
            let angle_rad = hook.angle.to_radians();
            let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());
            let perp = Vec2::new(-dir.y, dir.x);

            let end_pos = transform.translation.truncate() + dir * 4.0;

            gizmos.line_2d(base_pos, end_pos, rope_color);
            gizmos.line_2d(base_pos + perp * 0.5, end_pos + perp * 0.5, rope_color);
            gizmos.line_2d(base_pos - perp * 0.5, end_pos - perp * 0.5, rope_color);
        }

        if hook.is_grabing {
            // 切换玩家动画到 Grab 状态
            for mut player_anim in &mut q_player_anim {
                player_anim.update_state(PlayerAnimationState::Grab);
            }

            // 抓取逻辑：长度递增
            hook.length += time.delta_secs() * HOOK_GRAB_SPEED;

            let angle_rad = hook.angle.to_radians();
            let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());

            // 钩子末端位置 (用于渲染)
            let tip_pos = base_pos + dir * hook.length;
            transform.translation = tip_pos.extend(0.0);

            // 碰撞检测圆心位置 (末端 + 偏移)
            let collision_pos = base_pos + dir * (hook.length + HOOK_COLLISION_OFFSET);

            // 碰撞检测
            let mut collided = false;
            for (entity, entity_transform) in q_entities.iter() {
                let entity_pos = entity_transform.translation().truncate();
                // 使用 HOOK_COLLISION_RADIUS (6.0) 进行检测
                // 假设实体也有类似半径，这里简化为 HOOK_COLLISION_RADIUS * 2
                if collision_pos.distance(entity_pos) < HOOK_COLLISION_RADIUS * 2.0 {
                    hook.grabed_entity = Some(entity);
                    collided = true;

                    // 根据实体大小切换动画帧
                    if let Ok(descriptor) = q_descriptors.get(entity) {
                        // 简化判断：mass < 2.0 视为小物体
                        let is_tiny = descriptor.mass.unwrap_or(1.0) < 2.0;
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = if is_tiny {
                                HOOK_ANIM_GRAB_MINI
                            } else {
                                HOOK_ANIM_GRAB_NORMAL
                            };
                        }
                    }
                    break;
                }
            }

            // 屏幕边界检测 (虚拟屏幕 320x240，Bevy坐标系以中心为原点)
            let half_width = 160.0;
            let half_height = 120.0;
            let out_of_bounds = collision_pos.x < -half_width
                || collision_pos.x > half_width
                || collision_pos.y < -half_height
                || collision_pos.y > half_height;

            if collided || hook.length >= HOOK_MAX_LENGTH || out_of_bounds {
                hook.is_grabing = false;
                hook.is_backing = true;
                // 播放回缩音效
                if let Some(audio) = audio_assets.get_audio("GrabBack") {
                    commands.spawn(sound_effect(audio));
                }
            }
        } else if hook.is_backing {
            // 切换玩家动画到 GrabBack 状态（如非炸药状态）
            if !player.is_using_dynamite {
                for mut player_anim in &mut q_player_anim {
                    player_anim.update_state(PlayerAnimationState::GrabBack);
                }
            }

            // 回缩逻辑
            let mut speed = HOOK_GRAB_SPEED;
            if let Some(entity) = hook.grabed_entity {
                if let Ok(descriptor) = q_descriptors.get(entity) {
                    let mass = descriptor.mass.unwrap_or(1.0);
                    let strength = player.strength as f32;
                    speed = HOOK_GRAB_SPEED * strength / mass;
                }
            }

            hook.length -= time.delta_secs() * speed;

            if hook.length <= 0.0 {
                hook.length = 0.0;
                hook.is_backing = false;

                // 如果抓到了物体，进入奖励显示状态
                if hook.grabed_entity.is_some() {
                    hook.is_showing_bonus = true;
                    hook.bonus_timer = BONUS_DISPLAY_DURATION;
                } else {
                    // 无物体，重置动画
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = HOOK_ANIM_IDLE;
                    }
                    // 切换玩家动画回 Idle 状态
                    for mut player_anim in &mut q_player_anim {
                        player_anim.update_state(PlayerAnimationState::Idle);
                    }
                    // 播放重置音效
                    if let Some(audio) = audio_assets.get_audio("HookReset") {
                        commands.spawn(sound_effect(audio));
                    }
                }
            }

            let angle_rad = hook.angle.to_radians();
            let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());
            let tip_pos = base_pos + dir * hook.length;
            transform.translation = tip_pos.extend(0.0);
        } else {
            // 待机旋转逻辑
            if hook.rotate_right {
                hook.angle += HOOK_ROTATE_SPEED * time.delta_secs();
                if hook.angle >= HOOK_MAX_ANGLE {
                    hook.angle = HOOK_MAX_ANGLE;
                    hook.rotate_right = false;
                }
            } else {
                hook.angle -= HOOK_ROTATE_SPEED * time.delta_secs();
                if hook.angle <= HOOK_MIN_ANGLE {
                    hook.angle = HOOK_MIN_ANGLE;
                    hook.rotate_right = true;
                }
            }

            transform.rotation = Quat::from_rotation_z(hook.angle.to_radians());
            transform.translation = base_pos.extend(0.0);
        }
    }
}

/// 处理奖励显示状态的系统
fn update_bonus_state(
    time: Res<Time>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut stats: ResMut<crate::screens::stats::LevelStats>,
    mut query: Query<(&mut Hook, &mut Sprite)>,
    q_descriptors: Query<&EntityDescriptor>,
    mut entity_commands: Commands,
    mut q_player_anim: Query<&mut PlayerAnimation>,
) {
    for (mut hook, mut sprite) in &mut query {
        if !hook.is_showing_bonus {
            continue;
        }

        hook.bonus_timer -= time.delta_secs();

        // 奖励计时器结束
        if hook.bonus_timer <= 0.0 {
            // 结算金钱
            if let Some(entity) = hook.grabed_entity {
                if let Ok(descriptor) = q_descriptors.get(entity) {
                    let bonus = descriptor.bonus.unwrap_or(0);
                    stats.money += bonus as u32;

                    // 根据 bonus_type 播放对应音效
                    let sound_id = descriptor.bonus_type.as_deref().unwrap_or("Normal");
                    if let Some(audio) = audio_assets.get_audio(sound_id) {
                        commands.spawn(sound_effect(audio));
                    }

                    // TODO: 处理 extra_effect_chances 特殊效果

                    // 销毁被抓取的实体
                    entity_commands.entity(entity).despawn();
                }
            }

            // 重置钩子状态
            hook.grabed_entity = None;
            hook.is_showing_bonus = false;
            hook.bonus_timer = 0.0;

            // 重置动画帧
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = HOOK_ANIM_IDLE;
            }

            // 切换玩家动画回 Idle 状态
            for mut player_anim in &mut q_player_anim {
                player_anim.update_state(PlayerAnimationState::Idle);
            }

            // 播放重置音效
            if let Some(audio) = audio_assets.get_audio("HookReset") {
                commands.spawn(sound_effect(audio));
            }
        }
    }
}
