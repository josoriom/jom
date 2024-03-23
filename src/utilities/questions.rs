use std::io::{self, Write};

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

pub fn ask_boolean(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let input = read_input();
        let response = input.trim().to_lowercase();
        match response.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Invalid input. Please enter 'yes', 'no'."),
        }
    }
}

pub fn ask_option(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let input = read_input();
        match input.trim().parse::<usize>() {
            Ok(num) if num >= min && num <= max => return num,
            Ok(_) => println!(
                "Invalid input. Please enter a number between {} and {}.",
                min, max
            ),
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
