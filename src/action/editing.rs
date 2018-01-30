use termion::clear;

use editor::Editor;

impl Editor {
    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.cursor().pos();
        self.buffer[y].insert_str(x, &c.to_string());
        self.x += 1;
        let c = c.to_string();
        // TODO: Handle whitespace somewhere else
        match c.as_str() {
            "\n" => {
                self.cursor().goto(0, y + 1);
                self.buffer.push(String::new());
            }
            _ => {
                self.buffer[y].insert_str(x, &c);
                self.x += 1;
            }
        }
        self.refresh_line();
    }

    pub fn delete_char(&mut self) {
        let (x, y) = self.cursor().pos();
        self.buffer[y].remove(x);
        self.refresh_line();
    }

    pub fn backspace(&mut self) {
        let (x, y) = self.cursor().pos();
        if x > 0 {
            self.buffer[y].remove(x - 1);
            self.x -= 1;
        }
        self.refresh_line();
    }

    fn refresh_line(&mut self) {
        let (x, y) = self.cursor().pos();
        let line = self.buffer[y].clone();
        self.cursor().goto(0, y);
        self.print(clear::CurrentLine);
        self.print(line);
        self.cursor().right(x);
    }
}
