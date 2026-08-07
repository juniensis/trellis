use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub generation: u32,
    pub id: u32,
}

impl Id {
    pub fn new(generation: u32, id: u32) -> Self {
        Self { generation, id }
    }
    pub fn increment_generation(&mut self) {
        self.generation += 1;
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.generation, self.id)
    }
}
