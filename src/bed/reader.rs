#![doc="A Bedfile Reader. Doesn't work yet."]
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;

pub struct BedReader {
    bed_file: String,
    lines: Vec<String>
}

impl Debug for BedReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BedReader @ {}", self.bed_file)
    }
}

impl BedReader {
    pub fn new(bed_file: Option<String>) -> BedReader {
        let bed_file = bed_file.unwrap_or("tests/data/sample.bed".to_string());
        let lines = read_to_string(&bed_file).unwrap().lines().map(String::from).collect();
        BedReader { bed_file, lines }
    }

    pub fn length(&self) -> usize {
        self.lines.len()
    }
}
