use std::io::{stdout, Stdout, Write};
use std::fmt::Display;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    // TODO: Visual mode, selectable regions, copy/paste, etc
}

// TODO: Impl Debug, please
pub struct Editor {
    pub mode: Mode,
    pub output: RawTerminal<Stdout>,
    pub buffer: Vec<String>,
    pub x: usize,
    pub y: usize,
    pub command: String,
}

// TODO: Initialization with background/foreground colors
// TODO: Preserve terminal state before launch, restore on exit
// TODO: Status messages and prompts at bottom
impl Editor {
    pub fn new() -> Editor {
        let mut e = Editor {
            mode: Mode::Normal,
            output: stdout().into_raw_mode().unwrap(),
            buffer: vec![String::new()],
            x: 0,
            y: 0,
            command: String::new(),
        };
        e.init();
        e
    }

    fn init(&mut self) {
        self.reset();
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
        match self.mode {
            Mode::Command => self.setup_command_line(),
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.push(String::new());
        self.print(clear::All);
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("\rbuffer: {:?}", self.buffer);
        println!("\rmode: {:?}", self.mode);
        println!("\rcommand: {:?}", self.command);
        // self.reset();
    }
}
