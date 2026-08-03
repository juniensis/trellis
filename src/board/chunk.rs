use std::collections::{HashMap, HashSet};

use crate::id::Id;

#[derive(Debug, Default)]
pub struct Chunk {
    x: i32,
    y: i32,
    visitors: HashSet<Id>,
}

impl Chunk {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visitors: HashSet::new(),
        }
    }
    pub fn visit(&mut self, id: Id) {
        self.visitors.insert(id);
    }
    pub fn leave(&mut self, id: &Id) {
        self.visitors.remove(id);
    }
}
