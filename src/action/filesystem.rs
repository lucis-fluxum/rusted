use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use editor::Editor;

impl Editor {
    pub fn load(&mut self, filename: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());
        self.buffer = reader.lines().map(|line| line.unwrap()).collect();
        self.refresh_all();
    }

    pub fn save(&self) {}
}
