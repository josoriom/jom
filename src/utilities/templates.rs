use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::main_menu;
use crate::{print_color, Color};
struct TemplateItem {
    name: String,
    color: Color,
}

impl TemplateItem {
    fn new(name: &str, color: Color) -> TemplateItem {
        TemplateItem {
            name: name.to_string(),
            color,
        }
    }
}

pub fn templates() {
    let templates = vec![
        TemplateItem::new("Common Javascript", Color::Green),
        TemplateItem::new("ES Javascript", Color::Green),
        TemplateItem::new("Typescript", Color::Blue),
        TemplateItem::new("Python", Color::Yellow),
    ];

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor_position = 0;

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    display_templates(&mut stdout, &templates, cursor_position);

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') | Key::Esc | Key::Ctrl('c') => break,
            Key::Up => {
                if cursor_position > 0 {
                    cursor_position -= 1;
                }
            }
            Key::Down => {
                if cursor_position < templates.len() - 1 {
                    cursor_position += 1;
                }
            }
            Key::Char('\n') => {
                execute_action(&templates[cursor_position]);
                break;
            }
            Key::Backspace => {
                main_menu();
                break;
            }
            _ => {}
        }
        display_templates(&mut stdout, &templates, cursor_position);
    }
}

fn execute_action(template: &TemplateItem) {
    println!("Selected template: {}\r\n", template.name);
}

fn display_templates(
    stdout: &mut io::Stdout,
    dependencies: &Vec<TemplateItem>,
    cursor_position: usize,
) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    write!(
        stdout,
        "Select the template by using the arrow keys and pressing Enter. \r\n"
    )
    .unwrap();
    for (index, dependency) in dependencies.iter().enumerate() {
        if index == cursor_position {
            write!(stdout, "> ").unwrap();
        } else {
            write!(stdout, "  ").unwrap();
        }
        write!(stdout, "{}\r\n", dependency.name).unwrap();
    }
    write!(stdout, "To exit, type Q or ESC. \r\n ").unwrap();
    stdout.flush().unwrap();
}
