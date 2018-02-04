use std::io;
use termion::event::Key;
use termion::input::TermRead;

use editor::{Editor, Mode};

impl Editor {
    pub fn respond_to_keys(&mut self) {
        for key in io::stdin().keys().map(|key| key.unwrap()) {
            match self.mode {
                Mode::Normal => self.normal_mode_key(&key),
                Mode::Insert => self.insert_mode_key(&key),
            }
            self.any_mode_key(&key);
            if key == Key::Ctrl('c') {
                break;
            }
        }
    }

    fn insert_mode_key(&mut self, key: &Key) {
        match *key {
            Key::Char(c) => self.insert_char(c),
            Key::Backspace => self.backspace(),
            Key::Delete => self.delete_char(),
            _ => {}
        }
    }

    fn normal_mode_key(&mut self, key: &Key) {
        self.do_vim_move(key);
        match *key {
            Key::Char('i') => self.set_mode(Mode::Insert),
            _ => {}
        }
    }

    fn any_mode_key(&mut self, key: &Key) {
        self.do_arrow_key_move(key);
        match *key {
            Key::Esc => self.set_mode(Mode::Normal),
            _ => {}
        }
    }

    fn do_vim_move(&mut self, key: &Key) {
        match *key {
            Key::Char('h') => self.left(1),
            Key::Char('l') => self.right(1),
            Key::Char('k') => self.up(1),
            Key::Char('j') => self.down(1),
            // TODO: Backspace moves to end of previous line
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
