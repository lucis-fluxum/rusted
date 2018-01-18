use mode::Mode;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub struct Editor {
    pub mode: Mode,
    pub output: RawTerminal<Stdout>,
}
