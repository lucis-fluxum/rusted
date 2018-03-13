use termion::clear;
use editor::Editor;

impl Editor {
    pub fn setup_command_line(&mut self) {
        self.save_pos();
        let (_, y) = self.size();
        self.goto(0, y);
        self.print(clear::CurrentLine);
        self.print(":");
        self.right(1);
        self.command.clear();
    }

    // TODO: Note that self.command remains unchanged after teardown. Consider
    // saving this in the command history.
    pub fn teardown_command_line(&mut self) {
        self.print(clear::CurrentLine);
        self.restore_pos();
    }
}
