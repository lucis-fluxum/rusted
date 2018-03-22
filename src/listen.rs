use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};

impl Editor {
    pub fn respond_to_keys(&mut self) {
        for key in io::stdin().keys().map(|key| key.unwrap()) {
            if key == Key::Ctrl('c') {
                // TODO: Figure out how to exit Vim
                // That is, use :q instead
                break;
            }
            match self.mode {
                Mode::Normal => self.normal_mode_key(&key),
                Mode::Insert => self.insert_mode_key(&key),
                Mode::Command => self.command_mode_key(&key),
            }
            self.reset_status();
        }
    }

    fn normal_mode_key(&mut self, key: &Key) {
        match *key {
            // Switching modes
            Key::Char('i') => self.set_mode(Mode::Insert),
            Key::Char(':') => self.set_mode(Mode::Command),

            // Movement
            Key::Left | Key::Right | Key::Up | Key::Down => self.do_arrow_key_move(key),

            // Misc
            Key::Ctrl('s') => self.save(),

            // Fallback to general Vim bindings
            other => self.do_vim_move(&other),
        }
    }

    fn insert_mode_key(&mut self, key: &Key) {
        match *key {
            // Editing
            Key::Char(c) => self.insert_char(c),
            Key::Backspace => self.backspace(),
            Key::Delete => self.delete_char(),

            // Switching modes
            Key::Esc => {
                // TODO: Move to movement module
                let (x, y) = self.pos();
                let line_length = self.buffer[y].len();
                self.set_mode(Mode::Normal);
                if line_length > 0 && x == line_length {
                    self.left(1);
                }
            }

            // Movement
            Key::Left | Key::Right | Key::Up | Key::Down => self.do_arrow_key_move(key),

            // Misc
            Key::Ctrl('s') => self.save(),
            _ => {}
        }
    }

    fn command_mode_key(&mut self, key: &Key) {
        match *key {
            // TODO: Execute command
            Key::Char('\n') => self.print("[execute]"),
            Key::Char(c) => self.insert_char(c),
            Key::Backspace => self.backspace_command_line(),

            // Switching modes
            Key::Esc => self.quit_command_mode(),

            // Movement
            Key::Left | Key::Right => self.do_arrow_key_move(key),
            _ => {}
        }
    }

    fn do_vim_move(&mut self, key: &Key) {
        match *key {
            Key::Char('h') | Key::Backspace => self.left(1),
            Key::Char('l') => self.right(1),
            Key::Char('k') => self.up(1),
            Key::Char('j') => self.down(1),
            // TODO: 'w', 'b', 'e'?
            _ => {}
        }
    }

    fn do_arrow_key_move(&mut self, key: &Key) {
        match *key {
            Key::Left => self.left(1),
            Key::Right => self.right(1),
            Key::Up => self.up(1),
            Key::Down => self.down(1),
            _ => {}
        }
    }
}
