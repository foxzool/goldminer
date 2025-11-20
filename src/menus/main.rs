//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("HighScore", open_high_score_menu),
        ],
    ));
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

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
