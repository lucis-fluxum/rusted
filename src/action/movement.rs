use termion::event::Key;

use editor::{Editor, Mode};

pub fn do_vim_move(key: &Key, editor: &mut Editor) {
    match *key {
        Key::Char('h') => editor.cursor().left(1),
        Key::Char('l') => editor.cursor().right(1),
        Key::Char('k') => editor.cursor().up(1),
        Key::Char('j') => editor.cursor().down(1),
        // TODO: Backspace moves to end of previous line
        // TODO: 'w', 'b', 'e'?
        _ => {}
    }
}

pub fn do_arrow_key_move(key: &Key, editor: &mut Editor) {
    match *key {
        Key::Left => editor.cursor().left(1),
        Key::Right => editor.cursor().right(1),
        Key::Up => editor.cursor().up(1),
        Key::Down => editor.cursor().down(1),
        _ => {}
    }
}
