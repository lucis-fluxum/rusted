use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;

use editor::Editor;

impl Editor {
    /// Load the content of a file into the buffer, overwriting the current
    /// contents of the buffer.
    pub fn load(&mut self, filename: &str) {
        // TODO: Persist the reader within an editor, only read when needed
        let reader = BufReader::new(File::open(filename).unwrap());
        self.buffer = reader.lines().map(|line| line.unwrap()).collect();
        self.refresh_all();
    }

    /// Save the contents of the buffer to a file, truncating the file if it
    /// exists.
    pub fn save(&self, filename: &str) {
        let mut writer = LineWriter::new(File::create(filename).unwrap());
        for line in self.buffer.iter() {
            write!(writer, "{}\n", line).unwrap();
        }
    }
}
