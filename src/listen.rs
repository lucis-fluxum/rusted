use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};
use action::movement;

pub fn respond_to_keys(mut editor: Editor) {
    for key in io::stdin().keys().map(|key| key.unwrap()) {
        match editor.mode {
            Mode::Normal => normal_mode_key(&key, &mut editor),
            Mode::Insert => insert_mode_key(&key, &mut editor),
        }
        any_mode_key(&key, &mut editor);
        if key == Key::Ctrl('c') {
            break;
        }
    }
}

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
        Key::Char('i') => editor.set_mode(Mode::Insert),
        _ => {}
    }
}

pub fn any_mode_key(key: &Key, editor: &mut Editor) {
    movement::do_arrow_key_move(key, editor);
    match *key {
        Key::Esc => editor.set_mode(Mode::Normal),
        _ => {}
    }
}
