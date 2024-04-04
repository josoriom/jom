use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::dependencies_action::dependencies_action;
use crate::templates;
struct MainMenuOption {
    name: String,
}

impl MainMenuOption {
    fn new(name: &str) -> MainMenuOption {
        MainMenuOption {
            name: name.to_string(),
        }
    }
}

pub fn home() {
    let main_menu_options = vec![
        MainMenuOption::new("Dependencies"),
        MainMenuOption::new("Templates"),
    ];
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor_position = 0;
    write!(stdout, "{}", termion::clear::All).unwrap();
    draw(&mut stdout, &main_menu_options, cursor_position);
    action_controller(stdin, &mut stdout, &main_menu_options, &mut cursor_position);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn action_controller(
    stdin: io::Stdin,
    stdout: &mut io::Stdout,
    main_menu_options: &Vec<MainMenuOption>,
    cursor_position: &mut usize,
) {
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') | Key::Esc | Key::Ctrl('c') => break,
            Key::Up => {
                if *cursor_position > 0 {
                    *cursor_position -= 1;
                }
            }
            Key::Down => {
                if *cursor_position < main_menu_options.len() - 1 {
                    *cursor_position += 1;
                }
            }
            Key::Char('\n') => {
                execute(&main_menu_options[*cursor_position]);
                break;
            }
            _ => {}
        }
        draw(stdout, main_menu_options, *cursor_position);
        stdout.flush().unwrap();
    }
}

fn execute(template: &MainMenuOption) {
    if template.name == "Dependencies" {
        dependencies_action();
    } else {
        templates();
    }
}

fn draw(
    stdout: &mut io::Stdout,
    options: &Vec<MainMenuOption>,
    cursor_position: usize,
) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    write!(stdout, "What would you like to do?. \r\n").unwrap();
    for (index, dependency) in options.iter().enumerate() {
        if index == cursor_position {
            write!(stdout, "> ").unwrap();
        } else {
            write!(stdout, "  ").unwrap();
        }
        write!(stdout, "{}\r\n", dependency.name).unwrap();
    }
    write!(stdout, "To exit, type Q, ESC or Ctrl + C. \r\n ").unwrap();
    stdout.flush().unwrap();
}
