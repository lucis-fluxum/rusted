use std::fmt::Display;
use std::io::{stdout, Stdout, Write};
use termion::clear;
use termion::color;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    // TODO: Visual mode, selectable regions, copy/paste, etc
}

const BG_COLOR_RGB: color::Rgb = color::Rgb(45, 20, 0);
const FG_COLOR_RGB: color::Rgb = color::Rgb(215, 160, 120);

// TODO: Impl Debug, please
// TODO: Don't use AlternateScreen until you get error logging figured out
pub struct Editor {
    pub mode: Mode,
    // pub output: AlternateScreen<RawTerminal<Stdout>>,
    pub output: RawTerminal<Stdout>,
    pub buffer: Vec<String>,
    pub filename: String,
    pub x: usize,
    pub y: usize,
    pub command: String,
    pub saved_positions: Vec<(usize, usize)>,
}

// TODO: Initialization with background/foreground colors
// TODO: Status messages and prompts at bottom
impl Editor {
    pub fn new(filename: String) -> Editor {
        let mut e = Editor {
            mode: Mode::Normal,
            // output: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
            output: stdout().into_raw_mode().unwrap(),
            buffer: vec![],
            filename,
            x: 0,
            y: 0,
            command: String::new(),
            saved_positions: vec![],
        };
        e.init();
        e
    }

    fn init(&mut self) {
        self.reset();
        self.load();
        self.reset_status();
    }

    pub fn reset(&mut self) {
        self.buffer = vec![String::new()];
        self.set_mode(Mode::Normal);
        self.print(clear::All);
        self.goto(0, 0);
    }

    pub fn reset_status(&mut self) {
        // Don't use save_pos here since it messes up the status
        let (prev_x, prev_y) = self.pos();
        let pos = (0, self.size().1 - 2);
        let status = self.status();
        self.goto(pos.0, pos.1);
        self.print(clear::CurrentLine);
        self.print(status);
        self.goto(prev_x, prev_y);
    }

    pub fn print<T: Display>(&mut self, item: T) {
        write!(
            self.output,
            "{}{}{}",
            color::Bg(BG_COLOR_RGB),
            color::Fg(FG_COLOR_RGB),
            item
        )
        .unwrap();
        self.output.flush().unwrap();
    }

    pub fn print_notice<T: Display>(&mut self, item: T) {
        self.save_pos();
        let pos = (0, self.size().1 - 1);
        self.goto(pos.0, pos.1);
        self.print(clear::CurrentLine);
        self.print(item);
        self.restore_pos();
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

    // TODO: Extract return values into a single DebugInfo type
    pub fn debug_info(&self) -> (Vec<String>, (usize, usize), String, Vec<(usize, usize)>) {
        (
            self.buffer.clone(),
            self.size(),
            self.filename.clone(),
            self.saved_positions.clone(),
        )
    }

    fn status(&self) -> String {
        let mode = format!("{:?}", self.mode).to_uppercase();
        let (col, line) = match self.mode {
            Mode::Command => self.saved_positions[self.saved_positions.len() - 1],
            _ => (self.x, self.y), // TODO: NLL optimization
        };
        format!(
            "{} | {}% {}:{} | Cmd: {:?} | File: {:?}",
            mode,
            self.percentage_pos(line) as u8,
            line + 1,
            col + 1,
            self.command,
            self.filename,
        )
    }

    fn percentage_pos(&self, y: usize) -> f32 {
        let line = (y + 1) as f32;
        let height = self.buffer.len() as f32;
        line / height * 100.0
    }
}
