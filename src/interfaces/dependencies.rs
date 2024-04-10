use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    OperatingSystem,
    home,
    print_color,
    Color,
    DebianDependencies,
    is_command_available,
    execute_bash
};

struct MenuItem {
    name: String,
    selected: bool,
    color: Color,
}

impl MenuItem {
    fn new(name: &str, color: Color) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            selected: false,
            color,
        }
    }
}

fn initialize_dependencies() -> Vec<MenuItem> {
    let dependencies_names = vec![
        "google-chrome", "code", "curl", "fish", "git", "htop", "npm", "R", "docker", "rstudio"
    ];
    let mut dependencies = Vec::new();

    for name in dependencies_names {
        dependencies.push(MenuItem::new(
            name,
            if is_command_available(&name) {
                Color::Green
            } else {
                Color::Red
            },
        ));
    }

    dependencies
}
// TODO: check the OS and distribution
pub fn dependencies(choosen_action: &str, _os: OperatingSystem) {
    let mut dependencies = initialize_dependencies();

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor_position = 0;
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    draw(&mut stdout, &choosen_action, &dependencies, cursor_position);
    action_controller(stdin, &mut stdout, &mut dependencies, choosen_action, &mut cursor_position);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn action_controller(
    stdin: io::Stdin,
    stdout: &mut io::Stdout,
    dependencies: &mut Vec<MenuItem>,
    choosen_action: &str,
    cursor_position: &mut usize,
) {
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Esc | Key::Ctrl('c') => break,
            Key::Up => {
                if *cursor_position > 0 {
                    *cursor_position -= 1;
                }
            }
            Key::Down => {
                if *cursor_position < dependencies.len() - 1 {
                    *cursor_position += 1;
                }
            }
            Key::Char(' ') => {
                if let Some(dependency) = dependencies.get_mut(*cursor_position) {
                    dependency.selected = !dependency.selected;
                }
            }
            Key::Char('\n') => {
                execute(OperatingSystem::Debian, dependencies, choosen_action);
                break;
            }
            Key::Backspace => {
                home();
                break;
            }
            _ => {}
        }
        draw(stdout, choosen_action, dependencies, *cursor_position);
    }
}


fn execute(
    _os: OperatingSystem,
    dependencies: &Vec<MenuItem>,
    choosen_action: &str,
) {
    if let Some(datum) = DebianDependencies::for_distribution(OperatingSystem::Debian) {
        for item in dependencies {
            if item.selected {
                if let Some(other_datum) = datum.get(&item.name) {
                    if let Some(result) = other_datum.get(choosen_action) {
                        for element in result {
                            let _ = execute_bash(&item.name.to_string(), &element);
                        }
                    }
                }
            }
        };
    };
}

fn draw(
    stdout: &mut io::Stdout,
    choosen_action: &str,
    dependencies: &Vec<MenuItem>,
    cursor_position: usize,
) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    write!(
        stdout,
        "Select the dependencies to {} by using the arrow keys and pressing Space, to finish press Enter. \r\n",
        choosen_action
    )
    .unwrap();
    for (index, dependency) in dependencies.iter().enumerate() {
        if index == cursor_position {
            print_color(stdout, "> ", &dependency.color).unwrap();
        } else {
            print_color(stdout, "  ", &dependency.color).unwrap();
        }
        let _ = print_color(
            stdout,
            &format!(
                "[{}] - {}\r\n",
                if dependency.selected { "X" } else { " " },
                dependency.name
            ),
            &dependency.color,
        );
    }
    write!(stdout, "To exit, type Q or ESC \r\n").unwrap();
    stdout.flush().unwrap();
}
