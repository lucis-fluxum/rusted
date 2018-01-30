mod movement;

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
    movement::do_vim_move(key, editor);
    match *key {
        Key::Char('i') => {
            editor.mode = Mode::Insert;
        }
        _ => {}
    }
}

pub fn any_mode_key(key: &Key, editor: &mut Editor) {
    movement::do_arrow_key_move(key, editor);
    match *key {
        Key::Esc => {
            editor.mode = Mode::Normal;
        }
        _ => {}
    }
}
