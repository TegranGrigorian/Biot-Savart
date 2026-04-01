use std::f32::consts::PI;
const MU_0: f32 = 4.0 * PI * 1.0e-7; // 4pi * 10^-7 Tm / A
pub static K:f32 = MU_0 / (4.0 * std::f32::consts::PI);
pub const DIVIDER: i16 = 10000; // how many "dB"'s we will be calculating