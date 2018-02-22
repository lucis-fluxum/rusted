use termion::clear;
use editor::Editor;

impl Editor {
    pub fn setup_command_line(&mut self) {
        // FIXME: Use a more intuitive y coordinate
        self.goto(0, 50);
        self.print(":");
        self.right(1);
        self.command.clear();
    }

    // TODO: Note that self.command remains unchanged after teardown. Consider
    // saving this in the command history.
    pub fn teardown_command_line(&mut self) {
        self.print(clear::CurrentLine);
        self.goto(0, 0); // TODO: Go to last known position
    }
}
