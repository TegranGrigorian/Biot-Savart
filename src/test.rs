use crate::engine::components::point::Point;
use crate::engine::components::wire::Wire;
use nalgebra::Vector3;
use crate::engine::math::Math;
pub fn test_biot_savart() -> Result<(), ()> {
    let wire: Wire = Wire::new(String::from("Wire 1"), vec![
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.01, 0.0, 0.0)]);
    let point: Point = Point::new(String::from("P1"), Vector3::new(0.0, 1.0, 0.0));
    let output = Math::calculate_biot_savart(wire, point, 2.0).unwrap();
    if !(&output <= &(2.0e-9 + 0.5e-9) && (&output >= &(2.0e-9 - 0.5e-9))) {
        println!("NOT RIGHT VALUE");
        return Err(());
    }
    println!("GOOD, VALUE: {:.3e}", output);
    Ok(())
}
