use termion::clear;

use crate::editor::{Editor, Mode};

impl Editor {
    /// Insert a character at the cursor's current position.
    ///
    /// If the character is a newline, the rest of the current line is split
    /// off into a new line and the cursor is moved to the next line.
    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.pos();
        let c = c.to_string();
        match self.mode {
            Mode::Insert => {
                // TODO: Handle whitespace somewhere else
                match c.as_str() {
                    "\n" => {
                        let leftover = self.buffer[y].split_off(x);
                        self.buffer.insert(y + 1, leftover);
                        self.refresh_all();
                        self.goto(0, y + 1);
                    }
                    _ => {
                        self.buffer[y].insert_str(x, &c);
                        self.right(1);
                        self.refresh_line();
                    }
                }
            }
            Mode::Command => {
                self.command.insert_str(x - 1, &c);
                self.right(1);
                self.refresh_line();
            }
            // Shouldn't be inserting anything in normal mode
            Mode::Normal => {}
        }
    }

    // TODO: Deleting a character from a line wrapped around to a new line is
    // broken
    /// Delete the character under the cursor. If the cursor is one past the
    /// end of a line, it joins the current line with the line below, if
    /// possible.
    pub fn delete_char(&mut self) {
        let (x, y) = self.pos();
        if x == self.buffer[y].len() {
            if y + 1 < self.buffer.len() {
                let next_line = self.buffer.remove(y + 1);
                self.buffer[y] += &next_line;
                self.refresh_all();
            }
        } else {
            self.buffer[y].remove(x);
            self.refresh_line();
        }
    }

    /// Delete the character just before the cursor, move the cursor back 1
    /// character.
    pub fn backspace(&mut self) {
        let (x, y) = self.pos();
        match self.mode {
            Mode::Insert => {
                if x > 0 {
                    self.buffer[y].remove(x - 1);
                    self.left(1);
                }
            }
            Mode::Command => {
                if x > 1 {
                    self.command.remove(x - 2);
                    self.left(1);
                }
            }
            _ => {}
        }
        self.refresh_line();
    }

    /// Reload the contents of a line from the buffer. Useful when modifying a
    /// line in place. In command mode, refresh the command line.
    pub fn refresh_line(&mut self) {
        let (x, y) = self.pos();
        let line = match self.mode {
            Mode::Insert | Mode::Normal => self.buffer[y].clone(),
            Mode::Command => format!(":{}", self.command),
        };
        self.goto(0, y);
        self.print(clear::CurrentLine);
        self.print(line);
        self.right(x);
    }

    // TODO: What if the buffer is too large to fit into the current screen?
    /// Reload the contents of the entire screen from the buffer.
    pub fn refresh_all(&mut self) {
        self.save_pos();
        self.goto(0, 0);
        self.print(clear::All);
        let contents = self.buffer.join("\r\n");
        self.print(contents);
        self.restore_pos();
    }
}
