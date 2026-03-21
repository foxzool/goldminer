use bevy::color::Color;

pub const COLOR_YELLOW: Color = Color::srgb_u8(255, 214, 33);
pub const COLOR_ORANGE: Color = Color::srgb_u8(239, 108, 0);
pub const COLOR_DEEP_ORANGE: Color = Color::srgb_u8(194, 136, 4);
pub const COLOR_GREEN: Color = Color::srgb_u8(67, 160, 71);

// --- Atlas 尺寸常量 ---
/// 金币爆炸特效: 3x3 网格，每帧 16x16
pub const BIG_GOLD_FX_WIDTH: u32 = 16;
pub const BIG_GOLD_FX_HEIGHT: u32 = 16;
pub const BIG_GOLD_FX_FRAMES: u32 = 9;

/// 爆炸特效: 3x4 或 4x3 网格，每帧 16x16
pub const EXPLOSIVE_FX_WIDTH: u32 = 16;
pub const EXPLOSIVE_FX_HEIGHT: u32 = 16;
pub const EXPLOSIVE_FX_FRAMES: u32 = 12;

/// 店主: 2 帧水平排列，每帧 80x80
pub const SHOPKEEPER_WIDTH: u32 = 80;
pub const SHOPKEEPER_HEIGHT: u32 = 80;
pub const SHOPKEEPER_FRAMES: u32 = 2;
