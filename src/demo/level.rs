//! Spawn the main level.

use crate::config::SpritesAssets;
use crate::config::{EntitiesConfig, LevelDescriptor, LevelEntity, LevelsConfig, Position};
use crate::demo::hook::{hook, HookAssets};
use crate::utils::love_to_bevy_coords;
use crate::{
    asset_tracking::LoadResource,
    demo::player::{player, PlayerAssets},
    screens::Screen,
};
use bevy::ecs::entity::Entities;
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

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_assets: Res<PlayerAssets>,
    hook_assets: Res<HookAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("LevelBackground"),
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
        Transform::from_translation(love_to_bevy_coords(0.0, 0.0).extend(-1.0)),
        Anchor::TOP_LEFT,
        Sprite::from_image(asset_server.load("images/bg_top.png")),
    )
}

fn bg_level(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Level A Background"),
        Transform::from_translation(love_to_bevy_coords(0.0, 40.0).extend(-1.0)),
        Anchor::TOP_LEFT,
        Sprite::from_image(asset_server.load("images/bg_level_A.png")),
    )
}

#[derive(Resource)]
pub struct LevelHandle(Handle<LevelsConfig>);

#[derive(Resource)]
pub struct EntityHandle(Handle<EntitiesConfig>);

pub fn setup_level_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("config/levels.yaml"));
    commands.insert_resource(level);

    let entities = EntityHandle(asset_server.load("config/entities.yaml"));
    commands.insert_resource(entities);
}

pub fn spawn_level(
    mut commands: Commands,
    level_handle: Res<LevelHandle>,
    entity_handle: Res<EntityHandle>,
    levels: Res<Assets<LevelsConfig>>,
    entities: Res<Assets<EntitiesConfig>>,
) {
    if let (Some(level), Some(entities_config)) = (
        levels.get(level_handle.0.id()),
        entities.get(entity_handle.0.id()),
    ) {
        if let Some(config) = level.levels.get("LDEBUG") {
            // level_config.
            commands
                .spawn((
                    Name::new("LevelEntities"),
                    Transform::default(),
                    Visibility::default(),
                    DespawnOnExit(Screen::Gameplay),
                ))
                .with_children(|parent| {
                    for entity_config in config.entities.clone() {
                        parent.spawn((
                            Name::new(entity_config.entity_id.clone()),
                            Transform::from_translation(
                                love_to_bevy_coords(entity_config.pos.x, entity_config.pos.y)
                                    .extend(1.0),
                            ),
                            entities_config
                                .entities
                                .get(&entity_config.entity_id)
                                .unwrap()
                                .clone(),
                            entity_config,
                        ));
                    }
                });
        }
    }
}

pub fn spawn_entity_sprite(
    mut commands: Commands,
    entity_handle: Res<EntityHandle>,
    q_entities: Query<(Entity, &LevelEntity), Added<LevelEntity>>,
    entities_assets: Res<SpritesAssets>,
) {
    for (entity, level_entity) in q_entities.iter() {
        if let Some(img_handle) = entities_assets.get_sprite(&level_entity.entity_id) {
            commands
                .entity(entity)
                .insert((Anchor::TOP_LEFT, Sprite::from_image(img_handle.clone())));
        }
    }
}
