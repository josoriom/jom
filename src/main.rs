mod utilities;
mod interfaces;
mod os;

use utilities::{
    Color,
    print_color,
    is_command_available,
    execute_bash
};

use os::{
    DebianDependencies,
    DependenciesActions,
    OperatingSystem,
};

use interfaces::{ dependencies, dependencies_action, home, templates };

fn main() {
    home();
}
