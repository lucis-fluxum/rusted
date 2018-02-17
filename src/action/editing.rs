use termion::clear;

use editor::Editor;

impl Editor {
    /// Insert a character at the cursor's current position.
    ///
    /// If the character is a newline, an empty line is added to the buffer and
    /// the cursor is moved to the beginning of the new line.
    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.pos();
        let c = c.to_string();
        // TODO: Handle whitespace somewhere else
        match c.as_str() {
            "\n" => {
                self.goto(0, y + 1);
                self.buffer.push(String::new());
            }
            _ => {
                self.buffer[y].insert_str(x, &c);
                self.x += 1;
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
        if x > 0 {
            self.buffer[y].remove(x - 1);
            self.x -= 1;
        }
        self.refresh_line();
    }

    /// Reload the contents of a line from the buffer. Useful when modifying a
    /// line in place.
    pub fn refresh_line(&mut self) {
        let (x, y) = self.pos();
        let line = self.buffer[y].clone();
        self.goto(0, y);
        self.print(clear::CurrentLine);
        self.print(line);
        self.right(x);
    }

    // TODO: What if the buffer is too large to fit into the current screen?
    /// Reload the contents of the entire screen from the buffer.
    pub fn refresh_all(&mut self) {
        let (x, y) = self.pos();
        for i in 0..self.buffer.len() {
            self.goto(0, i);
            self.refresh_line();
        }
        self.goto(x, y);
    }
}
