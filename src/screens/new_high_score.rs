//! 新纪录界面：当玩家刷新最高分时显示

use crate::config::ImageAssets;
use crate::constants::{COLOR_GREEN, COLOR_YELLOW};
use crate::screens::{Screen, persistent::PersistentData};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::NewHighScore), spawn_new_high_score_ui);
    app.add_systems(
        Update,
        check_keyboard_input.run_if(in_state(Screen::NewHighScore)),
    );
}

fn spawn_new_high_score_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    persistent: Res<PersistentData>,
) {
    // 背景
    commands.spawn((
        Name::new("Goal Background"),
        Sprite::from_image(image_assets.get_image("Goal").unwrap()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        DespawnOnExit(Screen::NewHighScore),
    ));

    // 标题 (水平居中, y=20)
    commands.spawn((
        Name::new("Goal Title"),
        Sprite::from_image(image_assets.get_image("Title").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 20.0).extend(0.0)),
        DespawnOnExit(Screen::NewHighScore),
    ));

    // 面板 (水平居中, y=80)
    commands.spawn((
        Name::new("Goal Panel"),
        Sprite::from_image(image_assets.get_image("Panel").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 80.0).extend(0.0)),
        DespawnOnExit(Screen::NewHighScore),
    ));

    // 文字：New High Score: (x=70, y=100)
    let font = asset_server.load("fonts/Kurland.ttf");
    commands.spawn((
        Name::new("NewHighScore Label"),
        Text2d::new("New High Score:"),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(70.0, 100.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NewHighScore),
    ));

    // 金额 (绿色)
    commands.spawn((
        Name::new("HighScore Amount"),
        Text2d::new(format!("${}", persistent.high_score)),
        TextFont {
            font: font.clone(),
            font_size: 25.0,
            ..default()
        },
        TextColor(COLOR_GREEN),
        Transform::from_translation(love_to_bevy_coords(70.0, 130.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NewHighScore),
    ));

    // at LevelX (黄色)
    commands.spawn((
        Name::new("HighScore Level"),
        Text2d::new(format!("at Level{}", persistent.high_level)),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(70.0, 160.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NewHighScore),
    ));
}

fn check_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // 任意键返回主菜单
    if input.get_just_pressed().next().is_some() {
        next_screen.set(Screen::Title);
    }
}
