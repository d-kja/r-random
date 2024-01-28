mod config;
mod controls;
mod screen;

use controls::{move_line, Move};
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    // Initial rendering
    let mut selected_line: u16 = 1;
    let total_lines = render(selected_line, &mut stdout);

    for event_input in stdin.events() {
        let event = event_input.unwrap();

        match event {
            // Exit terminal
            Event::Key(Key::Char('q')) => {
                flush(&mut stdout);
                break;
            }
            Event::Key(Key::Ctrl('c')) => {
                flush(&mut stdout);
                break;
            }

            // Control keybinds
            Event::Key(Key::Up) => {
                move_line(Move::UP, &mut selected_line, total_lines);
            }
            Event::Key(Key::Down) => {
                move_line(Move::DOWN, &mut selected_line, total_lines);
            }
            Event::Key(Key::Char('\t')) => write!(stdout, "tab").unwrap(),
            Event::Key(Key::Char(' ')) => write!(stdout, "space").unwrap(),

            _ => (),
        }

        // Re-rendering based on actions
        render(selected_line, &mut stdout);
    }
}

fn render(
    current_line: u16,
    stdout: &mut MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
) -> u16 {
    let (to_render, total_lines) = screen::render(current_line);

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        to_render
    )
    .unwrap();

    stdout.flush().unwrap();

    total_lines
}

fn flush(stdout: &mut MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>) {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
    )
    .unwrap();

    stdout.flush().unwrap();
}
