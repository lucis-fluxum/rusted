use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};
use action::*;

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
