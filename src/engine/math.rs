use core::f32;

use nalgebra::Vector3;

use crate::constants;
use crate::engine::components::point::Point;
use crate::engine::components::wire::Wire;
pub struct Math;

impl Math {
    pub fn calculate_biot_savart(wire: Wire, point: Point, current: f32) -> Result<f32, String> {
        // get segment lengths
        let output: f32 = 0.0;
        let mut total_length: f32 = 0.0;
        let mut dB = Vector3::new(0.0, 0.00, 0.0);
        for i in 0..wire.points.len() -1 {
            total_length += (wire.points[i + 1] - wire.points[i]).norm();
        }

        let db_length = total_length / (constants::DIVIDER as f32);
        for i in 0..wire.points.len() -1 {
            let seg_vec = (wire.points[i + 1] - wire.points[i]);
            let seg_len = seg_vec.norm();
            let seg_dir = seg_vec / seg_len;
            let n = (seg_len / db_length).ceil() as i16;
            let step = (seg_len / (n as f32));
            if seg_len == 0.0{continue;}
            let dl = seg_dir * step;
            for j in 0..n {
                let t_mid = ((j as f32) + 0.5) / (n as f32);
                let source: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = &point.cords + seg_vec * t_mid;
                let R = point.cords - source;
                let R_norm = R.norm();
                dB += constants::MU_0 * current * (((dl.cross(&R))) / R_norm.powf(3.0)); 
            }
        }
        Ok(output)
    }
}