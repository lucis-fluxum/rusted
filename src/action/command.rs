use editor::Editor;
use termion::cursor::Goto;

impl Editor {
    pub fn setup_command_line(&mut self) {
        self.goto(0, 50);
        self.print(":");
        self.command = String::new();
    }
}
