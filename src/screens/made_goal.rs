//! 达成目标过渡界面

use crate::audio::{AudioAssets, music};
use crate::config::ImageAssets;
use crate::constants::COLOR_YELLOW;
use crate::screens::{Screen, stats::LevelStats};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MadeGoal), spawn_made_goal_ui);
    app.add_systems(Update, check_transition.run_if(in_state(Screen::MadeGoal)));
}

#[derive(Component)]
struct MadeGoalTimer(Timer);

fn spawn_made_goal_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    mut stats: ResMut<LevelStats>,
    audio_assets: Res<AudioAssets>,
) {
    // 增加等级并计算实际关卡配置
    stats.level += 1;
    stats.calculate_real_level();
    stats.reset_timer();

    // 播放音乐
    commands.spawn((
        Name::new("Made Goal Music"),
        music(audio_assets.get_audio("MadeGoal").unwrap()),
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 背景
    commands.spawn((
        Name::new("Goal Background"),
        Sprite::from_image(image_assets.get_image("Goal").unwrap()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 标题
    commands.spawn((
        Name::new("Goal Title"),
        Sprite::from_image(image_assets.get_image("Title").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 20.0).extend(0.0)),
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 面板
    commands.spawn((
        Name::new("Goal Panel"),
        Sprite::from_image(image_assets.get_image("Panel").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 80.0).extend(0.0)),
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 文字描述
    let font = asset_server.load("fonts/Kurland.ttf");
    commands.spawn((
        Name::new("Goal Text"),
        Text2d::new("You made it to\nthe next Level!"),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(90.0, 60.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 转换计时器 (2秒)
    commands.spawn((
        MadeGoalTimer(Timer::from_seconds(2.0, TimerMode::Once)),
        DespawnOnExit(Screen::MadeGoal),
    ));
}

fn check_transition(
    time: Res<Time>,
    mut query: Query<&mut MadeGoalTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            next_screen.set(Screen::Shop);
        }
    }
}
