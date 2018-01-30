use termion::event::Key;

use editor::Editor;

impl Editor {
    pub fn do_vim_move(&mut self, key: &Key) {
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

    pub fn do_arrow_key_move(&mut self, key: &Key) {
        match *key {
            Key::Left => self.left(1),
            Key::Right => self.right(1),
            Key::Up => self.up(1),
            Key::Down => self.down(1),
            _ => {}
        }
    }
}
