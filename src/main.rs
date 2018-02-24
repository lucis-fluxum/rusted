extern crate termion;

mod editor;
mod action;
mod listen;

use editor::Editor;

fn main() {
    // TODO: Use some kind of logging crate for debug information
    let debug_info = {
        let mut editor = Editor::new();
        editor.filename = std::env::args().nth(1).unwrap_or_default();

        editor.load();
        editor.respond_to_keys();
        editor.save();

        editor.debug_info()
    };
    println!("debug info: {:?}", debug_info);
}
