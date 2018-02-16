use termion::cursor::*;

use editor::Editor;
use editor::Mode;

// TODO: Scrolling when traveling up or down
// TODO: Jumping back to end of line when moving up/down from longer line
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
        } else if y > 0 {
            // Normal mode: last char of previous line
            // Insert mode: one beyond the last char
            let end = match self.mode {
                Mode::Normal => self.buffer[y - 1].len() - 1,
                Mode::Insert => self.buffer[y - 1].len(),
            };

            // Move to end of previous line instead
            self.goto(end, y - 1);
        }
    }

    pub fn right(&mut self, n: usize) {
        let (x, y) = self.pos();
        let current_line_len = self.buffer[y].len();

        // Depending on mode, do or don't move past the end of the line
        let regular_move = match self.mode {
            Mode::Normal => x + n < current_line_len,
            Mode::Insert => x + n <= current_line_len,
        };

        if regular_move {
            self.goto(x + n, y);
        } else if y + 1 < self.buffer.len() {
            // Next line exists, calculate leftover movement
            let leftover = match self.mode {
                Mode::Normal => n - (current_line_len - x),
                Mode::Insert => n - (current_line_len - x) - 1,
            };

            // Skip to next line and move leftover amount
            // This also skips to the line after, if needed.
            self.goto(0, y + 1);
            self.right(leftover);
        }
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
