use crate::asset_tracking::LoadResource;
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
        (handle_hook_input, update_hook)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

const HOOK_MIN_ANGLE: f32 = -75.0; // 最小角度 (-75度)
const HOOK_MAX_ANGLE: f32 = 75.0; // 最大角度 (75度)
const HOOK_ROTATE_SPEED: f32 = 65.0; // 旋转速度 (度/秒)
const HOOK_MAX_LENGTH: f32 = 460.0; // 最大伸出长度
const HOOK_GRAB_SPEED: f32 = 100.0; //  抓取速度 (像素/秒)

#[derive(Component)]
pub struct Hook {
    pub length: i32,
    pub angle: f32,

    pub rotate_right: bool,
    pub is_grabing: bool,
    pub is_backing: bool,
    pub is_stop_moving: bool,
    pub grabed_entity: Option<Entity>,

    pub bonus_timer: i32,
    pub is_show_bonus: bool,
}

impl Default for Hook {
    fn default() -> Self {
        Self {
            length: 0,
            angle: HOOK_MAX_ANGLE,
            rotate_right: true,
            is_grabing: false,
            is_backing: false,
            is_stop_moving: false,
            grabed_entity: None,
            bonus_timer: 1,
            is_show_bonus: false,
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
                index: 0,
            },
        ),
        Transform::from_translation(base_pos.extend(0.0)),
        Anchor::TOP_CENTER,
    ));
}

fn handle_hook_input(input: Res<ButtonInput<KeyCode>>, mut q_hook: Query<&mut Hook>) {
    if let Some(mut hook) = q_hook.iter_mut().next() {
        if input.just_pressed(KeyCode::Space) && !hook.is_grabing && !hook.is_backing {
            hook.is_grabing = true;
        }
    }
}

fn update_hook(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(&mut Hook, &mut Transform)>,
    q_entities: Query<
        (Entity, &GlobalTransform),
        (With<crate::config::LevelEntity>, Without<Hook>),
    >,
) {
    let rope_color = Color::srgb(66.0 / 255.0, 66.0 / 255.0, 66.0 / 255.0);
    let base_pos = love_to_bevy_coords(158.0, 30.0);

    for (mut hook, mut transform) in &mut query {
        if hook.is_grabing || hook.is_backing {
            gizmos.line_2d(base_pos, transform.translation.truncate(), rope_color);
        }

        if hook.is_grabing {
            // 抓取逻辑：长度递增
            hook.length += (time.delta_secs() * HOOK_GRAB_SPEED) as i32;

            // 更新位置（基于长度和角度）
            let base_pos = love_to_bevy_coords(158.0, 30.0);
            let angle_rad = hook.angle.to_radians();
            let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());
            let tip_pos = base_pos + dir * hook.length as f32;
            transform.translation = tip_pos.extend(0.0);

            // 碰撞检测，半径为12
            let mut collided = false;
            for (entity, entity_transform) in q_entities.iter() {
                let entity_pos = entity_transform.translation().truncate();
                if tip_pos.distance(entity_pos) < 12.0 {
                    hook.grabed_entity = Some(entity);
                    collided = true;
                    break;
                }
            }

            if collided || hook.length as f32 >= HOOK_MAX_LENGTH {
                hook.is_grabing = false;
                hook.is_backing = true;
            }
        } else if hook.is_backing {
            // 回缩逻辑 (用户未细说，这里先简单实现以便测试)
            hook.length -= (time.delta_secs() * HOOK_GRAB_SPEED) as i32;
            if hook.length <= 0 {
                hook.length = 0;
                hook.is_backing = false;
                hook.grabed_entity = None;
            }
            let base_pos = love_to_bevy_coords(158.0, 30.0);
            let angle_rad = hook.angle.to_radians();
            let dir = Vec2::new(angle_rad.sin(), -angle_rad.cos());
            let tip_pos = base_pos + dir * hook.length as f32;
            transform.translation = tip_pos.extend(0.0);
        } else {
            // 旋转逻辑
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
            // 将角度转换为弧度并应用到旋转
            transform.rotation = Quat::from_rotation_z(hook.angle.to_radians());

            // 确保回到原点
            let base_pos = love_to_bevy_coords(158.0, 30.0);
            transform.translation = base_pos.extend(0.0);
        }
    }
}
