pub mod data;
pub mod dependencies;
pub mod dependencies_action;
pub mod color;
pub mod main_menu;
pub mod templates;
pub mod get_operating_system;

pub use data::{AvailableDependencies, OperatingSystem};
pub use dependencies::dependencies;
pub use dependencies_action::dependencies_action;
pub use color::{ Color, print_color };
pub use main_menu::main_menu;
pub use templates::templates;
