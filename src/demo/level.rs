//! Spawn the main level.

use crate::demo::hook::{hook, HookAssets};
use crate::utils::love_to_bevy;
use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{player, PlayerAssets},
    screens::Screen,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_assets: Res<PlayerAssets>,
    hook_assets: Res<HookAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            bg_top(&asset_server),
            bg_level(&asset_server),
            player(&player_assets, &mut texture_atlas_layouts),
            hook(&hook_assets, &mut texture_atlas_layouts),
        ],
    ));
}

fn bg_top(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Top Background"),
        Transform::from(Transform::from_translation(
            love_to_bevy(0.0, 0.0).extend(-1.0),
        )),
        Anchor::TOP_LEFT,
        Sprite::from_image(asset_server.load("images/bg_top.png")),
    )
}

fn bg_level(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Level A Background"),
        Transform::from_translation(love_to_bevy(0.0, 40.0).extend(-1.0)),
        Anchor::TOP_LEFT,
        Sprite::from_image(asset_server.load("images/bg_level_A.png")),
    )
}
