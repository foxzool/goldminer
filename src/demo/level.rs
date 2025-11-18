//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{player, PlayerAssets},
    screens::Screen,
};

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            top_bg_sprite(&asset_server),
            level_bg_sprite(&asset_server),
            player(&player_assets, &mut texture_atlas_layouts),
        ],
    ));
}

fn top_bg_sprite(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Top Background"),
        Transform::from_xyz(0.0, 100.0, -1.0),
        Sprite::from_image(asset_server.load("images/bg_top.png")),
    )
}

fn level_bg_sprite(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Level A Background"),
        Transform::from_xyz(0.0, -20.0, -1.0),
        Sprite::from_image(asset_server.load("images/bg_level_A.png")),
    )
}
