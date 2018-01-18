extern crate termion;

mod mode;
mod editor;
mod handler;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::*;
use std::io::{Write, stdout, stdin};

use mode::Mode;
use editor::Editor;

fn main() {
    let mut ed = Editor { 
        mode: Mode::Normal, 
        output: stdout().into_raw_mode().unwrap(),
    };

    write!(ed.output, "{}{}", termion::clear::All, Goto(1, 1)) .unwrap();
    ed.output.flush().unwrap();

    for c in stdin().keys() {
        let c = c.unwrap();
        match ed.mode {
            Mode::Normal => handler::handle_key_for_normal(&c, &mut ed),
            Mode::Insert => handler::handle_key_for_insert(&c, &mut ed)
        }
        handler::handle_key_for_all(&c, &mut ed);
        if c == Key::Ctrl('c') { break; }
        ed.output.flush().unwrap();
    }

    write!(ed.output, "{}{}", termion::clear::All, Goto(1, 1)).unwrap();
}
