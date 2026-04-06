use std::vec;
use nalgebra::Vector3;

pub struct Curve {
    pub name: String,
    pub points: Vec<Vector3<f32>>
}

impl Curve {
    const MATH_EXPRESSIONS: std::vec::Vec<&str> = vec!["+", "-", "/", "*", "="];
    const VARIABLES: std::vec::Vec<&str> = vec!["x", "y", "z"];
    pub fn new(name: String, points: Vec<Vector3<f32>>) -> Curve {
        Curve {name: name, points: points}
    }

    // parse curve equation
    pub fn strip_space_equation(equation: &String){
        let equation_stripped: Vec<&str> = equation.split(" ").collect();
        println!("{:?}", String::from_iter(equation_stripped));
    }
}