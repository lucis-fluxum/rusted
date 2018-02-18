extern crate termion;

mod editor;
mod action;
mod listen;

use editor::Editor;

fn main() {
    let filename = std::env::args().nth(1).unwrap_or(String::from(""));
    let mut editor = Editor::new();

    if !filename.is_empty() {
        editor.load(&filename);
    }

    editor.respond_to_keys();

    if !filename.is_empty() {
        editor.save(&filename);
    } else {
        // TODO: Prompt for filename
    }
}
