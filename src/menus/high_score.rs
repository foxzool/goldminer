//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use crate::constants::{COLOR_GREEN, COLOR_YELLOW};
use crate::utils::love_to_bevy_coords;
use crate::{menus::Menu, theme::prelude::*};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HighScoreLog>();
    app.add_systems(OnEnter(Menu::HighScore), spawn_high_score_menu);
    app.add_systems(Update, go_back.run_if(in_state(Menu::HighScore)));
}

#[derive(Default, Resource)]
pub struct HighScoreLog {
    pub score: u32,
    pub level: u32,
}

fn spawn_high_score_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_score_log: Res<HighScoreLog>,
) {
    commands.spawn((
        widget::ui_root("High Score Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::HighScore),
        Sprite::from_image(asset_server.load("images/bg_goal.png")),
        children![
            title_area(&asset_server),
            panel_area(&asset_server, &high_score_log),
        ],
    ));
}

fn title_area(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Logo"),
        Sprite::from_image(asset_server.load("images/text_goldminer.png")),
        Transform::from_translation(love_to_bevy_coords(160.0 - 106.0, 20.0).extend(1.0)),
        Anchor::TOP_LEFT,
    )
}

fn panel_area(asset_server: &AssetServer, high_score_log: &Res<HighScoreLog>) -> impl Bundle {
    let font = asset_server.load("fonts/Kurland.ttf");
    let style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };
    (
        Name::new("panel"),
        Sprite::from_image(asset_server.load("images/panel.png")),
        Transform::from_translation(love_to_bevy_coords(160.0 - 133.0, 80.0).extend(1.0)),
        Anchor::TOP_LEFT,
        children![
            (
                Text2d::new("High Score:\n\n"),
                style.clone(),
                Transform::from_xyz(47.0, -10.0, 0.0),
                // Transform::from_translation(love_to_bevy_coords(70.0, 100.0).extend(1.0)),
                Anchor::TOP_LEFT,
                TextColor(COLOR_YELLOW),
            ),
            (
                Text2d::new(format!("${}", high_score_log.score)),
                style.clone(),
                Transform::from_xyz(47.0, -60.0, 0.0),
                Anchor::TOP_LEFT,
                TextColor(COLOR_GREEN),
            ),
            (
                Text2d::new(format!("at Level{}", high_score_log.level)),
                style,
                Transform::from_xyz(47.0, -80.0, 0.0),
                Anchor::TOP_LEFT,
                TextColor(COLOR_YELLOW),
            )
        ],
    )
}

fn go_back(input: Res<ButtonInput<KeyCode>>, mut next_menu: ResMut<NextState<Menu>>) {
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::NumpadEnter) {
        next_menu.set(Menu::Main);
    }
}
