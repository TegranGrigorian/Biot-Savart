use Biot_Savart::engine;
use nalgebra::Vector3;
mod constants;
use engine::components::wire::Wire;
use engine::components::point::Point;
fn main() {
    let wire1: Wire = Wire {name: String::from("W1"),
        start: Vector3::new(0.0, 0.0, 0.0),
        end: Vector3::new(1.0, 1.0, 1.0)
    };
    let point: Point = Point { name: String::from("P1"), cords: Vector3::new(0.0, 23.3, 21.1) };

    println!("Wire {} starts at {:?} and ends at {:?}", wire1.name, wire1.start, wire1.end);
    println!("Test cross product: {:?}",  wire1.start.cross(&wire1.end));
    println!("Mu0 {}", constants::MU_0);
    println!("Point Name: {} at {:?}", point.name, point.cords);
}
