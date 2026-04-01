use nalgebra::Vector;

use crate::constants;
use crate::engine::components::point::Point;
use crate::engine::components::wire::Wire;
pub struct Math;

impl Math {
    pub fn calculate_biot_savart(wire: Wire, point: Point) -> Result<f32, String> {
        // get segment lengths
        let output: f32 = 0.0;
        let mut total_length: f32 = 0.0;
        for i in 0..wire.points.len() -1 {
            total_length += (wire.points[i + 1] - wire.points[i]).norm();
        }

        let db_length = total_length / (constants::DIVIDER as f32);
        
        Ok(output)
    }
}