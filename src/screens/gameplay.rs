use crate::{
    menus::Menu,
    screens::{Screen, stats::LevelStats},
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
            update_gameplay_timer.run_if(in_state(Screen::Gameplay)),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), close_menu);
}

fn update_gameplay_timer(
    time: Res<Time>,
    mut stats: ResMut<LevelStats>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    stats.timer -= time.delta_secs();
    if stats.timer <= 0.0 {
        stats.timer = 0.0;
        if stats.reach_goal() {
            next_screen.set(Screen::MadeGoal);
        } else {
            next_screen.set(Screen::GameOver);
        }
    }
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
