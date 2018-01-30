extern crate termion;

mod editor;
mod cursor;
mod action;

use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};

fn main() {
    let mut ed = Editor::new();
    ed.reset();

    for key in io::stdin().keys() {
        let key = key.unwrap();
        match ed.mode {
            Mode::Normal => action::normal_mode_key(&key, &mut ed),
            Mode::Insert => action::insert_mode_key(&key, &mut ed),
        }
        action::any_mode_key(&key, &mut ed);
        if key == Key::Ctrl('c') {
            break;
        }
    }

    ed.reset();
}
