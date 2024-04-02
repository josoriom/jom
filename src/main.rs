mod utilities;
mod interface;

use utilities::{
    AvailableDependencies,
    Color,
    OperatingSystem,
    print_color,
    is_command_available,
    execute_bash
};

use interface::{ dependencies, dependencies_action, home, templates };

fn main() {
    home();
}
