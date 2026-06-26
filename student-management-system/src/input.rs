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

pub fn read_u8(prompt: &str) -> u8 {
    loop {
        let input = read_string(prompt);

        match input.parse::<u8>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid age."),
        }
    }
}

pub fn read_f32(prompt: &str) -> f32 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid mark."),
        }
    }
}
