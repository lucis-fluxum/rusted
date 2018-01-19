use termion::cursor::*;

use editor::Editor;

pub struct Cursor<'a> {
    pub editor: &'a mut Editor,
}

impl<'a> Cursor<'a> {
    pub fn pos(&mut self) -> (u16, u16) {
        self.editor.output.cursor_pos().unwrap()
    }

    pub fn goto(&mut self, x: u16, y: u16) {
        self.editor.print(Goto(x, y));
    }

    pub fn left(&mut self, n: u16) {
        self.editor.print(Left(n));
    }

    pub fn right(&mut self, n: u16) {
        self.editor.print(Right(n));
    }

    pub fn up(&mut self, n: u16) {
        self.editor.print(Up(n));
    }

    pub fn down(&mut self, n: u16) {
        self.editor.print(Down(n));
    }
}
