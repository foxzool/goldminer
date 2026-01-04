//! The main menu (seen on the title screen).

use crate::constants::COLOR_YELLOW;
use crate::menus::MenuSelect;
use crate::utils::love_to_bevy_coords;
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu)
        .add_systems(
            Update,
            (keyboard_input, update_menu_arrow).run_if(in_state(Menu::Main)),
        );
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_item: ResMut<NextState<MenuSelect>>,
) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        Sprite::from_image(asset_server.load("images/bg_start_menu.png")),
        children![
            play_button(&asset_server),
            score_button(&asset_server),
            developer_text(&asset_server),
            menu_arrow(&asset_server)
        ],
    ));

    next_item.set(MenuSelect::StartGame)
}

fn play_button(asset_server: &AssetServer) -> impl Bundle {
    let font = asset_server.load("fonts/Kurland.ttf");
    let style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };

    (
        Text2d::new("Start Game"),
        style,
        Transform::from_translation(love_to_bevy_coords(30.0, 150.0).extend(1.0)),
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
        Text2d::new("High Score"),
        style,
        Transform::from_translation(love_to_bevy_coords(30.0, 170.0).extend(1.0)),
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
        Text2d::new("Made with Bevy. Developed by Fox ZoOL."),
        style,
        Transform::from_translation(love_to_bevy_coords(75.0, 225.0).extend(1.0)),
        Anchor::TOP_LEFT,
        TextColor(COLOR_YELLOW),
    )
}

#[derive(Component)]
struct MenuArrow;

fn menu_arrow(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Menu Arrow"),
        Sprite::from_image(asset_server.load("images/menu_arrow.png")),
        Transform::from_translation(love_to_bevy_coords(5.0, 152.0).extend(1.0)),
        Anchor::TOP_LEFT,
        MenuArrow,
    )
}

fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    current_item: Res<State<MenuSelect>>,
    mut next_item: ResMut<NextState<MenuSelect>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
    resource_handles: Res<ResourceHandles>,
) {
    let mut up = input.just_pressed(KeyCode::ArrowUp);
    let mut down = input.just_pressed(KeyCode::ArrowDown);
    let mut confirm = input.just_pressed(KeyCode::Enter)
        || input.just_pressed(KeyCode::NumpadEnter)
        || input.just_pressed(KeyCode::KeyJ)
        || input.just_pressed(KeyCode::KeyK);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            up = true;
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            down = true;
        }
        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            confirm = true;
        }
    }

    if up || down {
        if current_item.get() == &MenuSelect::StartGame {
            next_item.set(MenuSelect::HighScore)
        } else {
            next_item.set(MenuSelect::StartGame)
        }
    }

    if confirm {
        if current_item.get() == &MenuSelect::StartGame {
            if resource_handles.is_all_done() {
                next_screen.set(Screen::NextGoal);
            } else {
                next_screen.set(Screen::Loading);
            }
        } else {
            next_menu.set(Menu::HighScore)
        }
    }
}

fn update_menu_arrow(
    mut q_arrow: Single<&mut Transform, With<MenuArrow>>,
    mut transitions: MessageReader<StateTransitionEvent<MenuSelect>>,
) {
    let Some(transition) = transitions.read().last() else {
        return;
    };
    let StateTransitionEvent { entered, .. } = transition;
    let Some(entered) = entered else {
        return;
    };

    let transform = if entered == &MenuSelect::StartGame {
        love_to_bevy_coords(5.0, 152.0).extend(1.0)
    } else {
        love_to_bevy_coords(5.0, 172.0).extend(1.0)
    };

    q_arrow.translation = transform;
}
