use termion::clear;
use editor::Editor;

impl Editor {
    pub fn setup_command_line(&mut self) {
        self.saved_pos = self.pos();
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
        let (old_x, old_y) = self.saved_pos;
        self.goto(old_x, old_y);
    }
}
