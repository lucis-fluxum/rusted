use std::io::{stdout, Stdout, Write};
use std::fmt::Display;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;

use cursor::Cursor;

pub enum Mode {
    Normal,
    Insert,
    // TODO: Visual mode, selectable regions, copy/paste, etc
}

pub struct Editor {
    pub mode: Mode,
    pub output: RawTerminal<Stdout>,
    pub x: u16,
    pub y: u16,
    // TODO: Add positional information (origin, width, height)
}

// TODO: Initialization with background/foreground colors
impl Editor {
    pub fn new() -> Editor {
        Editor {
            mode: Mode::Normal,
            output: stdout().into_raw_mode().unwrap(),
            x: 0,
            y: 0,
        }
    }

    pub fn reset(&mut self) {
        self.clear();
        self.cursor().goto(0, 0);
    }

    pub fn cursor(&mut self) -> Cursor {
        Cursor { editor: self }
    }

    pub fn print<T: Display>(&mut self, item: T) {
        write!(self.output, "{}", item).unwrap();
        self.output.flush().unwrap();
    }

    fn clear(&mut self) {
        self.print(clear::All);
    }
}
