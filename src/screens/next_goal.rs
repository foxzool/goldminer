//! 显示下一关目标界面

use crate::audio::{AudioAssets, music};
use crate::config::ImageAssets;
use crate::constants::{COLOR_GREEN, COLOR_YELLOW};
use crate::screens::{Screen, stats::LevelStats};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::NextGoal), spawn_next_goal_ui);
    app.add_systems(Update, check_transition.run_if(in_state(Screen::NextGoal)));
}

#[derive(Component)]
struct NextGoalTimer(Timer);

fn spawn_next_goal_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    mut stats: ResMut<LevelStats>,
    audio_assets: Res<AudioAssets>,
) {
    // 更新目标金额
    stats.update_goal();

    let goal_text = if stats.is_first_init {
        stats.is_first_init = false;
        "Your First Goal is"
    } else {
        "Your Next Goal is"
    };

    // 播放音乐
    commands.spawn((
        Name::new("Goal Music"),
        music(audio_assets.get_audio("Goal").unwrap()),
    ));

    // 背景
    commands.spawn((
        Name::new("Goal Background"),
        Sprite::from_image(image_assets.get_image("Goal").unwrap()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        DespawnOnExit(Screen::NextGoal),
    ));

    // 标题
    commands.spawn((
        Name::new("Goal Title"),
        Sprite::from_image(image_assets.get_image("Title").unwrap()),
        Transform::from_translation(love_to_bevy_coords(54.0, 20.0).extend(0.0)),
        Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NextGoal),
    ));

    // 面板
    commands.spawn((
        Name::new("Goal Panel"),
        Sprite::from_image(image_assets.get_image("Panel").unwrap()),
        Transform::from_translation(love_to_bevy_coords(27.0, 80.0).extend(0.0)),
        Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NextGoal),
    ));

    // 文字描述
    let font = asset_server.load("fonts/Kurland.ttf");
    commands.spawn((
        Name::new("Goal Text"),
        Text2d::new(goal_text),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(70.0, 100.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::NextGoal),
    ));

    // 金额文字
    commands.spawn((
        Name::new("Goal Amount"),
        Text2d::new(format!("${}", stats.goal)),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(COLOR_GREEN),
        Transform::from_translation(love_to_bevy_coords(160.0, 140.0).extend(1.0)),
        bevy::sprite::Anchor::CENTER,
        DespawnOnExit(Screen::NextGoal),
    ));

    // 转换计时器 (3秒)
    commands.spawn(NextGoalTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn check_transition(
    time: Res<Time>,
    mut query: Query<&mut NextGoalTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            next_screen.set(Screen::Gameplay);
        }
    }
}
