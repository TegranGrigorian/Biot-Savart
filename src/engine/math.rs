use nalgebra::Vector3;

use crate::constants;
use crate::engine::components::point::Point;
use crate::engine::components::wire::Wire;

pub struct Math;

impl Math {
    pub fn calculate_biot_savart(wire: Wire, point: Point, current: f32) -> Result<f32, String> {
        if wire.points.len() < 2 {
            return Err("Wire needs at least 2 points".to_string());
        }

        let mut total_length: f32 = 0.0;
        for i in 0..wire.points.len() - 1 {
            total_length += (wire.points[i + 1] - wire.points[i]).norm();
        }

        if total_length <= 0.0 {
            return Err("Wire length must be greater than zero".to_string());
        }

        let dl_length = total_length / (constants::DIVIDER as f32);
        let k = constants::MU_0 / (4.0 * std::f32::consts::PI);
        let mut b_vec = Vector3::new(0.0, 0.0, 0.0);

        for i in 0..wire.points.len() - 1 {
            let p0 = wire.points[i];
            let p1 = wire.points[i + 1];

            let seg_vec = p1 - p0;
            let seg_len = seg_vec.norm();
            if seg_len == 0.0 {
                continue;
            }

            let seg_dir = seg_vec / seg_len;
            let n = ((seg_len / dl_length).ceil() as usize).max(1);
            let step = seg_len / n as f32;
            let dl = seg_dir * step;

            for j in 0..n {
                let t_mid = (j as f32 + 0.5) / n as f32;
                let source = p0 + seg_vec * t_mid;
                let r = point.cords - source;
                let r_norm = r.norm();

                if r_norm < 1.0e-8 {
                    continue;
                }

                b_vec += k * current * dl.cross(&r) / r_norm.powi(3);
            }
        }

        Ok(b_vec.norm())
    }

    pub fn test_calc_bs() {
        // let wire = Wire {name: String::from("Wire 1"), 
        // points: vec![
        //     Vector3::new(0.0, 0.0, 0.0), 
        //     Vector3::new(0.01, 0.0, 0.0)] 
        // };
          let wire: Wire = Wire::new(String::from("Wire 1"), vec![
            Vector3::new(0.0, 0.0, 0.0), 
            Vector3::new(0.01, 0.0, 0.0)]);
        let point: Point = Point {name: String::from("p1"), 
        cords: Vector3::new(0.0, 1.0, 0.0)
        };
        let output = Math::calculate_biot_savart(wire, point, 2.0);
        println!("Output: {:.2e}", &output.unwrap());
    }
}