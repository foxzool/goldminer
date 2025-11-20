//! The main menu (seen on the title screen).

use crate::constants::COLOR_YELLOW;
use crate::utils::love_to_bevy_coords;
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        Sprite::from_image(asset_server.load("images/bg_start_menu.png")),
        children![
            logo_title(&asset_server),
            play_button(&asset_server),
            score_button(&asset_server),
            developer_text(&asset_server),
            menu_arrow(&asset_server)
        ],
    ));
}

fn logo_title(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Logo Title"),
        Sprite::from_image(asset_server.load("images/text_goldminer.png")),
        Transform::from_translation(love_to_bevy_coords(160.0, 10.0).extend(1.0)),
        Anchor::TOP_CENTER,
    )
}

fn play_button(asset_server: &AssetServer) -> impl Bundle {
    let font = asset_server.load("fonts/Kurland.ttf");
    let style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };

    (
        Name::new("Start Game"),
        Text2d::new("Start Game"),
        style,
        Transform::from_translation(love_to_bevy_coords(30.0, 150.0).extend(0.0)),
        Anchor::TOP_LEFT,
        TextColor(COLOR_YELLOW),
    )
}

fn score_button(asset_server: &AssetServer) -> impl Bundle {
    let font = asset_server.load("fonts/Kurland.ttf");
    let style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };

    (
        Name::new("High Score"),
        Text2d::new("High Score"),
        style,
        Transform::from_translation(love_to_bevy_coords(30.0, 170.0).extend(0.0)),
        Anchor::TOP_LEFT,
        TextColor(COLOR_YELLOW),
    )
}

fn developer_text(asset_server: &AssetServer) -> impl Bundle {
    let font = asset_server.load("fonts/Pixel-Square-10-1.ttf");
    let style = TextFont {
        font: font.clone(),
        font_size: 10.0,
        ..default()
    };

    (
        Name::new("Developer log"),
        Text2d::new("Made with Bevy. Developed by Fox ZoOL."),
        style,
        Transform::from_translation(love_to_bevy_coords(75.0, 225.0).extend(0.0)),
        Anchor::TOP_LEFT,
        TextColor(COLOR_YELLOW),
    )
}

fn menu_arrow(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Menu Arrow"),
        Sprite::from_image(asset_server.load("images/menu_arrow.png")),
        Transform::from_translation(love_to_bevy_coords(5.0, 152.0).extend(0.0)),
        Anchor::TOP_LEFT,
    )
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_high_score_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::HighScore);
}
