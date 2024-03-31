mod utilities {
    pub mod color;
    pub mod dependencies;
    pub mod main_menu;
    pub mod templates;
}

use utilities::{
    color::{print_color, Color},
    dependencies::dependencies,
    main_menu::main_menu,
    templates::templates,
};

fn main() {
    main_menu();
}
