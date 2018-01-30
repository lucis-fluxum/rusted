extern crate termion;

mod editor;
mod cursor;
mod action;
mod listen;

use editor::Editor;

fn main() {
    let editor = Editor::new();
    listen::respond_to_keys(editor);
}
