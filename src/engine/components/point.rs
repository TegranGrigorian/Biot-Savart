use nalgebra::Vector3;

pub struct Point {
    pub name: String,
    pub cords: Vector3<f32>
}

impl Point {
    pub fn new(name: String, cords: Vector3<f32>) -> Point {
        Point {name: name, cords: cords}
    }
}