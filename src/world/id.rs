use std::sync::Arc;
use trellis_core::utils::rand::psuedo_random_u64;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(u64);

impl Identifier {
    pub fn new() -> Self {
        Self(psuedo_random_u64())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementID(Identifier);

impl ElementID {
    pub fn new() -> Self {
        Self(Identifier::new())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemID(Identifier);
impl SystemID {
    pub fn new() -> Self {
        Self(Identifier::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_shape() {
        let ident = Identifier::new();
    }
}
