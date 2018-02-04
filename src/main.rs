extern crate termion;

mod editor;
mod action;
mod listen;

use editor::Editor;

fn main() {
    let mut editor = Editor::new();
    editor.respond_to_keys();
}
