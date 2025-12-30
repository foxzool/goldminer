//! 游戏结束界面

use crate::config::ImageAssets;
use crate::constants::COLOR_YELLOW;
use crate::screens::{Screen, persistent::PersistentData, stats::LevelStats};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_game_over_ui);
    app.add_systems(
        Update,
        check_keyboard_input.run_if(in_state(Screen::GameOver)),
    );
}

/// 标记本次游戏是否刷新了最高分
#[derive(Resource, Default)]
struct IsNewHighScore(bool);

fn spawn_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    stats: Res<LevelStats>,
    persistent: Res<PersistentData>,
) {
    // 检查是否刷新最高分（但不立即更新，等按键时再更新）
    let is_new_high_score = stats.money > persistent.high_score;
    commands.insert_resource(IsNewHighScore(is_new_high_score));

    // 背景
    commands.spawn((
        Name::new("Goal Background"),
        Sprite::from_image(image_assets.get_image("Goal").unwrap()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        DespawnOnExit(Screen::GameOver),
    ));

    // 标题 (水平居中, y=20)
    commands.spawn((
        Name::new("Goal Title"),
        Sprite::from_image(image_assets.get_image("Title").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 20.0).extend(0.0)),
        DespawnOnExit(Screen::GameOver),
    ));

    // 面板 (水平居中, y=80)
    commands.spawn((
        Name::new("Goal Panel"),
        Sprite::from_image(image_assets.get_image("Panel").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 80.0).extend(0.0)),
        DespawnOnExit(Screen::GameOver),
    ));

    // 文字描述 (x=50, y=130 根据文档)
    let font = asset_server.load("fonts/Kurland.ttf");
    commands.spawn((
        Name::new("GameOver Text"),
        Text2d::new("You didn't reach the\ngoal!"),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(50.0, 130.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::GameOver),
    ));

    // 提示按键
    commands.spawn((
        Name::new("Press Any Key"),
        Text2d::new("Press Any Key to Continue"),
        TextFont {
            font: font.clone(),
            font_size: 15.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(160.0, 200.0).extend(1.0)),
        bevy::sprite::Anchor::CENTER,
        DespawnOnExit(Screen::GameOver),
    ));
}

fn check_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    is_new_high_score: Res<IsNewHighScore>,
    stats: Res<LevelStats>,
    mut persistent: ResMut<PersistentData>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if input.get_just_pressed().next().is_some() {
        if is_new_high_score.0 {
            // 刷新最高分，保存并跳转到新纪录界面
            persistent.high_score = stats.money;
            persistent.high_level = stats.level;
            next_screen.set(Screen::NewHighScore);
        } else {
            // 未刷新，返回主菜单
            next_screen.set(Screen::Title);
        }
    }
}
