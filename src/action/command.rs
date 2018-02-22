use editor::Editor;

impl Editor {
    pub fn setup_command_line(&mut self) {
        // FIXME: Use a more intuitive y coordinate
        self.goto(0, 50);
        self.print(":");
        self.right(1);
        self.command.clear();
    }
}
