use std::vec;
use nalgebra::Vector3;

pub struct Curve {
    pub name: String,
    pub points: Vec<Vector3<f32>>
}

impl Curve {
    const MATH_EXPRESSIONS: &[&str] = &["+", "-", "/", "*", "="];
    const VARIABLES: &[&str] = &["x", "y", "z"];
    pub fn new(name: String, points: Vec<Vector3<f32>>) -> Curve {
        Curve {name: name, points: points}
    }

    // parse curve equation
    pub fn strip_space_equation(equation: &String) -> String {
        let equation_stripped: Vec<&str> = equation.split(" ").collect();
        println!("{}", String::from_iter(equation_stripped.clone()));
        String::from_iter(equation_stripped)
    }
    
    pub fn strip_parts_equation(equation: &String) {
        let mut parts: Vec<String> = vec![];
        let mut current_part = String::new();
        
        for ch in equation.chars() {
            if Self::MATH_EXPRESSIONS.contains(&ch.to_string().as_str()) {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                }
                parts.push(ch.to_string());
                current_part.clear();
            } else if ch != ' ' {
                current_part.push(ch);
            }
        }
        
        if !current_part.is_empty() {
            parts.push(current_part);
        }
        
        println!("{:?}", parts);
    }
}