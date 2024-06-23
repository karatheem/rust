use std::io;

fn main() {
    println!("Rust Calculator");
    println!("---------------");

    loop {
        // Read first number
        let mut a = String::new();
        println!("Please input the first number (or type 'exit' to quit):");
        io::stdin()
            .read_line(&mut a)
            .expect("F");
        if a.trim().to_lowercase() == "exit" {
            break;
        }
        let a: f64 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        // Read operator
        let mut operator = String::new();
        println!("Please enter operator you wish to use:");
        io::stdin()
            .read_line(&mut operator)
            .expect("F");
        let operator = operator.trim();

        // Read the second number
        let mut b = String::new();
        println!("Please input the second number:");
        io::stdin()
            .read_line(&mut b)
            .expect("F");
        let b: f64 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
    }
}
