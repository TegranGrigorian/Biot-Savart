use Biot_Savart::engine;
use nalgebra::Vector3;
mod constants;
use engine::components::wire::Wire;
use engine::components::point::Point;
fn main() {
    let wire1: Wire = Wire {name: String::from("W1"), points: vec![Vector3::new(1.0,0.0,0.0), Vector3::new(1.0,2.0,3.6)]};
    let point: Point = Point { name: String::from("P1"), cords: Vector3::new(-1.0, 1.0, 0.0) };
    println!("Wire {} points {:?}", wire1.name, wire1.points);
    println!("Test cross product: {:?}",  wire1.points[0].cross(&point.cords));
    println!("Mu0 {}", constants::MU_0);
    println!("Point Name: {} at {:?}", point.name, point.cords);
}
