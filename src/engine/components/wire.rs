use nalgebra::{Vector3};
pub struct Wire {
    pub name: String,
    pub points: Vec<Vector3<f32>>
}
impl Wire {
    pub fn new(name: String, points: Vec<Vector3<f32>>) -> Wire{
        Wire {name: name, points: points}
    }
}