pub mod operations {
    pub fn add(x: f64, y: f64) -> f64 {
        x + y
    }

    pub fn subtract(x: f64, y: f64) -> f64 {
        x - y
    }

    pub fn multiply(x: f64, y: f64) -> f64 {
        x * y
    }

    pub fn divide(x: f64, y: f64) -> f64 {
        if y != 0.0 {
            x / y
        } else {
            f64::NAN
        }
    }
}

use std::io;

pub fn input_parser() -> f64 {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    match x.trim().parse() {
        Ok(num) => num,
        Err(_) => f64::NAN,
    }
}