use std::io::Write;
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
        Key::Char('h') => ed.cursor().left(1),
        Key::Char('l') => ed.cursor().right(1),
        Key::Char('k') => ed.cursor().up(1),
        Key::Char('j') => ed.cursor().down(1),
        _ => {}
    }
}

pub fn any_mode_key(key: &Key, ed: &mut Editor) {
    match *key {
        Key::Left => ed.cursor().left(1),
        Key::Right => ed.cursor().right(1),
        Key::Up => ed.cursor().up(1),
        Key::Down => ed.cursor().down(1),
        Key::Esc => {
            ed.mode = Mode::Normal;
        }
        _ => {}
    }
}
