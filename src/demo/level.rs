//! Spawn the main level.

use crate::config::{EntitiesConfig, LevelEntity, LevelsConfig};
use crate::config::{EntityDescriptor, EntityType, ImageAssets};
use crate::constants::{COLOR_DEEP_ORANGE, COLOR_GREEN, COLOR_ORANGE};
use crate::utils::love_to_bevy_coords;
use crate::{
    demo::player::{player, PlayerAssets},
    screens::Screen,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_level_assets);
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (setup_ui, spawn_background, spawn_level),
    );

    app.add_systems(
        Update,
        (spawn_entity_sprite,).run_if(in_state(Screen::Gameplay)),
    );
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_font = asset_server.load("fonts/visitor1.ttf");
    let game_style = TextFont {
        font: game_font.clone(),
        font_size: 20.0,
        ..default()
    };
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(10),
            ..default()
        },
        children![
            (
                TextSpan::new("Money"),
                game_style.clone(),
                TextColor(COLOR_DEEP_ORANGE)
            ),
            (
                TextSpan::new(" $0"),
                game_style.clone(),
                TextColor(COLOR_GREEN)
            ),
        ],
    ));

    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(28),
            left: px(10),
            ..default()
        },
        children![
            (
                TextSpan::new(" Goal"),
                game_style.clone(),
                TextColor(COLOR_DEEP_ORANGE)
            ),
            (
                TextSpan::new(" $1000"),
                game_style.clone(),
                TextColor(COLOR_GREEN)
            ),
        ],
    ));

    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(30),
            left: px(520),
            ..default()
        },
        children![
            (
                TextSpan::new("Time: "),
                game_style.clone(),
                TextColor(COLOR_DEEP_ORANGE)
            ),
            (
                TextSpan::new("0"),
                game_style.clone(),
                TextColor(COLOR_ORANGE)
            ),
        ],
    ));

    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(50),
            left: px(500),
            ..default()
        },
        children![
            (
                TextSpan::new("Level: "),
                game_style.clone(),
                TextColor(COLOR_DEEP_ORANGE)
            ),
            (
                TextSpan::new("A"),
                game_style.clone(),
                TextColor(COLOR_ORANGE)
            ),
        ],
    ));
}

pub fn spawn_background(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("LevelBackground"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            bg_top(&image_assets),
            bg_level(&image_assets),
            player(&player_assets, &mut texture_atlas_layouts),
        ],
    ));
}

fn bg_top(image_assets: &Res<ImageAssets>) -> impl Bundle {
    (
        Name::new("Top Background"),
        Transform::from_translation(love_to_bevy_coords(0.0, 0.0).extend(-1.0)),
        Anchor::TOP_LEFT,
        Sprite::from_image(image_assets.get_image("LevelCommonTop").unwrap()),
    )
}

fn bg_level(image_assets: &Res<ImageAssets>) -> impl Bundle {
    (
        Name::new("Level A Background"),
        Transform::from_translation(love_to_bevy_coords(0.0, 40.0).extend(-1.0)),
        Anchor::TOP_LEFT,
        Sprite::from_image(image_assets.get_image("LevelA").unwrap()),
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
                    for level_entity in config.entities.clone() {
                        parent.spawn((
                            Name::new(level_entity.entity_id.clone()),
                            Transform::from_translation(
                                love_to_bevy_coords(level_entity.pos.x, level_entity.pos.y)
                                    .extend(1.0),
                            ),
                            entities_config
                                .entities
                                .get(&level_entity.entity_id)
                                .unwrap()
                                .clone(),
                            level_entity,
                        ));
                    }
                });
        }
    }
}

pub fn spawn_entity_sprite(
    mut commands: Commands,
    _entity_handle: Res<EntityHandle>,
    q_entities: Query<(Entity, &LevelEntity, &EntityDescriptor), Added<LevelEntity>>,
    entities_assets: Res<ImageAssets>,
) {
    for (entity, level_entity, entity_desc) in q_entities.iter() {
        if let Some(img_handle) = entities_assets.get_image(&level_entity.entity_id) {
            let anchor = if entity_desc.entity_type == EntityType::Basic {
                Anchor::TOP_LEFT
            } else {
                Anchor::CENTER
            };
            commands
                .entity(entity)
                .insert((anchor, Sprite::from_image(img_handle.clone())));
        }
    }
}
