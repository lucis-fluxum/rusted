use termion::cursor::*;

use editor::Editor;
use editor::Mode;

// TODO: Scrolling when traveling up or down
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
            // Previous line exists, calculate leftover movement
            let leftover = n - x - 1;

            // Skip to previous line and move leftover amount
            // Also skips to line above that, if needed.
            let end_of_prev_line = self.get_end_of_line(y - 1);
            self.goto(end_of_prev_line, y - 1);
            self.left(leftover);
        } else {
            // We're on the first line trying to move too far left.
            self.goto(0, 0);
        }
    }

    pub fn right(&mut self, n: usize) {
        let (x, y) = self.pos();
        let end_of_current_line = self.get_end_of_line(y);

        if x + n <= end_of_current_line {
            self.goto(x + n, y);
        } else if y + 1 < self.buffer.len() {
            // Next line exists, calculate leftover movement
            let leftover = n - (end_of_current_line - x) - 1;

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
            let end_of_above_line = self.get_end_of_line(y - n);
            if x > end_of_above_line {
                self.goto(end_of_above_line, y - n);
            } else {
                self.goto(x, y - n);
            }
        } else {
            // Scroll if possible
        }
    }

    pub fn down(&mut self, n: usize) {
        let (x, y) = self.pos();

        if y + n < self.buffer.len() {
            let end_of_below_line = self.get_end_of_line(y + n);
            if x > end_of_below_line {
                self.goto(end_of_below_line, y + n);
            } else {
                self.goto(x, y + n);
            }
        } else {
            // Scroll if possible
        }
    }

    // Normal mode: last char of previous line
    // Insert mode: one beyond the last char
    fn get_end_of_line(&self, n: usize) -> usize {
        match self.mode {
            Mode::Normal => self.buffer[n].len() - 1,
            Mode::Insert => self.buffer[n].len(),
        }
    }
}
