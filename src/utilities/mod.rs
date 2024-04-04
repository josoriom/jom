pub mod color;
pub mod get_operating_system;
pub mod is_command_available;
pub mod execute_bash;

pub use color::{ Color, print_color };
pub use is_command_available::is_command_available;
pub use execute_bash::execute_bash;