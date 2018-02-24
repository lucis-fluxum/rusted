use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;

use editor::Editor;

impl Editor {
    /// Load the contents of a file into the buffer, overwriting the current
    /// contents of the buffer.
    pub fn load(&mut self) {
        if !self.filename.is_empty() {
            // TODO: Persist the reader within an editor, only read when needed
            let reader = BufReader::new(File::open(&self.filename).unwrap());
            self.buffer = reader.lines().map(|line| line.unwrap()).collect();
            self.refresh_all();
        }
    }

    /// Save the contents of the buffer to a file, truncating the file if it
    /// exists.
    pub fn save(&mut self) {
        if !self.filename.is_empty() {
            let mut writer = LineWriter::new(File::create(&self.filename).unwrap());
            for line in &self.buffer {
                write!(writer, "{}\n", line).unwrap();
            }
            self.print_notice("Saved.");
        }
    }
}
