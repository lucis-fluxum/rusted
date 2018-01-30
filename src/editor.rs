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
    pub buffer: Vec<String>,
    pub x: usize,
    pub y: usize,
    // TODO: Add positional information (origin, width, height)
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
        self.cursor().goto(0, 0);
    }

    pub fn cursor(&mut self) -> Cursor {
        Cursor { editor: self }
    }

    pub fn print<T: Display>(&mut self, item: T) {
        write!(self.output, "{}", item).unwrap();
        self.output.flush().unwrap();
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.cursor().pos();
        let c = c.to_string();
        // TODO: Handle whitespace somewhere else
        match c.as_str() {
            "\n" => {
                self.cursor().goto(0, y + 1);
                self.buffer.push(String::new());
            }
            _ => {
                self.buffer[y].insert_str(x, &c);
                self.x += 1;
            }
        }
        self.refresh_line();
    }

    pub fn delete_char(&mut self) {
        let (x, y) = self.cursor().pos();
        self.buffer[y].remove(x);
        self.refresh_line();
    }

    pub fn backspace(&mut self) {
        let (x, y) = self.cursor().pos();
        if x > 0 {
            self.buffer[y].remove(x - 1);
            self.x -= 1;
        }
        self.refresh_line();
    }

    fn refresh_line(&mut self) {
        let (x, y) = self.cursor().pos();
        let line = self.buffer[y].clone();
        self.cursor().goto(0, y);
        self.print(clear::CurrentLine);
        self.print(line);
        self.cursor().right(x);
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
