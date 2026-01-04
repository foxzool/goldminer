use crate::AppSystems;
use crate::asset_tracking::LoadResource;
use crate::audio::{AudioAssets, sound_effect};
use crate::config::{EntityDescriptor, EntityType, ImageAssets};
use crate::constants::COLOR_GREEN;
use crate::demo::explosive::ExplosiveState;
use crate::demo::player::{PlayerAnimation, PlayerAnimationState, PlayerResource};
use crate::screens::Screen;
use crate::utils::love_to_bevy_coords;
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
            .run_if(in_state(Screen::Gameplay)),
    );
}

// --- 配置常量 ---
const HOOK_MIN_ANGLE: f32 = -75.0; // 最小角度 (-75度)
const HOOK_MAX_ANGLE: f32 = 75.0; // 最大角度 (75度)
const HOOK_ROTATE_SPEED: f32 = 65.0; // 旋转速度 (度/秒)
const HOOK_MAX_LENGTH: f32 = 230.0; // 最大伸出长度 (对齐 Lua)
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
    pub is_showing_bonus: bool, // 是否正在显示奖励
    pub grabed_entity: Option<Entity>,

    pub bonus_timer: f32,
    pub current_bonus: i32,  // 当前奖励金额
    pub show_strength: bool, // 是否显示力量增强图标
}

#[derive(Component)]
struct BonusText;

#[derive(Component)]
struct StrengthIcon;

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
            current_bonus: 0,
            show_strength: false,
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
        ZIndex(10),
        Transform::from_translation(base_pos.extend(0.0)),
        Anchor::TOP_CENTER,
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn handle_hook_input(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut q_hook: Query<(&mut Hook, &Transform, &mut Sprite)>,
    mut player: ResMut<PlayerResource>,
    stats: Res<crate::screens::stats::LevelStats>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut q_player_anim: Query<&mut PlayerAnimation>,
    image_assets: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut fire = input.just_pressed(KeyCode::ArrowDown)
        || input.just_pressed(KeyCode::KeyJ)
        || input.just_pressed(KeyCode::KeyK);
    let mut use_dynamite = input.just_pressed(KeyCode::ArrowUp)
        || input.just_pressed(KeyCode::KeyU)
        || input.just_pressed(KeyCode::KeyI);
    let mut skip = input.just_pressed(KeyCode::Space);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadDown)
            || gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::East)
        {
            fire = true;
        }
        if gamepad.just_pressed(GamepadButton::DPadUp)
            || gamepad.just_pressed(GamepadButton::West)
            || gamepad.just_pressed(GamepadButton::North)
        {
            use_dynamite = true;
        }
        if gamepad.just_pressed(GamepadButton::Select) {
            skip = true;
        }
    }

    if let Some((mut hook, transform, mut sprite)) = q_hook.iter_mut().next() {
        // 1. 发射钩子
        if fire && !hook.is_grabing && !hook.is_backing && !hook.is_showing_bonus {
            hook.is_grabing = true;
            if let Some(audio) = audio_assets.get_audio("GrabStart") {
                commands.spawn(sound_effect(audio));
            }
        }

        // 2. 使用炸药
        if use_dynamite
            && hook.is_backing
            && hook.grabed_entity.is_some()
            && player.dynamite_count > 0
            && !player.is_using_dynamite
        {
            player.dynamite_count -= 1;
            player.is_using_dynamite = true;
            player.using_dynamite_timer = 0.39;

            // 切换玩家动画
            for mut anim in &mut q_player_anim {
                anim.update_state(PlayerAnimationState::UseDynamite);
            }

            // 播放炸药生效音效 (Lua 版使用 Dynamite 音效)
            if let Some(audio) = audio_assets.get_audio("Explosive") {
                commands.spawn(sound_effect(audio));
            }

            // 在钩子位置产生爆炸特效
            let center = transform.translation.truncate();
            if let Some(fx_image) = image_assets.get_image("BiggerExplosiveFX") {
                let layout = TextureAtlasLayout::from_grid(UVec2::new(35, 35), 4, 2, None, None);
                let atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((
                    Name::new("DynamiteExplosionFX"),
                    crate::demo::explosive::ExplosionFX::new(center),
                    Sprite::from_atlas_image(
                        fx_image,
                        TextureAtlas {
                            layout: atlas_layout,
                            index: 0,
                        },
                    ),
                    Transform::from_translation(center.extend(10.0)),
                    Anchor::CENTER,
                    DespawnOnExit(Screen::Gameplay),
                ));
            }

            // 销毁被抓取的物品
            if let Some(entity) = hook.grabed_entity {
                commands.entity(entity).despawn();
                hook.grabed_entity = None;
            }

            // 钩子变回空载状态
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = HOOK_ANIM_IDLE;
            }
            // 钩子回缩速度变回正常 (因为物品没了)
        }

        // 3. 跳过关卡
        if skip && stats.reach_goal() {
            next_screen.set(Screen::MadeGoal);
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
    mut q_explosives: Query<&mut ExplosiveState>,
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
            transform.rotation = Quat::from_rotation_z(angle_rad);

            // 碰撞检测圆心位置 (末端 + 偏移)
            let collision_pos = base_pos + dir * (hook.length + HOOK_COLLISION_OFFSET);

            // 碰撞检测
            let mut collided = false;
            for (entity, entity_transform) in q_entities.iter() {
                let entity_pos = entity_transform.translation().truncate();

                // 获取实体的碰撞半径，默认为 HOOK_COLLISION_RADIUS
                let mut entity_radius = HOOK_COLLISION_RADIUS;
                if let Ok(descriptor) = q_descriptors.get(entity) {
                    if let Some(radius) = descriptor.collision_radius {
                        entity_radius = radius;
                    }
                }

                // 碰撞判定：当两圆心距离小于半径之和时发生碰撞 (对齐 Lua)
                if collision_pos.distance(entity_pos) < (HOOK_COLLISION_RADIUS + entity_radius) {
                    hook.grabed_entity = Some(entity);
                    collided = true;

                    // 根据实体大小切换动画帧
                    if let Ok(descriptor) = q_descriptors.get(entity) {
                        // TNT 爆炸处理：碰撞时触发爆炸，使用 is_destroyed_tiny 判定动画
                        if descriptor.entity_type == EntityType::Explosive {
                            // 触发爆炸
                            if let Ok(mut explosive_state) = q_explosives.get_mut(entity) {
                                explosive_state.is_exploding = true;
                            }
                            // 使用 is_destroyed_tiny 配置判断动画帧
                            let is_tiny = descriptor.is_destroyed_tiny.unwrap_or(true);
                            if let Some(atlas) = &mut sprite.texture_atlas {
                                atlas.index = if is_tiny {
                                    HOOK_ANIM_GRAB_MINI
                                } else {
                                    HOOK_ANIM_GRAB_NORMAL
                                };
                            }
                        } else {
                            // 普通实体：简化判断 mass < 2.0 视为小物体
                            let is_tiny = descriptor.mass.unwrap_or(1.0) < 2.0;
                            if let Some(atlas) = &mut sprite.texture_atlas {
                                atlas.index = if is_tiny {
                                    HOOK_ANIM_GRAB_MINI
                                } else {
                                    HOOK_ANIM_GRAB_NORMAL
                                };
                            }
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
                    let mut mass = descriptor.mass.unwrap_or(1.0);
                    // 力量饮料效果：质量 ÷ 1.5
                    if player.has_strength_drink {
                        mass /= 1.5;
                    }
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
            transform.rotation = Quat::from_rotation_z(angle_rad);
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
    asset_server: Res<AssetServer>,
    audio_assets: Res<AudioAssets>,
    image_assets: Res<ImageAssets>,
    mut stats: ResMut<crate::screens::stats::LevelStats>,
    mut query: Query<(&mut Hook, &mut Sprite)>,
    q_descriptors: Query<&EntityDescriptor>,
    q_level_entities: Query<&crate::config::LevelEntity>,
    mut q_player_anim: Query<&mut PlayerAnimation>,
    mut q_transforms: Query<&mut Transform, Without<Hook>>,
    mut player: ResMut<PlayerResource>,
    q_bonus_text: Query<Entity, With<BonusText>>,
    q_strength_icon: Query<Entity, With<StrengthIcon>>,
) {
    let base_pos = love_to_bevy_coords(158.0, 30.0);

    for (mut hook, mut sprite) in &mut query {
        // 如果正在抓取物体，同步物体位置和旋转 (对齐 Lua)
        if let Some(entity) = hook.grabed_entity {
            if let Ok(mut transform) = q_transforms.get_mut(entity) {
                let angle_rad = hook.angle.to_radians();
                let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());
                let collision_pos = base_pos + dir * (hook.length + HOOK_COLLISION_OFFSET);

                transform.translation = collision_pos.extend(1.0);
                transform.rotation = Quat::from_rotation_z(angle_rad);
            }
        }

        if !hook.is_showing_bonus {
            continue;
        }

        // 首帧进入奖励状态: 结算并显示 UI
        if hook.bonus_timer == BONUS_DISPLAY_DURATION {
            if let Some(entity) = hook.grabed_entity {
                if let Ok(descriptor) = q_descriptors.get(entity) {
                    let mut bonus = descriptor.bonus.unwrap_or(0);
                    let sound_id = descriptor.bonus_type.as_deref().unwrap_or("Normal");

                    // 获取实体 ID 以判断类型
                    let entity_id = q_level_entities
                        .get(entity)
                        .map(|le| le.entity_id.as_str())
                        .unwrap_or("");

                    // 石头收藏书效果：岩石价值 ×3
                    if player.has_rock_collectors_book {
                        if matches!(entity_id, "MiniRock" | "NormalRock" | "BigRock") {
                            bonus *= 3;
                        }
                    }

                    // 宝石抛光剂效果：钻石价值 ×1.5
                    if player.has_gem_polish {
                        if entity_id == "Diamond" {
                            bonus = (bonus as f32 * 1.5) as i32;
                        } else if entity_id == "MoleWithDiamond" {
                            // MoleWithDiamond 特殊处理：只对钻石部分加成
                            // bonus = (bonus - mole_bonus) * 1.5 + mole_bonus
                            // 暂时简化处理：整体 ×1.5
                            bonus = (bonus as f32 * 1.5) as i32;
                        }
                    }

                    // 幸运草效果：翻倍 extra_effect_chances
                    let mut chances = descriptor.extra_effect_chances.unwrap_or(0.0);
                    if player.has_lucky_clover {
                        chances *= 2.0;
                    }

                    // 处理 extra_effect_chances 特殊效果 (对齐 Lua)
                    if chances > 0.0 {
                        let rand_val = rand::random::<f32>();
                        if rand_val < chances {
                            // 20% 概率增加炸药，80% 概率增加玩家力量
                            if rand::random::<f32>() < 0.2 {
                                player.dynamite_count += 1;
                            } else {
                                // 力量增加，最大值为 6
                                player.strength = (player.strength + 1).min(6);
                                hook.show_strength = true;
                                // Spawn Strength! 图标 - 位置 (80, 10) → Bevy 换算
                                if let Some(strength_img) = image_assets.get_image("Strength!") {
                                    commands.spawn((
                                        StrengthIcon,
                                        Sprite::from_image(strength_img),
                                        Transform::from_translation(
                                            love_to_bevy_coords(80.0, 10.0).extend(10.0),
                                        ),
                                    ));
                                }
                            }
                            if let Some(audio) = audio_assets.get_audio("High") {
                                commands.spawn(sound_effect(audio));
                            }
                        } else {
                            // 正常奖励
                            hook.current_bonus = bonus;
                            stats.money += bonus as u32;
                            if let Some(audio) = audio_assets.get_audio(sound_id) {
                                commands.spawn(sound_effect(audio));
                            }
                        }
                    } else {
                        // 无特殊效果概率，正常奖励
                        hook.current_bonus = bonus;
                        stats.money += bonus as u32;
                        if let Some(audio) = audio_assets.get_audio(sound_id) {
                            commands.spawn(sound_effect(audio));
                        }
                    }

                    // 如果有奖励金额，spawn 显示文本
                    if hook.current_bonus > 0 {
                        let font = asset_server.load("fonts/Kurland.ttf");
                        commands.spawn((
                            BonusText,
                            Text::new(format!("${}", hook.current_bonus)),
                            TextFont {
                                font,
                                font_size: 64.0, // Lua 16pt * 4 = 64
                                ..default()
                            },
                            TextColor(COLOR_GREEN),
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(72.0),   // Lua y=18 -> 18*4=72
                                left: px(360.0), // Lua x=90 -> 90*4=360
                                ..default()
                            },
                        ));
                    }

                    // 销毁被抓取的实体
                    commands.entity(entity).despawn();
                }
            }
        }

        hook.bonus_timer -= time.delta_secs();

        // 奖励计时器结束
        if hook.bonus_timer <= 0.0 {
            // 清理显示元素
            for entity in &q_bonus_text {
                commands.entity(entity).despawn();
            }
            for entity in &q_strength_icon {
                commands.entity(entity).despawn();
            }

            // 重置钩子状态
            hook.grabed_entity = None;
            hook.is_showing_bonus = false;
            hook.bonus_timer = 0.0;
            hook.current_bonus = 0;
            hook.show_strength = false;

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
