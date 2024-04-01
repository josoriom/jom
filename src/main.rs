mod utilities {
    pub mod color;
    pub mod dependencies;
    pub mod dependencies_action;
    pub mod distributions_data;
    pub mod main_menu;
    pub mod templates;
}

use utilities::{
    color::{print_color, Color},
    dependencies::dependencies,
    dependencies_action::dependencies_action,
    distributions_data,
    main_menu::main_menu,
    templates::templates,
};

fn main() {
    main_menu();
}
