use std::io::{stdout, Stdout, Write};
use std::fmt::Display;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;

pub enum Mode {
    Normal,
    Insert,
    // TODO: Visual mode, selectable regions, copy/paste, etc
}

pub struct Editor {
    pub mode: Mode,
    pub output: RawTerminal<Stdout>,
    pub buffer: Vec<String>,
    pub x: usize,
    pub y: usize,
}

// TODO: Initialization with background/foreground colors
impl Editor {
    pub fn new() -> Editor {
        let mut e = Editor {
            mode: Mode::Normal,
            output: stdout().into_raw_mode().unwrap(),
            buffer: vec![String::new()],
            x: 0,
            y: 0,
        };
        e.reset();
        e
    }

    pub fn reset(&mut self) {
        self.clear();
        self.goto(0, 0);
    }

    pub fn print<T: Display>(&mut self, item: T) {
        write!(self.output, "{}", item).unwrap();
        self.output.flush().unwrap();
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.push(String::new());
        self.print(clear::All);
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("{:?}", self.buffer);
        // self.reset();
    }
}
