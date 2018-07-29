use crate::editor::{Editor, Mode};
use termion::clear;

impl Editor {
    crate fn setup_command_line(&mut self) {
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
    crate fn teardown_command_line(&mut self) {
        self.print(clear::CurrentLine);
        self.restore_pos();
    }

    crate fn backspace_command_line(&mut self) {
        if self.command.is_empty() {
            self.quit_command_mode();
        } else {
            self.backspace();
        }
    }

    crate fn quit_command_mode(&mut self) {
        self.teardown_command_line();
        self.set_mode(Mode::Normal);
    }

    // TODO: Add behaviors for specific commands
    crate fn execute_command(&mut self) {
        self.print("[execute]");
    }
}
