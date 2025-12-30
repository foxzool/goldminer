//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

pub mod entity;
pub mod hook;
pub mod level;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, hook::plugin, entity::plugin));
}


#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    ShowNextGoal,
    Game,
    /// 达成目标/时间结束且达标
    ShoeMadeGoal,
    GameOver,
    Shop
}
