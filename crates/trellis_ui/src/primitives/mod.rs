pub mod bounds;
pub mod region;
pub mod scatter;

pub use bounds::Bounds;
pub use region::Region;
pub use scatter::Scatter;

#[derive(Debug, Clone)]
pub enum Primitive {
    Region(Region),
    Scatter(Scatter),
}

impl From<Region> for Primitive {
    fn from(value: Region) -> Self {
        Self::Region(value)
    }
}

impl From<Scatter> for Primitive {
    fn from(value: Scatter) -> Self {
        Self::Scatter(value)
    }
}
