//! 达成目标过渡界面

use crate::audio::{AudioAssets, TransitionMusicStatus, play_transition_music};
use crate::config::ImageAssets;
use crate::constants::COLOR_YELLOW;
use crate::screens::{Screen, stats::LevelStats};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MadeGoal), spawn_made_goal_ui);
    app.add_systems(Update, check_transition.run_if(in_state(Screen::MadeGoal)));
}

#[derive(Component)]
struct MadeGoalTimer {
    timer: Timer,
    music_finished: bool,
}

fn spawn_made_goal_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    mut stats: ResMut<LevelStats>,
    audio_assets: Res<AudioAssets>,
    mut transition_music: ResMut<TransitionMusicStatus>,
) {
    // 增加等级并计算实际关卡配置
    stats.level += 1;
    stats.calculate_real_level();
    stats.reset_timer();

    play_transition_music(
        &mut commands,
        &mut transition_music,
        "Made Goal Music",
        audio_assets.get_audio("MadeGoal").unwrap(),
        DespawnOnExit(Screen::MadeGoal),
    );

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
        Anchor::TOP_CENTER,
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 面板
    commands.spawn((
        Name::new("Goal Panel"),
        Sprite::from_image(image_assets.get_image("Panel").unwrap()),
        Transform::from_translation(love_to_bevy_coords(27.0, 80.0).extend(0.0)),
        Anchor::TOP_LEFT,
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
        Transform::from_translation(love_to_bevy_coords(70.0, 100.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::MadeGoal),
    ));

    // 延迟计时器（音乐播放完成后开始计时）
    commands.spawn((
        MadeGoalTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            music_finished: false,
        },
        DespawnOnExit(Screen::MadeGoal),
    ));
}

fn check_transition(
    time: Res<Time>,
    mut query: Query<&mut MadeGoalTimer>,
    transition_music: Res<TransitionMusicStatus>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for mut timer in &mut query {
        if !timer.music_finished {
            timer.music_finished = transition_music.is_finished();
            if !timer.music_finished {
                continue;
            }
        }

        if timer.timer.tick(time.delta()).just_finished() {
            next_screen.set(Screen::Shop);
            return;
        }
    }
}
