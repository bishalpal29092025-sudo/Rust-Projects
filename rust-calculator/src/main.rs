use std::io;

fn main() {
    println!("\n=========================");
    println!("Rust CLI Calculator");
    println!("=========================");

    loop {
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Division");
        println!("5. Exit");

        println!("Choose an Operation: ");

        let choice = read_number();

        match choice {
            1 => calculate(add),
            2 => calculate(subtract),
            3 => calculate(multiply),
            4 => calculate(divide),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .unwrap_or(-1)
}

fn read_float(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn calculate(operation: fn(f64, f64) -> f64){
    let num1 = read_float("Enter first number: ");
    let num2 = read_float("Enter the second number: ");

    let result = operation(num1, num2);

    println!("Result: {}", result);
}

fn add(a: f64, b: f64) -> f64{
    a + b
}

fn subtract(a: f64, b: f64) -> f64{
    a - b
}


fn multiply(a: f64, b: f64) -> f64{
    a * b
}

fn divide(a: f64, b: f64) -> f64{
    if b == 0.0{
        println!("Cannot divide by zero.");
        return 0.0
    }else {
        a / b
    }
}