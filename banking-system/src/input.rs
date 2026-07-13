use std::io;

pub fn read_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_string(prompt);

        match input.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

pub fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid amount."),
        }
    }
}