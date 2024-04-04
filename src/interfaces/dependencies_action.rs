use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    dependencies, home,
    utilities::color::{print_color, Color},
    OperatingSystem
};

struct MenuItem {
    name: String,
    selected: bool,
}

impl MenuItem {
    fn new(name: &str, selected: bool) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            selected,
        }
    }
}

pub fn dependencies_action() {
    let actions = vec!["install", "uninstall"];
    let mut options = Vec::new();
    for name in actions {
        options.push(MenuItem::new(name, false));
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor_position = 0;
    write!(stdout, "{}", termion::clear::All).unwrap();
    draw(&mut stdout, &options, cursor_position);
    action_controller(stdin, &mut stdout, &mut options, &mut cursor_position);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn action_controller(
    stdin: io::Stdin,
    stdout: &mut io::Stdout,
    options: &mut Vec<MenuItem>,
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
                if *cursor_position < options.len() - 1 {
                    *cursor_position += 1;
                }
            }
            Key::Char(' ') => {
                if let Some(dependency) = options.get_mut(*cursor_position) {
                    dependency.selected = !dependency.selected;
                }
            }
            Key::Char('\n') => {
                if let Some(dependency) = options.get_mut(*cursor_position) {
                    execute(dependency);
                }
                break;
            }
            Key::Backspace => {
                home();
                break;
            }
            _ => {}
        }
        draw(stdout, options, *cursor_position);
    }
}


fn execute(choice: &mut MenuItem) {
    dependencies(&choice.name, OperatingSystem::Debian)
}

fn draw(stdout: &mut io::Stdout, dependencies: &Vec<MenuItem>, cursor_position: usize) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    write!(
        stdout,
        "Select the dependencies by using the arrow keys and pressing Space, to finish press Enter. \r\n"
    )
    .unwrap();
    for (index, dependency) in dependencies.iter().enumerate() {
        if index == cursor_position {
            print_color(stdout, "> ", &Color::Blue).unwrap();
        } else {
            print_color(stdout, "  ", &Color::Blue).unwrap();
        }
        let _ = print_color(stdout, &format!("{}\r\n", dependency.name), &Color::Blue);
    }
    write!(stdout, "To exit, type Q or ESC").unwrap();
    stdout.flush().unwrap();
}
