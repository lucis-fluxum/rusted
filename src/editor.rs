use std::io::{stdout, Stdout, Write};
use std::fmt::Display;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear;
use termion::terminal_size;

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
// TODO: Use AlternateScreen to preserve/restore terminal state
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
        self.reset_status();
    }

    pub fn reset(&mut self) {
        self.clear();
        self.goto(0, 0);
    }

    pub fn reset_status(&mut self) {
        let prev_pos = self.pos();
        let pos = (0, self.size().1 - 2);
        let status = self.status();
        self.goto(pos.0, pos.1);
        self.print(clear::CurrentLine);
        self.print(status);
        self.goto(prev_pos.0, prev_pos.1);
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

    pub fn size(&self) -> (usize, usize) {
        let (cols, rows) = terminal_size().unwrap();
        (cols as usize, rows as usize)
    }

    fn status(&self) -> String {
        let mode = format!("{:?}", self.mode).to_uppercase();
        format!(
            "{}\t{}:{}\tCmd: {:?}",
            mode,
            self.y + 1,
            self.x + 1,
            self.command
        )
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.push(String::new());
        self.print(clear::All);
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        print!(
            "\r\nbuffer: {:?}\r\nsize: {:?}\r\n",
            self.buffer,
            self.size()
        );
    }
}
