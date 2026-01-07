//! The high score menu.
//!
//! Displays the highest score achieved by the player.

use crate::constants::{COLOR_GREEN, COLOR_YELLOW};
use crate::screens::persistent::PersistentData;
use crate::utils::love_to_bevy_coords;
use crate::{menus::Menu, theme::prelude::*};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::HighScore), spawn_high_score_menu);
    app.add_systems(Update, go_back.run_if(in_state(Menu::HighScore)));
}

fn spawn_high_score_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    persistent: Res<PersistentData>,
) {
    commands.spawn((
        widget::ui_root("High Score Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::HighScore),
        Sprite::from_image(asset_server.load("images/bg_goal.png")),
        children![
            title_area(&asset_server),
            panel_area(&asset_server, &persistent),
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

fn panel_area(asset_server: &AssetServer, persistent: &Res<PersistentData>) -> impl Bundle {
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
                Anchor::TOP_LEFT,
                TextColor(COLOR_YELLOW),
            ),
            (
                Text2d::new(format!("${}", persistent.high_score)),
                style.clone(),
                Transform::from_xyz(47.0, -60.0, 0.0),
                Anchor::TOP_LEFT,
                TextColor(COLOR_GREEN),
            ),
            (
                Text2d::new(format!("at Level {}", persistent.high_level)),
                style,
                Transform::from_xyz(47.0, -80.0, 0.0),
                Anchor::TOP_LEFT,
                TextColor(COLOR_YELLOW),
            )
        ],
    )
}

fn go_back(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    let mut pressed = input.get_just_pressed().next().is_some();
    if !pressed {
        for gamepad in &gamepads {
            if gamepad.get_just_pressed().next().is_some() {
                pressed = true;
                break;
            }
        }
    }

    if pressed {
        next_menu.set(Menu::Main);
    }
}
