//! The game's main screen states and transitions between them.

mod game_over;
mod gameplay;
mod loading;
mod made_goal;
mod new_high_score;
mod next_goal;
pub mod persistent;
mod shop;
mod splash;
pub mod stats;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.init_resource::<stats::LevelStats>();
    app.init_resource::<persistent::PersistentData>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        next_goal::plugin,
        made_goal::plugin,
        shop::plugin,
        game_over::plugin,
        new_high_score::plugin,
    ));

    app.add_systems(Update, handle_global_exit);
}

fn handle_global_exit(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::Mode) {
            exit.write(AppExit::Success);
        }
    }
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    NextGoal,
    Gameplay,
    MadeGoal,
    Shop,
    GameOver,
    NewHighScore,
}
