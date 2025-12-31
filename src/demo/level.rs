//! Spawn the main level.

use crate::config::{EntitiesConfig, LevelEntity, LevelsConfig};
use crate::config::{EntityDescriptor, EntityType, ImageAssets};
use crate::constants::{COLOR_DEEP_ORANGE, COLOR_GREEN, COLOR_ORANGE};
use crate::screens::Screen;
use crate::utils::love_to_bevy_coords;
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
        (spawn_entity_sprite, update_ui).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
struct MoneyText;
#[derive(Component)]
struct GoalText;
#[derive(Component)]
struct TimerText;
#[derive(Component)]
struct LevelDisplay;

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<crate::screens::stats::LevelStats>,
) {
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
                TextSpan::new(format!(" ${}", stats.money)),
                game_style.clone(),
                TextColor(COLOR_GREEN),
                MoneyText,
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
                TextSpan::new(format!(" ${}", stats.goal)),
                game_style.clone(),
                TextColor(COLOR_GREEN),
                GoalText,
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
                TextSpan::new(format!("{:.0}", stats.timer)),
                game_style.clone(),
                TextColor(COLOR_ORANGE),
                TimerText,
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
                TextSpan::new(format!("{}", stats.level)),
                game_style.clone(),
                TextColor(COLOR_ORANGE),
                LevelDisplay,
            ),
        ],
    ));
}

fn update_ui(
    stats: Res<crate::screens::stats::LevelStats>,
    mut q_money: Query<&mut TextSpan, (With<MoneyText>, Without<GoalText>, Without<TimerText>)>,
    mut q_goal: Query<&mut TextSpan, (With<GoalText>, Without<MoneyText>, Without<TimerText>)>,
    mut q_timer: Query<&mut TextSpan, (With<TimerText>, Without<MoneyText>, Without<GoalText>)>,
) {
    for mut span in &mut q_money {
        let new_text = format!(" ${}", stats.money);
        if span.0 != new_text {
            span.0 = new_text;
        }
    }
    for mut span in &mut q_goal {
        let new_text = format!(" ${}", stats.goal);
        if span.0 != new_text {
            span.0 = new_text;
        }
    }
    for mut span in &mut q_timer {
        let new_text = format!("{:.0}", stats.timer);
        if span.0 != new_text {
            span.0 = new_text;
        }
    }
}

pub fn spawn_background(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Name::new("LevelBackground"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![bg_top(&image_assets), bg_level(&image_assets),],
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, level_entity, entity_desc) in q_entities.iter() {
        if let Some(img_handle) = entities_assets.get_image(&level_entity.entity_id) {
            let anchor = if entity_desc.entity_type == EntityType::MoveAround {
                Anchor::CENTER
            } else {
                // 抓取物体：锚点设为 (W/2, H/3)
                // Bevy 坐标系中，中心为 (0,0)，顶部为 0.5，底部为 -0.5
                // H/3 from top = 0.5 - 1.0/3.0 = 1/6
                Anchor::from(Vec2::new(0.0, 1.0 / 6.0))
            };

            if entity_desc.entity_type == EntityType::MoveAround {
                // Mole (MoveAround) logic
                // 纹理图集尺寸：mole_sheet.png 是 126x13，包含 7 个 sprite
                // 每个 sprite 尺寸：18x13 (126/7=18)
                // Lua config: idle frames={1}, move frames={1,2,3,4,5,6,7}, interval=0.15
                let layout = TextureAtlasLayout::from_grid(UVec2::new(18, 13), 7, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.entity(entity).insert((
                    anchor,
                    Sprite::from_atlas_image(
                        img_handle.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: 0,
                        },
                    ),
                    crate::demo::entity::EntityAnimation::new(
                        0.15,
                        vec![0],
                        vec![0, 1, 2, 3, 4, 5, 6],
                    ),
                ));
            } else {
                // Basic logic
                commands
                    .entity(entity)
                    .insert((anchor, Sprite::from_image(img_handle.clone())));
            }
        }
    }
}
