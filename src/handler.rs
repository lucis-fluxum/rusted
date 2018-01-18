use std::io::Write;
use termion::cursor::*;
use termion::event::Key;

use editor::{Editor, Mode};

pub fn insert_mode_key(key: &Key, ed: &mut Editor) {
    match *key {
        Key::Char(c) => write!(ed.output, "{}", c).unwrap(),
        Key::Backspace => write!(ed.output, "\x08 \x08").unwrap(),
        _ => {}
    }
}

pub fn normal_mode_key(key: &Key, ed: &mut Editor) {
    match *key {
        Key::Char('i') => {
            ed.mode = Mode::Insert;
        }
        Key::Char('h') => write!(ed.output, "{}", Left(1)).unwrap(),
        Key::Char('l') => write!(ed.output, "{}", Right(1)).unwrap(),
        Key::Char('k') => write!(ed.output, "{}", Up(1)).unwrap(),
        Key::Char('j') => write!(ed.output, "{}", Down(1)).unwrap(),
        _ => {}
    }
}

pub fn any_mode_key(key: &Key, ed: &mut Editor) {
    match *key {
        Key::Left => write!(ed.output, "{}", Left(1)).unwrap(),
        Key::Right => write!(ed.output, "{}", Right(1)).unwrap(),
        Key::Up => write!(ed.output, "{}", Up(1)).unwrap(),
        Key::Down => write!(ed.output, "{}", Down(1)).unwrap(),
        Key::Esc => {
            ed.mode = Mode::Normal;
        }
        _ => {}
    }
}
