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
        update_hook
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
        Transform::from(Transform::from_translation(
            love_to_bevy_coords(158.0, 30.0).extend(0.0),
        )),
        Anchor::TOP_CENTER,
    ));
}

fn update_hook(time: Res<Time>, mut query: Query<(&mut Hook, &mut Transform)>) {
    for (mut hook, mut transform) in &mut query {
        if !hook.is_grabing && !hook.is_backing {
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
        }
    }
}
