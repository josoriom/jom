use std::io::{self, Write};

pub enum Color {
    Green,
    Blue,
    Yellow,
    Red,
}

pub fn print_color(stdout: &mut dyn Write, text: &str, color: &Color) -> io::Result<()> {
    let color_code = match color {
        Color::Green => 32,
        Color::Blue => 34,
        Color::Yellow => 33,
        Color::Red => 31,
    };
    write!(stdout, "\x1B[{}m{}\x1B[0m", color_code, text)?;
    stdout.flush()?;
    Ok(())
}
