#![doc="A Bedfile Reader. Doesn't work yet."]
use std::fmt::{Debug, Formatter};

pub struct BedReader {
    bed_file: String,
}

impl Debug for BedReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BedReader @ {}", self.bed_file)
    }
}

pub fn build_bed_reader(bed_file: String) -> BedReader {
    BedReader {
        bed_file
    }
}