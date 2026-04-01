use bevy::prelude::Color;
// Camera configs
pub const ROTATE_SENSITIVITY: f32 = 0.004;
pub const ZOOM_SENSITIVITY:f32 = 0.0025;
pub const PAN_SENSITIVITY: f32 = 0.0025;

// Arrow Configs
pub const CURRENT_ARROW_MIN_LENGTH: f32 = 0.22;
pub const CURRENT_ARROW_SCALE: f32 = 0.28;
pub const CURRENT_ARROW_MAX_LENGTH: f32 = 3.5;

pub const B_ARROW_MIN_LENGTH: f32 = 0.20;
pub const B_ARROW_SCALE: f32 = 0.55;
pub const B_ARROW_MAX_LENGTH: f32 = 4.5;
pub const B_ARROW_NORMALIZATION_FACTOR: f32 = 1.0e-9;
pub const B_ARROW_MIN_NORMALIZED: f32 = 1.0e-6;
pub const B_ARROW_EXPONENT: f32 = 0.25;

// Seg Configs
pub const SEG_LENGTH_SQUARED: f32 = 1.0e-10;

// Point Color
pub const POINT_COLOR: Color = Color::srgb(1.0, 0.45, 0.1);