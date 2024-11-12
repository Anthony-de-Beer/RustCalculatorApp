mod calculator;

use calculator::{input_parser, operations::{add, subtract, multiply, divide}};

fn main() {
    loop {
        let result: f64;
        let ops: [fn(f64, f64) -> f64; 4] = [add, subtract, multiply, divide];
    
        println!("Enter the first number: ");
        let x: f64 = input_parser();
    
        if f64::is_nan(x) {
            println!("Invalid input!");
            return;
        }
    
        println!("Enter the second number: ");
        let y: f64 = input_parser();
    
        if f64::is_nan(y) {
            println!("Invalid input!");
            return;
        }
    
        println!("List of operators:");
        println!("(1) Add");
        println!("(2) Subtract");
        println!("(3) Multiply");
        println!("(4) Divide");
        println!("Select the number associated with the desired operation: ");
    
        let op: f64 = input_parser();
    
        if f64::is_nan(op) {
            println!("Invalid input!");
            return;
        }
    
        let op: i32 = op as i32;
    
        if op > 4 || op < 1 {
            println!("Invalid Selection!");
            return;
        }
    
        result = ops[(op - 1) as usize](x, y);
    
        println!("The result is: {}", result);
    
        println!("Continue? (y/n)");
        io::stdin().read_line(&mut y_or_n).expect("Invalid Input");
    
        if y_or_n.trim() == "n" {
            break;
        }
    }
}