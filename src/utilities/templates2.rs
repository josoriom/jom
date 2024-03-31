mod utilities {
    pub mod color;
    pub mod questions;
}
use utilities::{
    color::{print_color, Color},
    questions::{ask_boolean, ask_option},
};
struct ColoredText {
    text: String,
    color: Color,
}
fn main() {
    let templates = vec![
        ColoredText {
            text: "Available templates: \n".to_string(),
            color: Color::Green,
        },
        ColoredText {
            text: "1. Javascript common".to_string(),
            color: Color::Green,
        },
        ColoredText {
            text: "2. Javascript ES".to_string(),
            color: Color::Green,
        },
        ColoredText {
            text: "3. Typescript".to_string(),
            color: Color::Blue,
        },
        ColoredText {
            text: "4. React + Typescript".to_string(),
            color: Color::Blue,
        },
        ColoredText {
            text: "5. Python".to_string(),
            color: Color::Yellow,
        },
        ColoredText {
            text: "To exit, type Ctrl + C".to_string(),
            color: Color::Red,
        },
    ];

    for template in templates {
        print_color(&template.text, template.color);
    }

    // Ask the user to select an option
    let option = ask_option("Enter the number of the option you want to select: ", 1, 5);

    // Based on the selected option, ask additional questions
    match option {
        3 => {
            let build_script = ask_boolean("build script? (yes/no): ");
            if build_script {
                println!("...");
            } else {
                println!("Ok");
            }
        }
        // Add more cases for other options here...
        _ => println!("Option not yet implemented."),
    }
}
