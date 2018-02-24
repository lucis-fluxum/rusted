extern crate termion;

mod editor;
mod action;
mod listen;

use editor::Editor;

fn main() {
    // TODO: Use some kind of logging crate for debug information
    let debug_info = {
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

        editor.debug_info()
    };
    println!("debug info: {:?}", debug_info);
}
