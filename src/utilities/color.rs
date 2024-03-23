use std::io::{self, Write};
pub enum Color {
    Green,
    Blue,
    Yellow,
    Red,
}

pub fn print_color(text: &str, color: Color) {
    let color_code = match color {
        Color::Green => 32,
        Color::Blue => 34,
        Color::Yellow => 33,
        Color::Red => 31,
    };
    println!("\x1B[{}m{}\x1B[0m", color_code, text);
    io::stdout().flush().unwrap();
}
