use std::io::{stdout, Stdout, Write};
use std::fmt::Display;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;

use cursor::Cursor;

pub enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    pub mode: Mode,
    pub output: RawTerminal<Stdout>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            mode: Mode::Normal,
            output: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn reset(&mut self) {
        self.clear();
        self.cursor().goto(1, 1);
    }

    pub fn clear(&mut self) {
        write!(self.output, "{}", clear::All).unwrap();
    }

    pub fn flush(&mut self) {
        self.output.flush().unwrap();
    }

    pub fn cursor(&mut self) -> Cursor {
        Cursor { editor: self }
    }

    pub fn print<T: Display>(&mut self, item: T) {
        write!(self.output, "{}", item).unwrap();
    }
}
