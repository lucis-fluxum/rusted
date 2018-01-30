use termion::cursor::*;

use editor::Editor;

pub struct Cursor<'a> {
    pub editor: &'a mut Editor,
}

// TODO: Scrolling when traveling up or down
// TODO: Jumping to next/previous lines when moving right or left
impl<'a> Cursor<'a> {
    pub fn pos(&mut self) -> (u16, u16) {
        // TODO: Use cursor_pos function once
        // https://github.com/ticki/termion/issues/136 is fixed
        // self.editor.output.cursor_pos().unwrap()
        (self.editor.x, self.editor.y)
    }

    // (0, 0)-based movement, instead of (1, 1)-based.
    pub fn goto(&mut self, x: u16, y: u16) {
        self.editor.x = x;
        self.editor.y = y;
        self.editor.print(Goto(x + 1, y + 1));
    }

    pub fn left(&mut self, n: u16) {
        let (x, y) = self.pos();
        // Don't move past the left edge of the terminal
        if n <= x {
            self.goto(x - n, y);
        } else {
            // Move to previous line if possible
        }
    }

    pub fn right(&mut self, n: u16) {
        let (x, y) = self.pos();
        // Depending on mode, do/don't move past the end of the line
        // Move to next line if possible
        self.goto(x + n, y);
    }

    pub fn up(&mut self, n: u16) {
        let (x, y) = self.pos();
        // Don't move past the top of the terminal
        if n <= y {
            self.goto(x, y - n);
        } else {
            // Scroll if possible
        }
    }

    pub fn down(&mut self, n: u16) {
        let (x, y) = self.pos();
        self.goto(x, y + n);
    }
}
