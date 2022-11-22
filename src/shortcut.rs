use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortcut {
    pub name: String,
    pub abbreviation: String,
    pub destination: String,
    pub frequency: i64
}

impl Shortcut {
    pub fn increment_frequency(&mut self) {
        self.frequency += 1
    }
}