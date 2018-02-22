use editor::Editor;

impl Editor {
    pub fn setup_command_line(&mut self) {
        self.goto(0, 50);
        self.print(":");
        self.right(1);
        self.command.clear();
    }

    pub fn insert_command_char(&mut self, c: char) {
        let (x, _) = self.pos();
        let c = c.to_string();
        self.command.insert_str(x - 1, &c);
        self.right(1);
        self.refresh_line();
    }
}
