use std::io::Write;
use termion::cursor::*;

use editor::Editor;

pub struct Cursor<'a> {
    pub editor: &'a mut Editor,
}

impl<'a> Cursor<'a> {
    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.editor.output, "{}", Goto(x, y)).unwrap();
    }

    pub fn left(&mut self, n: u16) {
        write!(self.editor.output, "{}", Left(n)).unwrap();
    }

    pub fn right(&mut self, n: u16) {
        write!(self.editor.output, "{}", Right(n)).unwrap();
    }

    pub fn up(&mut self, n: u16) {
        write!(self.editor.output, "{}", Up(n)).unwrap();
    }

    pub fn down(&mut self, n: u16) {
        write!(self.editor.output, "{}", Down(n)).unwrap();
    }
}
