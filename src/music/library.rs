#![doc="A Music Library initialized from a folder location. Doesn't do squat yet."]
use std::fmt::{Debug, Formatter};

pub struct Library {
    root_folder: String,
}

impl Debug for Library {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Library @ {}", self.root_folder)
    }
}

pub fn build_library(root_folder: String) -> Library {
    Library {
        root_folder
    }
}