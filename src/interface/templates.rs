use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::home;
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

    let _ = display_templates(&mut stdout, &templates, cursor_position);

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
                home();
                break;
            }
            _ => {}
        }
        let _ = display_templates(&mut stdout, &templates, cursor_position);
    }
}

fn execute_action(template: &TemplateItem) {
    println!("Selected template: {}\r\n", template.name);
}

fn display_templates(
    stdout: &mut dyn std::io::Write,
    templates: &Vec<TemplateItem>,
    cursor_position: usize,
) -> std::io::Result<()> {
    print_color(stdout, &termion::clear::All.to_string(), &Color::Yellow)?;
    print_color(
        stdout,
        &termion::cursor::Goto(1, 2).to_string(),
        &Color::White,
    )?;
    print_color(
        stdout,
        "Select the template by using the arrow keys and pressing Enter\r\n",
        &Color::White,
    )?;
    for (index, template) in templates.iter().enumerate() {
        if index == cursor_position {
            print_color(stdout, "> ", &template.color)?;
        } else {
            print_color(stdout, "  ", &template.color)?;
        }
        print_color(stdout, &format!("{}\r\n", template.name), &template.color)?;
    }
    print_color(
        stdout,
        "To return to the main menu, press the Backspace\r\n",
        &Color::Red,
    )?;
    print_color(
        stdout,
        "To exit, press the Q or ESC key.\r\n",
        &Color::White,
    )?;
    stdout.flush()?;
    Ok(())
}
