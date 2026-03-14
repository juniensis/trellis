use std::ops::{Range, RangeInclusive};

use trellis_core::primitives::Pos;

#[derive(Debug)]
pub struct Bounds {
    x: RangeInclusive<i16>,
    y: RangeInclusive<i16>,
}

impl Bounds {
    pub fn new(x: RangeInclusive<i16>, y: RangeInclusive<i16>) -> Self {
        Self { x, y }
    }
    pub fn contains<P: Into<Pos>>(&self, pos: P) -> bool {
        let p = pos.into();
        self.x.contains(&p.x) && self.y.contains(&p.y)
    }
}
