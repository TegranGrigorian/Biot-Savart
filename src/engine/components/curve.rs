use nalgebra::Vector3;

pub struct Curve {
    pub name: String,
    pub points: Vec<Vector3<f32>>
}

impl Curve {
    pub fn new(name: String, points: Vec<Vector3<f32>>) -> Curve {
        Curve {name: name, points: points}
    }

}