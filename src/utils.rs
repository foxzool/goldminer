use bevy::prelude::Vec2;

pub const VIRTUAL_WIDTH: f32 = 320.0;
pub const VIRTUAL_HEIGHT: f32 = 240.0;

/// convert love coord to bevy coord
pub fn love_to_bevy_coords(x: f32, y: f32) -> Vec2 {
    Vec2::new(
        x - VIRTUAL_WIDTH / 2.0,  // X: 从左上角转到中心
        VIRTUAL_HEIGHT / 2.0 - y, // Y: 翻转并转到中心
    )
}
