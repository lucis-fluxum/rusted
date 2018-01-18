extern crate termion;

mod editor;
mod cursor;
mod handler;

use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};

fn main() {
    let mut ed = Editor::new();
    ed.reset();
    ed.flush();

    for key in io::stdin().keys() {
        let key = key.unwrap();
        match ed.mode {
            Mode::Normal => handler::normal_mode_key(&key, &mut ed),
            Mode::Insert => handler::insert_mode_key(&key, &mut ed),
        }
        handler::any_mode_key(&key, &mut ed);
        if key == Key::Ctrl('c') {
            break;
        }
        ed.flush();
    }

    ed.reset();
}
