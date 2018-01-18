use std::io::Write;
use termion::cursor::Goto;

use editor::Editor;

pub struct Cursor<'a> {
    pub editor: &'a mut Editor,
}

impl<'a> Cursor<'a> {
    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.editor.output, "{}", Goto(x, y)).unwrap();
    }
}
