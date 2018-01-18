use std::io::Write;
use termion::cursor::*;
use termion::event::Key;

use editor::Editor;
use mode::Mode;

pub fn handle_key_for_insert(c: &Key, ed: &mut Editor) {
    match *c {
        Key::Char(c) => write!(ed.output, "{}", c).unwrap(),
        Key::Backspace => write!(ed.output, "\x08 \x08").unwrap(),
        _ => {}
    }
}

pub fn handle_key_for_normal(c: &Key, ed: &mut Editor) {
    match *c {
        Key::Char('i') => {
            ed.mode = Mode::Insert;
        },
        _ => {}
    }
}

pub fn handle_key_for_all(c: &Key, ed: &mut Editor) {
    match *c {
        Key::Left => write!(ed.output, "{}", Left(1)).unwrap(),
        Key::Right => write!(ed.output, "{}", Right(1)).unwrap(),
        Key::Up => write!(ed.output, "{}", Up(1)).unwrap(),
        Key::Down => write!(ed.output, "{}", Down(1)).unwrap(),
        Key::Esc => { ed.mode = Mode::Normal; }
        _ => {}
    }
}
