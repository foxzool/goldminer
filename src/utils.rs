use bevy::prelude::Vec2;

pub const VIRTUAL_WIDTH: u32 = 320;
pub const VIRTUAL_HEIGHT: u32 = 240;

/// convert love coord to bevy coord
pub fn love_to_bevy(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, VIRTUAL_HEIGHT as f32 - y)
}
