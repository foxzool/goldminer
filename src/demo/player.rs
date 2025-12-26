//! Player-specific behavior.

use crate::screens::Screen;
use crate::utils::love_to_bevy_coords;
use crate::{
    asset_tracking::LoadResource, demo::{animation::PlayerAnimation, movement::MovementController},
    AppSystems,
    PausableSystems,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.init_resource::<PlayerResource>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
}

/// The player character.
fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.init_resource::<PlayerResource>();
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 40), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let _player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        PlayerMarker,
        Sprite::from_atlas_image(
            player_assets.miner.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_translation(love_to_bevy_coords(165.0, 39.0).extend(0.0)),
        Anchor::BOTTOM_CENTER,
    ));
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
struct PlayerResource {
    /// 当前关卡数
    level: i32,
    /// 对应的关卡配置 ID (如 'L1_1')
    real_level: String,
    /// 玩家当前实际拥有的原始金钱
    money: i32,
    /// 当前关卡需要达到的目标金额
    goal: i32,
    /// 每一关增加的目标金额幅度
    goal_add_on: i32,

    /// 玩家的基础力量倍率
    strength: i32,
    /// 拥有的炸药数量
    dynamite_count: i32,
    /// 是否持有力量药水
    has_strength_drink: bool,
    /// 是否持有幸运草
    has_lucky_clover: bool,
    /// 是否持有宝石抛光剂
    has_gem_polish: bool,

    /// 标识玩家是否正在扔炸药
    is_using_dynamite: bool,
    /// 扔炸药动作的倒计时
    using_dynamite_timer: f32,
}

impl Default for PlayerResource {
    fn default() -> Self {
        Self {
            level: 1,
            real_level: "LDEBUG".to_string(),
            money: 375,
            goal: 275,
            goal_add_on: 0,
            strength: 1,
            dynamite_count: 0,
            has_strength_drink: false,
            has_lucky_clover: false,
            has_gem_polish: false,
            is_using_dynamite: false,
            using_dynamite_timer: 0.39,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    miner: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            miner: assets.load("images/miner_sheet.png"),
        }
    }
}
