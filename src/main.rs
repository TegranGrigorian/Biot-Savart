use nalgebra::Vector3;
mod constants;
fn main() {
    println!("Hello, world!");
    let wire1 = Wire {name: String::from("W1"),
        start: Vector3::new(0.0, 0.0, 0.0),
        end: Vector3::new(1.0, 1.0, 1.0)
    };

    println!("Wire {} starts at {:?} and ends at {:?}", wire1.name, wire1.start, wire1.end);
    println!("Test cross product: {:?}",  wire1.start.cross(&wire1.end));
    println!("Mu0 {}", constants::MU_0);   
}

struct Wire {
    name: String,
    start: Vector3<f32>,
    end: Vector3<f32>
}

