use termion::clear;

use editor::Editor;

impl Editor {
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

    pub fn delete_char(&mut self) {
        let (x, y) = self.pos();
        self.buffer[y].remove(x);
        self.refresh_line();
    }

    pub fn backspace(&mut self) {
        let (x, y) = self.pos();
        if x > 0 {
            self.buffer[y].remove(x - 1);
            self.x -= 1;
        }
        self.refresh_line();
    }

    pub fn refresh_line(&mut self) {
        let (x, y) = self.pos();
        let line = self.buffer[y].clone();
        self.goto(0, y);
        self.print(clear::CurrentLine);
        self.print(line);
        self.right(x);
    }

    pub fn refresh_all(&mut self) {
        let (x, y) = self.pos();
        for i in 0..self.buffer.len() {
            self.goto(0, i);
            self.refresh_line();
        }
        self.goto(x, y);
    }
}
