#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub generation: u32,
    pub id: u32,
}

impl Id {
    pub fn new(generation: u32, id: u32) -> Self {
        Self { generation, id }
    }
}
