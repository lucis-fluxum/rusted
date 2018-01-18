use termion::event::Key;

use editor::{Editor, Mode};

pub fn insert_mode_key(key: &Key, editor: &mut Editor) {
    match *key {
        Key::Char(c) => editor.print(c),
        Key::Backspace => editor.print("\x08 \x08"),
        _ => {}
    }
}

pub fn normal_mode_key(key: &Key, editor: &mut Editor) {
    match *key {
        Key::Char('i') => {
            editor.mode = Mode::Insert;
        }
        Key::Char('h') => editor.cursor().left(1),
        Key::Char('l') => editor.cursor().right(1),
        Key::Char('k') => editor.cursor().up(1),
        Key::Char('j') => editor.cursor().down(1),
        _ => {}
    }
}

pub fn any_mode_key(key: &Key, editor: &mut Editor) {
    match *key {
        Key::Left => editor.cursor().left(1),
        Key::Right => editor.cursor().right(1),
        Key::Up => editor.cursor().up(1),
        Key::Down => editor.cursor().down(1),
        Key::Esc => {
            editor.mode = Mode::Normal;
        }
        _ => {}
    }
}
