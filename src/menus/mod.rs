//! The game's menus and transitions between them.

mod high_score;
mod main;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>().init_state::<MenuSelect>();

    app.add_plugins((main::plugin, high_score::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    HighScore,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum MenuSelect {
    #[default]
    StartGame,
    HighScore,
}
