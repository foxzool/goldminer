//! The game's menus and transitions between them.

mod high_score;
mod main;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((main::plugin, high_score::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    HighScore,
}
