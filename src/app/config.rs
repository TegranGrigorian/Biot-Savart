use bevy::prelude::Color;
use bevy_egui::{egui, EguiContexts};

// Camera configs
pub const ROTATE_SENSITIVITY: f32 = 0.004;
pub const ZOOM_SENSITIVITY:f32 = 0.05;
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
pub const POINT_LABEL_COLOR: egui::Color32 = egui::Color32::from_rgb(175, 245, 255);

// Current Arrow Color
pub const CURRENT_COLOR: Color = Color::srgb(0.2, 1.0, 0.2);
pub const CURRENT_LABEL_COLOR: egui::Color32 = egui::Color32::from_rgb(160, 255, 160);

// Wire Line Color
pub const WIRE_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);
pub const WIRE_LABEL_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 230, 170);

// B-field Vector Color
pub const B_FIELD_COLOR: Color = Color::srgb(0.2, 0.8, 1.0);
pub const B_FIELD_LABEL_COLOR: egui::Color32 = egui::Color32::from_rgb(130, 230, 255);
pub const B_FIELD_LABEL_ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 130, 130);

// Orbit configs
pub const ORBIT_MINIMUM_ZOOM: f32 = 0.5;
pub const ORBIT_MAXIMUM_ZOOM: f32 = 150.0;
pub const ORBIT_ROTATE_MOE: f32 = 1.54;

// Font configs
pub const FONT_SIZE: f32 = 16.0;

// Sandbox Configs
pub const ZERO_PLANE_COLOR: Color = Color::srgb(0.15, 0.15, 0.18);
pub const PLANE_MESH_SIZE: f32 = 20.0;

// Sidebar dragable textbox
pub const DRAGABLE_TEXTBOX_SPEED: f32 = 0.1;