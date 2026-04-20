use crate::engine::components::point::Point;
use crate::engine::components::wire::Wire;
use nalgebra::Vector3;
use crate::engine::math::Math;
// use crate::engine::components::curve::Curve;
/*
Test Engine with pre computed answer
Probe: (0, 1, 0)
Wire: (0,0,0) -> (0.01, 0, 0) @ 2 Amps
Ans = (0i + 0j + 2.0 * 10^-9 K)T
*/
pub fn test_biot_savart() {
    // wire
    let wire: Wire = Wire::new(String::from("Wire 1"), vec![
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.01, 0.0, 0.0)]);
    
    // point
    let point: Point = Point::new(String::from("P1"), 
    Vector3::new(0.0, 1.0, 0.0));

    // output + check if output is in range of what it should be
    let output = Math::calculate_biot_savart(wire, point, 2.0).unwrap();
    if !(&output <= &(2.0e-9 + 0.5e-9) && (&output >= &(2.0e-9 - 0.5e-9))) {
        println!("NOT RIGHT VALUE");
    }
    
    // print debug
    println!("GOOD, VALUE: {:.3e}", output);
}

pub fn test_equation_strip() {
    // equation in stirng form
    let equation = String::from("y = mx + b");
    // Curve::strip_parts_equation(&Curve::strip_space_equation(&equation));
}