use termion::clear;

use editor::{Editor, Mode};

impl Editor {
    // TODO: Inserting a newline inside a word should split the word onto a new
    // line
    /// Insert a character at the cursor's current position.
    ///
    /// If the character is a newline, an empty line is added to the buffer and
    /// the cursor is moved to the beginning of the new line.
    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.pos();
        let c = c.to_string();
        match self.mode {
            Mode::Insert | Mode::Normal => {
                // TODO: Handle whitespace somewhere else
                match c.as_str() {
                    "\n" => {
                        self.goto(0, y + 1);
                        self.buffer.push(String::new());
                    }
                    _ => {
                        self.buffer[y].insert_str(x, &c);
                        self.right(1);
                    }
                }
            }
            Mode::Command => {
                self.command.insert_str(x - 1, &c);
                self.right(1);
            }
        }

        self.refresh_line();
    }

    // TODO: Deleting one after the end of a line should join it with the line below
    /// Delete the character under the cursor.
    pub fn delete_char(&mut self) {
        let (x, y) = self.pos();
        self.buffer[y].remove(x);
        self.refresh_line();
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
        let (x, y) = self.pos();
        self.goto(0, 0);
        self.print(clear::All);
        let contents = self.buffer.join("\r\n");
        self.print(contents);
        self.goto(x, y);
    }
}
