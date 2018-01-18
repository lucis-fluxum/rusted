use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;
use termion::cursor::Goto;

use mode::Mode;

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
        self.goto(1, 1);
    }

    pub fn clear(&mut self) {
        write!(self.output, "{}", clear::All).unwrap();
    }

    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.output, "{}", Goto(x, y)).unwrap();
    }

    pub fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}
