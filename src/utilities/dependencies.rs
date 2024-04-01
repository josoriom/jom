use std::{
    fs,
    io::{self, Write},
    process::Command,
};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    distributions_data::OperatingSystem,
    main_menu,
    utilities::{
        color::{print_color, Color},
        dependencies,
        distributions_data::AvailableDependencies,
    },
};

#[derive(Debug)]
struct MenuItem {
    name: String,
    selected: bool,
    color: Color,
}

impl MenuItem {
    fn new(name: &str, color: Color) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            selected: true,
            color,
        }
    }
}

pub fn dependencies(choosen_action: &str, os: OperatingSystem) {
    let dependencies_names = vec!["fish", "git", "htop", "npm", "python3", "pip3", "R"];
    let mut dependencies = Vec::new();

    // Display the dependency status with Red and Green
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

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor_position = 0;

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    display_dependencies(&mut stdout, &dependencies, cursor_position);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Esc | Key::Ctrl('c') => break,
            Key::Up => {
                if cursor_position > 0 {
                    cursor_position -= 1;
                }
            }
            Key::Down => {
                if cursor_position < dependencies.len() - 1 {
                    cursor_position += 1;
                }
            }
            Key::Char(' ') => {
                if let Some(dependency) = dependencies.get_mut(cursor_position) {
                    dependency.selected = !dependency.selected;
                }
            }
            Key::Char('\n') => {
                execute_action(
                    &mut stdout,
                    OperatingSystem::Debian,
                    &dependencies,
                    choosen_action,
                );
                break;
            }
            Key::Backspace => {
                main_menu();
                break;
            }
            _ => {}
        }
        display_dependencies(&mut stdout, &dependencies, cursor_position);
    }
}

fn execute_action(
    stdout: &mut io::Stdout,
    os: OperatingSystem,
    dependencies: &Vec<MenuItem>,
    choosen_action: &str,
) {
    println!("Selected Dependencies: \r\n");
    for item in dependencies {
        // println!("Menu Items: {:?}", *item);
        if item.selected {
            print!("{:?}", item);
        }
    }
    if let Some(dependencies_list) = AvailableDependencies::for_distribution(os) {}
}

fn display_dependencies(
    stdout: &mut io::Stdout,
    dependencies: &Vec<MenuItem>,
    cursor_position: usize,
) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    write!(
        stdout,
        "Select the dependencies by using the arrow keys and pressing Space, to finish press Enter. \r\n"
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
    write!(stdout, "To exit, type Q or ESC").unwrap();
    stdout.flush().unwrap();
}

fn is_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
