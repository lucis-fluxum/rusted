use termion::cursor::*;

use editor::Editor;
use editor::Mode;

// TODO: Scrolling when traveling up or down
impl Editor {
    // TODO: Use cursor_pos function once
    // https://github.com/ticki/termion/issues/136 is fixed
    /// Get the current (0, 0)-based position of the cursor.
    pub fn pos(&mut self) -> (usize, usize) {
        // self.output.cursor_pos().unwrap()
        (self.x, self.y)
    }

    /// Set the position of the cursor.
    ///
    /// This is (0, 0)-based cursor movement, instead of (1, 1)-based as termion
    /// provides.
    pub fn goto(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.print(Goto(x as u16 + 1, y as u16 + 1));
    }

    /// Move the cursor left 'n' positions.
    ///
    /// If moving past the left edge: the cursor will move to the end of the
    /// previous line if it exists, otherwise it will move to the origin.
    ///
    /// Will move up multiple lines if needed.
    pub fn left(&mut self, n: usize) {
        let (x, y) = self.pos();
        if n <= x {
            self.goto(x - n, y);
        } else if y > 0 {
            // Previous line exists, calculate leftover movement
            let leftover = n - x - 1;

            // Skip to previous line and move leftover amount
            let end_of_prev_line = self.get_end_of_line(y - 1);
            self.goto(end_of_prev_line, y - 1);
            self.left(leftover);
        } else {
            self.goto(0, 0);
        }
    }

    /// Move the cursor right 'n' positions.
    ///
    /// If moving past the end of a line: the cursor will move to the beginning
    /// of the next line if it exists, otherwise it will move to the end of the
    /// file.
    ///
    /// Will move down multiple lines if needed.
    pub fn right(&mut self, n: usize) {
        let (x, y) = self.pos();
        let end_of_current_line = self.get_end_of_line(y);

        if x + n <= end_of_current_line {
            self.goto(x + n, y);
        } else if y + 1 < self.buffer.len() {
            // Next line exists, calculate leftover movement
            let leftover = n - (end_of_current_line - x) - 1;

            // Skip to next line and move leftover amount
            self.goto(0, y + 1);
            self.right(leftover);
        } else {
            self.goto(end_of_current_line, y);
        }
    }

    /// Move the cursor up 'n' positions.
    ///
    /// If moving to a line shorter than the current line, the cursor will jump
    /// to the end of the shorter line.
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

    /// Move the cursor down 'n' positions.
    ///
    /// If moving to a line shorter than the current line, the cursor will jump
    /// to the end of the shorter line.
    pub fn down(&mut self, n: usize) {
        let (x, y) = self.pos();

        // Don't move past the end of the buffer
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

    /// Determine the ending position of a given line, depending on the current
    /// mode.
    ///
    /// For normal mode: last char of previous line.
    /// For insert mode: one beyond the last char of previous line.
    fn get_end_of_line(&self, n: usize) -> usize {
        match self.mode {
            Mode::Normal => self.buffer[n].len() - 1,
            Mode::Insert => self.buffer[n].len(),
        }
    }
}
