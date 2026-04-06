use std::f32::consts::PI;

//consts
const MU_0: f32 = 4.0 * PI * 1.0e-7; // 4pi * 10^-7 Tm / A
pub static K:f32 = MU_0 / (4.0 * std::f32::consts::PI);
pub const DIVIDER: i32 = 10000000; // how many "dB"'s we will be calculating
pub const BASE_GROUND_SIZE: f32 = 20.0;
pub const MIN_B_RENDER_MAG: f32 = 1.0e-18;