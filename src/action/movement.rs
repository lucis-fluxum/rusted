use termion::cursor::*;

use editor::Editor;

// TODO: Scrolling when traveling up or down
// TODO: Jumping to next/previous lines when moving right or left
impl Editor {
    pub fn pos(&mut self) -> (usize, usize) {
        // TODO: Use cursor_pos function once
        // https://github.com/ticki/termion/issues/136 is fixed
        // self.output.cursor_pos().unwrap()
        (self.x, self.y)
    }

    // (0, 0)-based movement, instead of (1, 1)-based.
    pub fn goto(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.print(Goto(x as u16 + 1, y as u16 + 1));
    }

    pub fn left(&mut self, n: usize) {
        let (x, y) = self.pos();
        // Don't move past the left edge of the terminal
        if n <= x {
            self.goto(x - n, y);
        } else {
            // Move to previous line if possible
        }
    }

    pub fn right(&mut self, n: usize) {
        let (x, y) = self.pos();
        // TODO: Depending on mode, do/don't move past the end of the line
        // Move to next line if possible
        self.goto(x + n, y);
    }

    pub fn up(&mut self, n: usize) {
        let (x, y) = self.pos();
        // Don't move past the top of the terminal
        if n <= y {
            self.goto(x, y - n);
        } else {
            // Scroll if possible
        }
    }

    pub fn down(&mut self, n: usize) {
        let (x, y) = self.pos();
        self.goto(x, y + n);
    }
}
