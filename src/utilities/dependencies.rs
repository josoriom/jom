use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::main_menu;
struct MenuItem {
    name: String,
    selected: bool,
}

impl MenuItem {
    fn new(name: &str) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            selected: true,
        }
    }
}

pub fn dependencies() {
    let mut dependencies = vec![
        MenuItem::new("npm"),
        MenuItem::new("python3"),
        MenuItem::new("git"), // Move cursor to next line
        MenuItem::new("htop"),
        MenuItem::new("fish"),
    ];

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
                execute_action(&dependencies);
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

fn execute_action(dependencies: &Vec<MenuItem>) {
    println!("Selected Dependencies: \r\n");
    for dependency in dependencies {
        if dependency.selected {
            println!("{} \r\n", dependency.name);
        }
    }
}

fn display_dependencies(
    stdout: &mut io::Stdout,
    dependencies: &Vec<MenuItem>,
    cursor_position: usize,
) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap(); // Move cursor to next line
    write!(
        stdout,
        "Select the dependencies by using the arrow keys and pressing Space, to finish press Enter. \r\n"
    )
    .unwrap();
    for (index, dependency) in dependencies.iter().enumerate() {
        if index == cursor_position {
            write!(stdout, "> ").unwrap();
        } else {
            write!(stdout, "  ").unwrap();
        }
        write!(
            stdout,
            "[{}] - {}\r\n",
            if dependency.selected { "X" } else { " " },
            dependency.name
        )
        .unwrap();
    }
    write!(stdout, "To exit, type Q or ESC").unwrap();
    stdout.flush().unwrap();
}
