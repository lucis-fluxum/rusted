extern crate termion;

mod mode;
mod editor;
mod handler;

use std::io::{stdin, Write};
use termion::event::Key;
use termion::input::TermRead;

use editor::Editor;
use mode::Mode;

fn main() {
    let mut ed = Editor::new();
    ed.reset();
    ed.flush();

    for c in stdin().keys() {
        let c = c.unwrap();
        match ed.mode {
            Mode::Normal => handler::normal_mode_key(&c, &mut ed),
            Mode::Insert => handler::insert_mode_key(&c, &mut ed),
        }
        handler::any_mode_key(&c, &mut ed);
        if c == Key::Ctrl('c') {
            break;
        }
        ed.flush();
    }

    ed.reset();
}
